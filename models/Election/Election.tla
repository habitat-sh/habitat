----------------------------- MODULE Election -----------------------------
EXTENDS Naturals, FiniteSets, Sequences, TLC

(***************************************************************************)
(* The set of members involved in the election.                            *)
(*                                                                         *)
(* These should be numbers since we compare IDs to break ties (i.e., Bully *)
(* Algorithm)                                                              *)
(***************************************************************************)
CONSTANT Member
ASSUME \A m \in Member: m \in Nat

(***************************************************************************)
(* Election Statuses                                                       *)
(*                                                                         *)
(* As these are just simple identifiers, these should be plain model       *)
(* values.                                                                 *)
(***************************************************************************)
CONSTANTS Running, NoQuorum, Finished
ElectionStatus == { Running, NoQuorum, Finished }

CONSTANT Nil

----

(***************************************************************************)
(* A mapping of member ID to the election messages they are receiving from *)
(* other members.  This acts as the communication channel between all      *)
(* members.                                                                *)
(*                                                                         *)
(* Currently messages are modeled as a bag, directly allowing for the      *)
(* modeling of out-of-order receipt of messages, as well as duplicate      *)
(* messages.                                                               *)
(***************************************************************************)
VARIABLE messages
InitMessages ==
    messages = [ m \in {} |-> 0]

\* 72 distinct types (assuming 3 members, 3 election statuses, and 1 term)
ElectionMessageType == [ member_id: Member,
                         votes: SUBSET Member,
                         term: {0}, \* Nat
                         status: ElectionStatus ]

\* 648 distinct types (naively, and only assuming a single possible term), but we can probably reduce this.
\* e.g., we shouldn't ever be sending a message from 3 to 1 that is voting for 2, because 3 should have already claimed votes for itself.
MessageEnvelopeType == [ src: Member,
                         dest: Member,
                         payload: ElectionMessageType ]

MessageTypeInvariant ==
    \A msg \in DOMAIN messages: msg \in MessageEnvelopeType

VARIABLE elections
InitElections ==
    elections = [ id \in Member |-> Nil ]

ElectionsTypeInvariant ==
    \A m \in Member: elections[m] \in ElectionMessageType \union {Nil}

VARIABLE leaders
InitLeaders ==
    leaders = [ id \in Member |-> Nil ]

LeadersTypeInvariant ==
    \A m \in Member: leaders[m] \in Member \union {Nil}

VARIABLE rumorHeat
InitRumorHeat ==
    rumorHeat = [ id \in Member |-> [ rumor |-> Nil,
                                      targets |-> {} ]]

RumorHeatTypeInvariant ==
    \A m \in Member: rumorHeat[m] \in [ rumor: ElectionMessageType \union {Nil},
                                        targets: SUBSET Member ]

vars == << messages, elections, leaders, rumorHeat >>

TypeInvariant == /\ MessageTypeInvariant
                 /\ ElectionsTypeInvariant
                 /\ LeadersTypeInvariant
                 /\ RumorHeatTypeInvariant
----

\* Network Messaging Functions
\* All these were basically stolen directly from the Raft TLA+ model

WithMessage(m, msgs) ==
    IF m \in DOMAIN msgs THEN
        [ msgs EXCEPT![m] = @ + 1 ]
    ELSE
         msgs @@ (m :> 1)

WithoutMessage(m, msgs) ==
    IF m \in DOMAIN msgs THEN
        [ msgs EXCEPT![m] = @ - 1 ]
    ELSE
        msgs

Send(m) ==
    messages' = WithMessage(m, messages)

Discard(m) ==
    messages' = WithoutMessage(m, messages)


----

(***************************************************************************)
(* Create a new election message, voting for member "i".                   *)
(***************************************************************************)
\* Roughly corresponds to Election::new() in butterfly/src/rumor/election.rs
NewElection(i, term) ==
    [ member_id |-> i,
      votes |-> {i},
      term |-> term,
      status |-> Running ]

NoElection(m) ==
    elections[m] = Nil

NoLeader(m) ==
    leaders[m] = Nil

HotRumor(i, e) ==
    rumorHeat' = [ rumorHeat EXCEPT![i] = [ rumor |-> e,
                                            targets |-> Member ]]

SaveElection(i, e, share) ==
    /\ elections' = [ elections EXCEPT![i] = e ]
    /\ IF share
       THEN HotRumor(i, e)
       ELSE UNCHANGED rumorHeat \* Our implementation is still sharing older versions, right?

StartElection(i) ==
    /\ NoElection(i)
    /\ SaveElection(i, NewElection(i, 0), TRUE)
    /\ UNCHANGED << messages, leaders >>

(***************************************************************************)
(* Given two election messages, take the votes from "src" and add them to  *)
(* those of "dest", returning the resulting election message.              *)
(***************************************************************************)
CombineVotes(dest, src) ==
    [dest EXCEPT!.votes = @ \union src.votes ]

\* current = our current election rumor
\* new     = the incoming rumor
MergeElections(current, new) ==
    CASE current = new
         -> [ msg |-> current, share |-> FALSE ]
      [] /\ new.term >= current.term
         /\ new.status = Finished
         -> [ msg |-> new, share |-> TRUE ]
      [] /\ new.term = current.term
         /\ new.status = Finished
         \* /\ current.term = Running \* I ADDED THIS TO THE SPECIFICATION
         \* Assumes that OUR term is also Finished... really, we should keep 'new' rather than 'current'
         \* THIS IS A PROBLEM... how does our algorithm actually get things right?
         -> [ msg |-> current, share |-> FALSE ] \* no further sharing... REALLY?
         \* -> [ msg |-> new, share |-> TRUE ] \* I CHANGED THIS IN THE SPEC
      [] current.term > new.term
         -> [ msg |-> current, share |-> TRUE ] \* Really?
\*    [] current.suitability > new.suitability
\*       -> [ msg |-> CombineVotes(current, new), share |-> TRUE ]
\*    [] new.suitability > current.suitability
\*       -> [ msg |-> CombineVotes(new, current), share |-> TRUE ]
      \* Bully Criterion: Highest ID wins (thought it was lowest, though? Not that it truly matters....)
      [] current.member_id >= new.member_id \* assumes ids are sortable
         -> [ msg |-> CombineVotes(current, new), share |-> TRUE ]  \* same as if our suitability is greater
      [] OTHER \* TODO: What specifically is this case?
         \* New rumor wins the tie-breaker
         -> [ msg |-> CombineVotes(new, current), share |-> TRUE ]  \* same as if their suitability wins

\* Create a rumor for ourselves AND finalize the incoming rumor, if necessary
\* Merging comes later
\*
\* Returns both rumors
InsertElection(m, msg) ==
    \* If this is for a service group we care about...
    \* If we already have an election....
    LET current == elections[m]
     IN
        IF current = Nil
        THEN
            [ ours   |-> NewElection(m, msg.term),
              theirs |-> msg ]
            \* If we *don't* already have an election
            \* Create a new one with the same term as what's coming in
        ELSE
        LET
            maybeFinished ==
            IF /\ msg.votes = Member \* everyone has voted
               /\ msg.member_id = m  \* and they voted for YOU
            THEN
               [ msg EXCEPT!.status = Finished ] \* Tell everyone that this election is done
            ELSE
                msg
        IN
            \* if it's for a previous term, remove it and create a new one
            [ ours   |-> IF msg.term > current.term
                         THEN NewElection(m, msg.term)
                         ELSE current,
              theirs |-> maybeFinished ]

        \* see if the incoming election is for you, and if the election is over
            \* if so, mark it as finished
            \* if there's no quorum, though, mark it as no-quorum


    \* If we *don't* already have an election
        \* Create a new one with the same term as what's coming in
            \* This then gets rumored unnecessarily :|
            \* and merged unnecessarily, too

\* Process a message from the network
Receive(m) ==
    LET me == m.dest
        prelim == InsertElection(me, m.payload)
        result == MergeElections(prelim.ours, prelim.theirs)
     IN /\ SaveElection(me, result.msg, result.share)
        /\ Discard(m)
        /\ UNCHANGED leaders

\* Share your state if you still have members you haven't shared it with yet.
SpreadElection(i, j) ==
    /\ i /= j
    /\ elections[i] /= Nil
    /\ j \in rumorHeat[i].targets
    /\ Send([src      |-> i,
             dest     |-> j,
             payload  |-> rumorHeat[i].rumor ])
    /\ rumorHeat' = [ rumorHeat EXCEPT![i].targets = @ \ {j} ]
    /\ UNCHANGED << elections, leaders >>

\* If your current state indicates that YOU have been voted the leader,
\* mark the election as finished and tell everybody.
EndElection(i) ==
    /\ elections[i] /= Nil
    /\ LET state == elections[i]
       IN /\ state.member_id = i
          /\ state.votes = Member
          /\ state.status = Running
          /\ LET finished == [ state EXCEPT!.status = Finished ]
              IN SaveElection(i, finished, TRUE)
          /\ leaders' = [ leaders EXCEPT![i] = i ]
          /\ UNCHANGED messages

\* If your current state indicates that the election is over,
\* then accept the winner as your leader.
AcceptLeader(i) ==
    /\ NoLeader(i)
    /\ elections[i] /= Nil
    /\ elections[i].status = Finished
    /\ leaders' = [ leaders EXCEPT![i] = elections[i].member_id ]
    /\ UNCHANGED << messages, elections, rumorHeat >>

----

----

\* Specification and Temporal Formulae

Init == /\ InitMessages
        /\ InitElections
        /\ InitLeaders
        /\ InitRumorHeat

Next == \/ \E i \in Member: StartElection(i)
        \/ \E i, j \in Member: SpreadElection(i, j)
        \/ \E i \in Member: EndElection(i)
        \/ \E i \in Member: AcceptLeader(i)
        \/ \E m \in DOMAIN messages: /\ messages[m] > 0 \* TODO: properly remove entry if we remove the last message
                                     /\ Receive(m)

Spec == Init /\ [][Next]_vars

----

\* Invariants and Properties

\* If an election is Finished, it should have votes for all members
\* (Eventually, this will need to be adjusted for quorum, probably)
FinishedElectionHearsFromAllMembers ==
\A m \in Member:
    \/ elections[m] = Nil
    \/ elections[m].status = Finished => elections[m].votes = Member

\* The member with the highest ID is assumed to be the leader.
\* NOTE: When quorum and suitability are added to the model, this will have to change!
PresumedLeader ==
    CHOOSE m \in Member : \A o \in Member : m >= o

ElectionFinishes ==
    \* Really, based on the Bully criterion, it's not just that there *exists* a
    \* member that is the leader, it's that that leader is the one with the largest ID,
    \* so this invariant can be strengthened.
    <> LET leader == PresumedLeader
        IN \A m \in Member:
                /\ leaders[m] = leader \* We have a leader, and everyone agrees on who it is
                /\ elections[m] /= Nil
                /\ elections[m].member_id = leader
                /\ elections[m].status = Finished \* It's over

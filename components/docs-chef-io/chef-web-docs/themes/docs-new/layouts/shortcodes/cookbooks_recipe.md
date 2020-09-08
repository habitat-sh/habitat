A recipe is the most fundamental configuration element within the
organization. A recipe:

-   Is authored using Ruby, which is a programming language designed to
    read and behave in a predictable manner
-   Is mostly a collection of [resources](/resources/), defined
    using patterns (resource names, attribute-value pairs, and actions);
    helper code is added around this using Ruby, when needed
-   Must define everything that is required to configure part of a
    system
-   Must be stored in a cookbook
-   May be included in another recipe
-   May use the results of a search query and read the contents of a
    data bag (including an encrypted data bag)
-   May have a dependency on one (or more) recipes
-   Must be added to a run-list before it can be used by Chef Infra
    Client
-   Is always executed in the same order as listed in a run-list
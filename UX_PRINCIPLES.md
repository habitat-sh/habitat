# Habitat CLI UX Principles

## Commands

### Make it Readable
- Keep commands human readable and memorizable by using a sentence like structure `hab <noun> <verb>`.
- The consistent noun/verb format becomes rhythmic and builds a mental model if used properly.
- Use terms (nouns and verbs) that fit with a real-world analogy.
- Be precise with meaning and pay attention to expectations when choosing words. For example, if you use a command like `hab pkg create` then the expectation should match the result (i.e. an actual package should be created, not simply a directory or scaffold).
- Don’t make up words. Consider using a different term or add a dash instead of jamming words together or using words that don’t exist in the dictionary. People won’t remember how to spell it which in turn generates unnecessary confusion. Clarity is king.
- Be careful with abbreviations - keep things speakable - imagine you are telling somebody else to run a command verbally, can they understand/interpret what you say without misspelling it? (this probably means it’s also easier for you to recall on your own as well).
- Look out for flags that contain boolean wording. For example, try to avoid flags like `thing_disabled` or `thing_enabled`, or even worse, a boolean-ish flag that also takes a boolean value such as hab foo thing_disabled=false or hab foo thing_enabled=true.
  - Instead, take out boolean wording and just use the setting name like hab foo some_option=enabled or hab foo some_option=false.

### Be Judicious
- Be judicious with the number of available commands.
- Limit the number of nouns - keeps the cognitive load small; users can memorize the core commands (keep them writing plans, not fumbling and looking up commands).

### Build Consistency
- Using the noun before the verb fosters consistency; you also get help at the noun level (imagine your command set as a tree hierarchy branching out from the root); if you can remember the nouns then you can quickly get the list of verbs for that noun (e.g. hab package -help).
- Use shortcuts sparingly - they’re certainly useful but obstruct understanding and meaning.
- Don’t treat flags as 2nd class citizens. Be aware of existing flags and consider how they can/will be used across multiple commands and subcommands.

### Kill Your Darlings
- If something feels confusing (even if it’s just single term), then get consensus and change it.
- Don’t ignore and postpone obvious shortcomings. Others see it too, as will users, so take the time to fix it.
- “Fixing it” doesn’t have to change the underlying code. If possible, simply change the part exposed to the user (e.g. it might be called ‘Projects’ in your codebase, but to users it’s all ‘Packages’).
- Don’t expose your ‘inside baseball’ to users. Re-word things for them and avoid jargon or overly complex technical terminology.
- As features develop and grow over time, the concepts and terminology may begin to break down and lead to confusion. Don’t be afraid to change a command later if it no longer makes sense.

### Share Context
- Map out the commands/subcommands/flags in a visual diagram.
- Walk the team through the diagram and post it in an easily accessible place.
- Refer to the map, as a team, when new c/s/f are going to be added (i.e. as new features are developed).

### Be a Good Host
- Provide guidepost commands that help people get started quickly and avoid common pitfalls.
  - For example, use a ‘getting started’ command to streamline rote one-time setup and decrease the time it takes for a user to get to the ‘good stuff’ (i.e. shorten the time to delight).
- If you find something unnecessarily (or annoyingly) complex as you use the product yourself, then it’s probably generating the same negative response for others. Consider how to simplify the flow and remove the potholes.
- Familiarity, simplicity and readability make it memorable. Docs are useful, but we don’t want people having to continually refer back to them.

### Explain Yourself
- Comment your code, be transparent, and use these as notes when building your help docs. If you have a public repo, then you can direct users there later to better understand the system and contribute (in addition to docs).

## Output

### Offer Timely Help
- Provide targeted/contextual help - make assumptions based upon their current setup (have they completed the install step?).
- Use flags to scope the help docs (normal/frequent, all, install, emergency).
- In times of emergency, provide a clear path to a human at Chef support (if possible) - remind of Chef support email or phone number within help docs.

### Embrace Design Constraints
- Use characters (icons), progress bars, and color in your output.
- Provide an option to remove characters, progress bars and color (can break automated build tasks, for example, that humans aren’t reading/watching anyway).

### Generate Humane Output
- Make output human readable and concise by default. Offer more/less verbose output as an option via a flag.
- Examine the current state of the system and make output contextual.
- Identify for analogs and antilogs. Do repeat CLI experiences that you’ve enjoyed elsewhere; Don’t repeat mistakes others have made. (e.g. In Habitat, building a package follows a format like bundle install).
- Write output with a clear message that a) sets the stage with “what we’re about to do”, b) shows progress on “what is happening now”, and c) wrap-up with “did it fail or succeed?”. If it fails, then explain why and provide a recommended next step. If it succeeds, then consider celebrating with a playful message.

### Expose Your Personality
- It’s a machine, but it doesn’t have to feel like one. Remind your users that there are people behind this product and that they care about your experience.
- Don’t overuse this principle by being overly silly or ever-present (that will be annoying), but a couple of inside jokes or playfully-worded sentences (especially during moments of success) sprinkled throughout keeps things interesting and personable (e.g. when a plan builds successfully out last line of output is “I love it when a plan.sh comes together.”).

### Provide Feedback
- Provide a sense of how long something will take if it’s going to longer than a few seconds with progress bars and/or honest message like “this may take several minutes” and show that it is progressing (not stuck).

### Avoid Dead-ends
- Don’t leave people hanging in the event that something goes away. Provide clear error messages that describe the situation and always end with a way forward (e.g. link to docs, further error detail, a recommended way forward, etc.).

## Instrumentation

### Measure Effectiveness
- It’s the command line so you can’t get any analytics, right? Wrong. As with a web or mobile application, you can still implement tracking with common tools like Google Analytics. That said, you have some unique challenges to address:
  - Treat the terminal experience as a more private and sacred interface for your users. Unlike the app world where there is obvious reliance upon the internet and thus an implied notion of less privacy, the command line experience feels more confined to the walls of your machine.
  - We’ve seen examples of community backlash when analytics were opted in by default (see homebrew) and it was not pretty. Given this more private setting, be courteous and transparent to retain trust and get the data you need. Some tips on accomplishing include:
   - Set analytics as opt out by default unless you are absolutely certain (and have communicated thoroughly elsewhere) that you’re tracking things.
   - Be explicit on what/when/why/how you are sending data. Include this level of detail in your permission request, code comments, and docs. Be transparent; leave nothing to the imagination.
   - Find the right time to request permission. For example, provide some helpers for getting up and running (see ‘Be a Good Host’ above) and in return, after having helped them, ask for participation in your analytics program.
   - Do not send any sort of personally identifiable, private or sensitive data.
    - Outline every data point that you intend to capture and send.
    - Once it has been sent, leave no trace of that history.
   - Avoid creating delays or failures in the core experience. It goes without saying, but your users are here to accomplish a task and that is paramount to your data needs. Don’t degrade or break the UX to serve yourself and your analytical desires.
    - For example, piggyback other activities that require internet calls and note in the output of that command that you’re sending analytics data.
    - Batch data and send it in chunks.
   - Make opting in and out both simple and clear. User should be confident that when they opt out, all tracking remnants have been removed. Add a simple command (e.g. hab analytics off) along with manual instructions if they want to nuke the possibility on their own (e.g. delete the /analtyics/directory).
- Do track things that will help you improve the CLI UX.
  - Which commands are most used? 
  - Which would be good indicators of usage trends?
  - Any larger product goals that you’re tracking? Are there any commands that help you track progress towards them?
  - Are there any errors or areas of concern you want to monitor?

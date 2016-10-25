---
title: Debugging Plans
---

# Debugging Plans

While working on plans, you may wish to stop the build and inspect the environment at any point during a phase (e.g. `do_download`, `do_build`). Habitat provides an `attach` function for use in your plan that functions like a debugging breakpoint and provides an easy <acronym title="Read, Evaluation, Print Loop">REPL</acronym> at that point.

To use `attach`, insert it into your plan at the point where you would like to use it, e.g.

       do_build() {
         attach
         make
       }

Now, perform a [build](/docs/create-packages-build) -- we recommend using an interactive studio so you do not need to set up the environment from scratch for every build.

       $ hab studio enter
       $ build yourapp

The build system will proceed until the point where the `attach` function is invoked, and then drop you into a limited shell:

~~~
### Attaching to debugging session

From: /src/yourapp/plan.sh @ line 15 :

    5: pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
    6: pkg_source=http://download.yourapp.io/releases/${pkg_name}-${pkg_version}.tar.gz
    7: pkg_shasum=c2a791c4ea3bb7268795c45c6321fa5abcc24457178373e6a6e3be6372737f23
    8: pkg_bin_dirs=(bin)
    9: pkg_build_deps=(core/make core/gcc)
    10: pkg_deps=(core/glibc)
    11: pkg_expose=(3000)
    12:
    13: do_build() {
 => 14:   attach
    15:   make
    16: }

[1] yourapp(do_build)>
~~~

You can use basic Linux commands like `ls` in this environment. You can also use the `help` command the Habitat build system provides in this context to see what other functions can help you debug the plan.

~~~
[1] yourapp(do_build)> help
Help
  help          Show a list of command or information about a specific command.

Context
  whereami      Show the code surrounding the current context
                (add a number to increase the lines of context).

Environment
  vars          Prints all the environment variables that are currently in scope.

Navigating
  exit          Pop to the previous context.
  exit-program  End the /hab/pkgs/core/hab-plan-build/0.6.0/20160604180818/bin/hab-plan-build program.

Aliases
  @             Alias for `whereami`.
  quit          Alias for `exit`.
  quit-program  Alias for `exit-program`.
~~~

  Type `quit` when you are done with the debugger, and the remainder of the build will continue. If you wish to abort the build entirely, type `quit-program`.

### Outputting the run hook to STDOUT

  If you want to output your run hook to STDOUT you can add this underneath the
  top line.

~~~
#!/bin/bash -xe

exec 2>&1 # output to STDOUT
~~~

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/create-packages-binary-only">Create binary-only packages</a></li>
</ul>

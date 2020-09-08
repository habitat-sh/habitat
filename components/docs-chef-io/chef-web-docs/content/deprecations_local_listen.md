+++
title = "Deprecation: Local Mode Listen (CHEF-18)"
draft = false
robots = "noindex"


aliases = "/deprecations_local_listen.html"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/deprecations_local_listen.md)

When using <span class="title-ref">chef-client</span> Local Mode, there
are two ways to launch the internal Chef Zero server. Originally we
launched it as a normal network service on localhost and then connected
to it as per normal. Unfortunately this meant that any user or process
on the machine could also connect to the Zero server during the converge
and because Chef Zero has no authentication or authorization systems,
they could potentially alter data mid-converge. We later added a
"socketless" mode, which runs the Zero server completely internally and
never exposes it on a real socket.

## Remediation

If you need to re-enable socket mode for now, you can run <span
class="title-ref">chef-client --local-mode --listen</span> or set <span
class="title-ref">knife\[:listen\] = true</span> in your <span
class="title-ref">.chef/knife.rb</span> or <span
class="title-ref">.chef/config.rb</span>.

kitchen is the command-line tool for Test Kitchen, an integration
testing tool maintained by Chef Software. Test Kitchen runs tests
against any combination of platforms using any combination of test
suites. Each test, however, is done against a specific instance, which
is comprised of a single platform and a single set of testing criteria.
This allows each test to be run in isolation, ensuring that different
behaviors within the same codebase can be tested thoroughly before those
changes are committed to production.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

Any Test Kitchen subcommand that does not specify an instance will be
applied to all instances.



</div>

</div>
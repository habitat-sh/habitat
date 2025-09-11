# Habitat 2.0.x README

## Handlebars Crate Upgraded

The [handlebars crate](https://crates.io/crates/handlebars) was upgraded from an early version that habitat had pinned due to breaking changes after habitat was released for general use.  The code base was upgraded to the most recent version available at the time the work was done.  Also it has been, and will continue to be, updated as new releases of the crate become available.

The impact of this is that you may have to update your templates as described in the following sections.

### Object Access Syntax Removed

In Habitat versions prior to 2.0.56 both `object.[index]` and `object[index]` were valid syntax for object access.  After habitat 2.0.56 only the `object[index]` remains valid syntax.

The action required is that you will need to proactively or reactively change any usages of the now removed `object.[index]` syntax to the still viable `object[index]` syntax.  See [PR #6323](https://github.com/habitat-sh/habitat/issues/6323) [PR #9585](https://github.com/habitat-sh/habitat/pull/9585) for more information.

One way to identify files for review is `find . -type f | xargs grep --perl-regexp '\.\[.*\]' --files-with-matches` but this should be adapted as appropriate for use against your codebase.

### Trimming Whitespace Via `{{~` and `~}}` Now Works Correctly

The `{{~` and `~}}` syntax for whitespace trimming was effectively a noop for habitat versions prior to 2.0.56. After 2.0.56 usage of `{{~` and `~}}` will trim whitespace as expected which may result in errors.

The action required is that you need to review your usage of habitat templating where `{{~` and `~}}` has been and update as appropriate for the context in which the whitespace trimming operators are used as the effect. The reason the guidance here is to "update as appropriate for the context" is that effect of using `{{~` and `~}}` is very dependent on where and how the syntax is used.

For example, one issue that the habitat team encountered was with templated nginx.conf files. In the context of a templated nginx.conf where semicolons and braces terminate simple and block directives the effect was one of poor formatting as opposed to borken habitat package because the file that was produced was syntactically valid and parseable by nginx.

However, another example the habitat team encountered was in the context of generating a PostgreSQL pg_hba.conf file. There the use of `{{~` caused a line that should have been created on a line by itself to be concatentated with the previous line. In this instance the plan built as expected but contained an unparseable pg_hba.conf file that caused an error when attempting to run postgres.

One way to identify files for review is `find . -type f | xargs grep --perl-regexp '{{~|~}}' --files-with-matches` but this should be adapted as appropriate for use against your code base.

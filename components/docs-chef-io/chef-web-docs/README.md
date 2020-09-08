# chef-web-docs

This repo is the source of the Chef reference documentation located at
https://docs.chef.io/

## Documentation for Other Repositories

We use [Hugo modules](https://gohugo.io/hugo-modules/) to grab documentation from
other Chef repositories.

The Chef Workstation documentation is stored in the `chef/chef-workstation` repository
in the [`www`](https://github.com/chef/chef-workstation/tree/master/www) directory.

The Chef Effortless documentation is stored in the `chef/effortless` repository
in the [`effortless-chef-io`](https://github.com/chef/effortless/tree/master/effortless-chef-io) directory.

The Chef Desktop documentation is stored in the `chef/desktop-config` repository
in the `docs` directory. This is a private repository.

The Chef InSpec documentation is stored in the `inspec/inspec` repository
in the `www` directory.

The Chef Automate documentation is stored in the `chef/automate` repository
in the `components/docs-chef-io` directory.

### Preview Local Changes to Documentation from Other Repos from chef-web-docs

Follow these steps to preview changes to the chef-workstation documentation while
running Hugo from chef-web-docs.

1. Clone `chef/chef-web-docs` and `chef/other-repo-with-docs` into the same directory.

1. Modify the go.mod file in `chef-web-docs`.

   Add a replace directive to the `go.mod` file that points that repo to your
   local copy of the repo. For example, if you want to preview changes to chef-workstation,
   add `replace github.com/chef/chef-workstation/www => ../chef-workstation/www` below the
   `require` statement. The whole file should look something like this:

   ```
   module github.com/chef/chef-web-docs

   go 1.14

   require (
     github.com/chef/automate/components/docs-chef-io v0.0.0-<commit timestamp>-<commit SHA> // indirect
     github.com/chef/chef-workstation/www v0.0.0-<commit timestamp>-<commit SHA> // indirect
     github.com/chef/desktop-config/docs v0.0.0-<commit timestamp>-<commit SHA> // indirect
     github.com/chef/effortless/effortless-chef-io v0.0.0-<commit timestamp>-<commit SHA> // indirect
     github.com/inspec/inspec/www v0.0.0-<commit timestamp>-<commit SHA> // indirect
   )

   replace github.com/chef/chef-workstation/www => ../chef-workstation/www
   ```

1. Start the Hugo server from `chef-web-docs`:

   ```
   make serve_ignore_vendor
   ```

You can preview any changes made to the documentation in `chef-workstation` as
they would appear on https://docs.chef.io.

See the documentation about Hugo vendoring below to understand the `--ignoreVendor` option.

**Before you submit a PR**

- Delete or comment out the `replace` directive in the `chef-web-docs/go.mod` file.

### Update Hugo Modules

Hugo modules are pinned to a particular commit of the master branch in their repository.
If you look in the `go.mod` and `go.sum` files, you'll notice that each repository
specifies a git commit timestamp and SHA.

To update a particular repo, run:

```
hugo mod get github.com/chef/repo_to_update/subdirectory
hugo mod tidy
```

For example, to update the chef-workstation repository:

```
hugo mod get github.com/chef/chef-workstation/www
hugo mod tidy
```

This will update that repository to the most recent commit.

You can also update a module to a commit version number. For example:

```
hugo mod get github.com/chef/chef-workstation/www@20.6.62
hugo mod tidy
```

To update all Hugo modules at the same time, run:

```
hugo mod get -u
hugo mod tidy
```

The `hugo mod tidy` command removes references to commits in the
`go.mod` and `go.sum` files that are no longer relevant.

Sometimes the `go.sum` file gets a little out of control and `hugo mod tidy` won't
clean it up. Each repository listed in the `go.mod` file should have two lines
in the `go.sum` file. If it has more than that and `hugo mod tidy` doesn't remove them,
delete the `go.sum` file and rebuild it with `hugo mod get -u`.

See Hugo's [documentation](https://gohugo.io/hugo-modules/use-modules/#update-modules)
for additional information about updating Hugo Modules.

#### go.sum File

The go.sum file should reference only one commit for each repository that is added
as a module to chef-web-docs. Each module and commit in the go.sum file will take
two or three lines. For example, the `chef/chef-workstation` repository documentation will
look like this:

```
github.com/chef/chef-workstation/www v0.0.0-20200625161326-f43898a8e6c0 h1:MTVSgikrlIqceXki6uVwbf+iCVPwkpxsh1ERseRG31g=
github.com/chef/chef-workstation/www v0.0.0-20200625161326-f43898a8e6c0/go.mod h1:rktT78z3KaWu7A+wf1g6KmYszrwn6Y3o3IFlTg8OpQg=
```

If there are references to older commits, delete those lines.

The `hugo mod tidy` command should remove those lines, but sometimes it doesn't.

The commit SHA and timestamp in the go.sum file should match the SHA and timestamp
in the go.mod file.

#### What If Hugo Doesn't Want to Update a Module

Sometimes Hugo gets a bit difficult and won't update a module cleanly or will leave
references to older commits of a module in the go.sum file. In those cases, I give
you the nuclear option. Re-initialize the modules:

1. Delete the go.mod and go.sum files.
1. Re-initialize the Hugo modules, `hugo mod init github.com/chef/chef-web-docs`
   This will generate a new, blank go.mod file.
1. Update the references to the other GitHub repositories, `hugo mod get -u`.
1. The previous step will update all modules to the latest commit of their source
   repositories.
   If you don't want that, look at the git history of those files and manually edit the
   go.mod and go.sum files to keep the older commits for the modules that
   you don't want to update.
1. Run `hugo mod tidy`. This probably won't do anything on newly initialized go.mod
   and go.sum files, but it can't hurt either.
1. Vendor the modules in chef-web-docs, `hugo mod vendor`.

## Hugo Vendoring

[Vendoring](https://gohugo.io/commands/hugo_mod_vendor/) stores all of the module content
from other repositories in the `_vendor` directory at the commit specified by
the `go.mod` file. When Hugo builds the documentation, it will grab content from
the `_vendor` directory instead of the original repository OR a local copy of a
that repository. To see which commits the vendored files reference, see the
`_vendor/modules.txt` file.

To vendor the modules in chef-web-docs, run `hugo mod vendor`.

To update the vendored modules, first update the Hugo module(s) (see above), then
run `hugo mod vendor`.

To ignore the vendored files in a Hugo build, run `make serve_ignore_vendor`. This
is the same as `make serve` except it adds the `--ignoreVendor` flag. This will
build the documentation from the GitHub repositories or from a local copy of a repository
if the `go.mod` file specifies pulling content from a local repository. (see above)

## The fastest way to contribute

The fastest way to change the documentation is to edit a page on the
GitHub website using the GitHub UI.

To perform edits using the GitHub UI, click on the `[edit on GitHub]` link at
the top of the page that you want to edit. The link takes you to that topic's GitHub
page. In GitHub, click on the pencil icon and make your changes. You can preview
how they'll look right on the page ("Preview Changes" tab).

We also require contributors to include their [DCO signoff](https://github.com/chef/chef/blob/master/CONTRIBUTING.md#developer-certification-of-origin-dco)
in the comment section of every pull request, except for obvious fixes. You can
add your DCO signoff to the comments by including `Signed-off-by:`, followed by
your name and email address, like this:

`Signed-off-by: Julia Child <juliachild@chef.io>`

See our [blog post](https://blog.chef.io/introducing-developer-certificate-of-origin/)
for more information about the DCO and why we require it.

After you've added your DCO signoff, add a comment about your proposed change,
then click on the "Propose file change" button at the bottom of the page and
confirm your pull request. The CI system will do some checks and add a comment
to your PR with the results.

The Chef docs team can normally merge pull requests within seven days. We'll
fix build errors before we merge, so you don't have to
worry about passing all of the CI checks, but it might add an extra
few days. The important part is submitting your change.

## Local Development Environment

The Chef Documentation website is built using:
- [Hugo](https://gohugo.io/) 0.61 or higher
- [Node](https://www.nodejs.com) 10.0.0 or higher
- [NPM](https://www.npmjs.com/) 5.6.0 or higher
- [Go](https://golang.org/dl/) 1.12 or higher

To install Hugo, NPM, and Go on Windows and macOS:

- On macOS run: `brew install hugo node go`
- On Windows run: `choco install hugo nodejs golang`

To install Hugo on Ubuntu, run:

- `apt install -y build-essential`
- `snap install node --classic --channel=12`
- `snap install hugo --channel=extended`

To build the docs and preview locally:

- Run `make serve`
- go to http://localhost:1313

Note that this repository grabs content from other repositories using Hugo modules.
That content is stored in the `_vendor` directory. `make serve` will use the
content in the `_vendor` directory instead of from its source GitHub repository
OR from a local copy of a repository.

To build the docs from the source repositories:

- Run `make serve_ignore_vendor`

Some of our documentation is stored in a private repository so this option is only
available to people with access to that repository. See the documentation above
concerning Hugo modules and vendoring.

To clean your local development environment:

- Running `make clean` will delete the sass files, javascript, and fonts. These will
	be rebuilt the next time you run `make serve`.

- Running `make clean_all` will delete the node modules used to build this site
	in addition to the functions of `make clean` described above. Those node
	modules will be reinstalled the next time you run `make serve`.

Hugo uses [Goldmark](https://github.com/yuin/goldmark) which is a
superset of Markdown that includes GitHub styled tables, task lists, and
definition lists.

## Shortcodes

Shortcodes are simple snippets of code that can be used to modify a Markdown
page by adding content or changing the appearance of content in a page. See
Hugo's [shortcode documentation](https://gohugo.io/content-management/shortcodes/)
for general information about shortcodes.

We primarily use shortcodes in two ways:

- adding reusable text
- highlighting blocks of text in notes or warnings to warn users or
provide additional important information

### Adding reusable text

There are often cases where we want to maintain blocks of text that are identical
from one page to the next. In those cases, we add that text, formatted in Markdown,
to a shortcode file located in `chef-web-docs/themes/docs-new/layouts/shortcodes`.

To add that shortcode to a page in `chef-web-docs/content`, add the file name,
minus the .md suffix, wrapped in double curly braces and percent symbols to
the location in the Markdown page where you want that text included. For example,
if you want to add the text in `shortcode_file_name.md` to a page, add
`{{% shortcode_file_name %}}` to the text of that page and it will appear when
Hugo rebuilds the documentation.

**Shortcodes in lists**

Hugo doesn't handle shortcodes that are indented in a list item properly. It interprets
the text of the shortcode as a code block. More complicated shortcodes with
code blocks, notes, additional list items, or other formatting look pretty
bad. We've created a simple shortcode for handling shortcodes in lists or definition
lists called `readFile_shortcode`.

To include a shortcode in a list or definition list, just add its file name
to the `file` parameter of `readFile_shortcode`.

For example, if you wanted to add `shortcode_file_name.md` to a list:
``` md
1.  Here is some text introducing the shortcode, but it's not necessary.

    {{< readFile_shortcode file="shortcode_file_name.md" >}}
```

### Highlighting blocks of text

We also use shortcodes to highlight text in notes, warnings or danger notices.
These should be used sparingly especially danger notices or warnings. Wrap text
that you want in a note using opening and closing shortcode notation. For example,

```
{{< note >}}

Note text that gives the user additional important information.

{{< /note >}}
```

To add a warning or danger, replace the word `note` with `warning` or `danger` in the
example above.

**Notes in lists**

Hugo doesn't handle shortcodes that are indented in lists very well, that includes the Note,
Warning, and Danger shortcodes. It interprets the indented text that's inside
the Note as a code block when it should be interpreted as Markdown.

To resolve this problem, there's a `spaces` parameter that can be added to the Note,
Warning, and Danger shortcodes. The value of spaces should be set to the number
of spaces that the note is indented.

For example:
```
This is a list:

- List item.

    {{< note spaces=4 >}}

    Text that gives the user additional important information about that list item.

    {{< /note >}}
```

This parameter also works on Danger and Warning shortcodes.

## Release Notes

Release notes are added to release notes pages using Javascript and content from
https://omnitruck.chef.io and https://packages.chef.io.

Chef Automate release versions, release dates, and links to release note Markdown
files comes from https://packages.chef.io/releases/current/automate.json.

Release versions for all other Chef products come from
https://omnitruck.chef.io/stable/<PRODUCT>/versions/all.

Each release note page comes from a Markdown file from
https://packages.chef.io/release-notes/<PRODUCT>/<VERSION>.md

If a release note Markdown file is not returned from packages.chef.io, the release
note page will show the text "This release does not have any release notes."

### Adding Release Notes

To add release notes to a page, add `release_notes = "<PRODUCT>"` to the page
front matter. For example, `release_notes = "inspec"`.

This will overwrite all content on that page.

## Sending feedback

We love getting feedback. You can use:

- Email --- Send an email to docs@chef.io for documentation bugs,
  ideas, thoughts, and suggestions. This email address is not a
  support email address, however. If you need support, contact Chef
  support.
- Pull request --- Submit a PR to this repo using either of the two
  methods described above.
- GitHub issues --- Use the https://github.com/chef/chef/issues page
  for issues specific to Chef Infra itself. This is a good place for
  "important" documentation bugs that may need visibility among a
  larger group, especially in situations where a doc bug may also
  surface a product bug. You can also use
  [chef-web-docs issues](https://github.com/chef/chef-web-docs/issues),
  especially for docs feature requests and minor docs bugs.
- https://discourse.chef.io/ --- This is a great place to interact with Chef and others.

## Documentation snapshots

The previous scoped doc sets that were found off of https://docs.chef.io/release/ are no longer available in this repo. Instead,
those doc sets are located at https://docs-archive.chef.io/. The index page on the docs archive site provides links to them. The doc sets retain their unique
left nav and can be used to view content at a particular point in time for a given release. In the future, snapshots
will be added for major releases of products/projects or for products/projects/components that are no longer supported.

## Archive of pre-2016 commit history

The commit history of this repo before February 12, 2016 has been
archived to the [chef-web-docs-2016 repo](https://github.com/chef-boneyard/chef-web-docs-2016) to save space. No changes
to the archive repo will be merged; it's just for historical purposes.

## Questions

If you need tips for making contributions to our docs, check out the
[instructions](https://docs.chef.io/style_guide.html).

If you see an error, open an [issue](https://github.com/chef/chef-web-docs/issues)
or submit a pull request.

If you have a question, send an email to docs@chef.io.

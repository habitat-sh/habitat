# Automate Documentation

The Automate Documentation is deployed on https://docs.chef.io/automate/ using Hugo
modules.

## The Fastest Way to Contribute

There are two steps to updating the Chef Automate documentation:

1. Update the documentation in the `habitat-sh/habitat` repository.
1. Update the Chef Habitat repository module in `chef/chef-web-docs`.

### Update Content in `habitat-sh/habitat`

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

The Chef documentation team can normally merge pull requests within seven days.
We'll fix build errors before we merge, so you don't have to
worry about passing all the CI checks, but it might add an extra
few days. The important part is submitting your change.

### Update the Habitat Repository Module In `chef/chef-web-docs`

We use [Hugo modules](https://gohugo.io/hugo-modules/) and [vendoring](https://gohugo.io/hugo-modules/use-modules/#vendor-your-modules)
to build Chef's documentation from multiple repositories. The Hugo modules are
pinned to commits in each repository, and the documentation files from those
repositories are vendored in `chef/chef-web-docs`. When the documentation
is updated in a repository, those changes won't appear in https://docs.chef.io until the
Hugo modules are updated to the new commit and documentation files are updated
in the `_vendor` directory.

This can be useful if you want to update documentation in `habitat-sh/habitat`, but
don't want it to appear on https://docs.chef.io until a later date.

To update the Hugo module for documentation in `habitat-sh/habitat`:

1. Make sure your documentation changes are merged into master in `habitat-sh/habitat`.
1. Contact your friendly local Docs Team who will update the Habitat Hugo module for you.

Or, for the adventurous:

1. Make sure your documentation changes are merged into master in `habitat-sh/habitat`.
1. On a local clone of `chef/chef-web-docs` run:
   1. `hugo mod get github.com/habitat-sh/habitat/components/docs-chef-io`
   1. `hugo mod tidy`
   1. `hugo mod vendor`
1. Submit a pull request to `chef/chef-web-docs`.

This will updated the Habitat files that are vendored in `chef-web-docs/_vendor/github.com/habitat-sh/habitat/`,
and update the commits in the go.mod and go.sum files in chef-web-docs.

## Local Development Environment

We use [Hugo](https://gohugo.io/), [Go](https://golang.org/), [NPM](https://www.npmjs.com/),
[go-swagger](https://goswagger.io/install.html), and [jq](https://stedolan.github.io/jq/).
You will need Hugo 0.61 or higher installed and running to build and view our documentation properly.

To install Hugo, NPM, and Go on Windows and macOS:

- On macOS run: `brew tap go-swagger/go-swagger && brew install go-swagger hugo node go jq`
- On Windows run: `choco install hugo nodejs golang jq`
  - See the Go-Swagger [docs to install go-swagger](https://goswagger.io/install.html)

To install Hugo on Linux, run:

- `apt install -y build-essential`
- `sudo apt-get install jq`
- `snap install node --classic --channel=12`
- `snap install hugo --channel=extended`
- See the Go-Swagger [docs](https://goswagger.io/install.html) to install go-swagger

1. (Optional) [Install cspell](https://github.com/streetsidesoftware/cspell/tree/master/packages/cspell)

    To be able to run the optional `make spellcheck` task you'll need to install `cspell`:

    ```shell
    npm install -g cspell
    ```

## Hugo Theme

We use a git submodule to grab the Hugo theme from the `chef/chef-web-docs` repository.

## Preview Habitat Documentation

There are three ways to preview the documentation in `habitat-sh/habitat`:

- Submit a PR
- `make serve`
- Preview content from `chef/chef-web-docs`

### Submit a PR

When you submit a PR to `habitat-sh/habitat`, Netlify will build the documentation
and add a notification to the GitHub pull request page. You can review your
documentation changes as they would appear on docs.chef.io.

### make serve

`make serve` will only preview the documentation that exists in `habitat-sh/habitat`.
This also shows a preview page that includes page metadata which can be useful
for changing where a page exists in the left navigation menu.

To build the docs and preview locally:

- Run `make serve`
- go to http://localhost:1313

The landing page shows navigation menu metadata and the left navigation menu
shows the menu weight for each page. You can use this information to add,
remove, or reorganize Workstation documentation in the menu. None of this will
appear on the [Chef Documentation](https://docs.chef.io) site when the
workstation content is updated.

While the Hugo server is running, any changes you make to content
in the `docs/content` directory will be automatically compiled and updated in the
browser.

#### Clean Your Local Environment

To clean your local development environment:

##### `make clean`
Deletes the SASS files, Javascript, fonts in `themes/docs-new`, and the . These will be rebuilt the next time you run `make serve`.

##### `make clean_all`
Deletes the node modules used to build this site in addition to `make clean` described above. These will be reinstalled the next time you run `make serve`.

##### `make reset_chef_web_docs`
Deletes all changes to the chef-web-docs submodule. Changes to chef-web-docs must be made in the chef/chef-web-docs repo and cannot be made from any other repository. Use `make reset_chef_web_docs` to restore the submodule to its initial state.

### Preview Content from chef/chef-web-docs

You can run the Hugo server locally from `chef/chef-web-docs` and direct Hugo to
preview content from your local copy of `habitat-sh/habitat` instead of from the
GitHub repo. This allows you to live reload documentation in `habitat-sh/habitat` and
see how it would look in https://docs.chef.io.

See the README in `chef/chef-web-docs` for instructions.

Also, see the [Hugo documentation](https://gohugo.io/hugo-modules/use-modules/#make-and-test-changes-in-a-module)
for previewing local changes to a module.

## Manage the chef-web-docs Git Submodule

We build previews on Netlify by adding `chef/chef-web-docs` as a Git submodule
in `docs/chef-web-docs`.

To get the commit that the chef-web-docs submodule is set to, run:

```bash
git submodule status
```

To update the submodule, run:

```bash
git submodule foreach git pull origin master
```

This will update the submodule to the latest commit in `chef/chef-web-docs`.

If local changes have been made to the chef-web-docs submodule and you want to
delete them, run:

```bash
make reset_chef_web_docs
```

This will reset the content of the chef-web-docs submodule to the commit that it
is set to.

## Creating New Pages

Please keep all of the InSpec documentation in the `content/inspec` directory.
To add a new Markdown file, run the following command from the `www` directory:

```
hugo new content/automate/<filename>.md
```

This will create a draft page with enough front matter to get you going.

Hugo uses [Goldmark](https://github.com/yuin/goldmark) which is a
superset of Markdown that includes GitHub styled tables, task lists, and
definition lists.

See our [Style Guide](https://docs.chef.io/style_guide/) for more information
about formatting documentation using Markdown.

## Chef Habitat Page Menu

Adding pages to a menu or modifying a menu should be handled by the Docs Team.

If you add content, it will not automatically show up in the left navigation menu.
Build the site locally (`make serve`) and see the landing page (`http://localhost:1313`).
Any page followed by `Habitat Menu: False` has not been added to the left navigation menu.

Each page needs a page title, an identifier, and a parent.

**Title**
The title is the name of the page as it appears in the left navigation menu.

**Parent**
The parent is the path to that page in the left navigation menu. For example, the
`getting started` page is found by clicking on Chef Habitat so it's parent is
`automate`.

**Identifier**
Each menu identifier must be unique. We use the menu parent value, followed by
the file name, followed by the page title.

**Menu Weight**
The menu weight is optional. If it isn't included, Hugo assigns each page a weight of 0
and pages with the same weight are put in alphabetical order. Pages with a higher weight
are lower in the menu. Pages with a weight precede pages without a weight.

Below is an example of a page menu entry:

```
[menu]
  [menu.automate]
    title = "Page Menu Title"
    identifier = "automate/<file_name>.md Page Title"
    parent = "automate"
    weight = 10
```

## Habitat Menu Config

The framework for the Habitat menu is located in the `config.toml` file in
`chef/chef-web-docs` file. This defines the parent menu directories that each
page can be added to.

You can modify the menu to try out a different layout in the `config.toml` file in
`automate/components/docs-chef-io` and preview it locally, but that information
is not copied to the `config.toml` file in `chef/chef-web-docs` when the documentation
is build for docs.chef.io.

You can add links to the Habitat menu that navigate to other pages on
the [Chef Documentation](https://docs.chef.io) site or to an external site. See
the example below.

```
[[menu.automate]]
title = "Page Menu Title"
identifier = "automate/<filename> Page Title"
parent = "automate"
url = "relative or absolute URL"
weight = 10
```

See the [Hugo menu documentation](https://gohugo.io/content-management/menus/)
for additional information on formatting a menu item.

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
to a shortcode file located in `inspec/inspec/docs/layouts/shortcodes`.

Each shortcode in the Chef InSpec documentation must be prefixed with `in_`.
For example, `in_shortcode_name.md`.

To add that shortcode to a page in `inspec/inspec/docs/content`, add the file name,
minus the .md suffix, wrapped in double curly braces and percent symbols to
the location in the Markdown page where you want that text included. For example,
if you want to add the text in `dt_shortcode_file_name.md` to a page, add
`{{% in_shortcode_file_name %}}` to the text of that page and it will appear when
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
1.  Here is some text in a list item introducing the shortcode.

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

-   List item.

    {{< note spaces=4 >}}

    Text that gives the user additional important information about that list item.

    {{< /note >}}
```

This parameter also works on Danger and Warning shortcodes.

## Aliases

Add an alias to the page metadata to redirect users from a page to the page you are
editing. They are only needed if a page has been deleted and you want to redirect
users from the deleted page to a new or existing page.

## Resource Pages

The resource pages are located in `www/content/inspec/resources/`.

The InSpec resources index page is located in `www/content/inspec/resources/_index.md`
and can be found on https://docs.chef.io/inspec/resources/

The resource index page has a shortcode called `inspec_resources` that lists
resource pages by platform. To use the shortcode, add the shortcode and
specify the platform parameter: `{{< inspec_resources platform="<platform>" >}}`

A resource page must be located in `www/content/inspec/resources` and must have
the platform parameter set in its frontmatter. Add `platform = <platform>` to
the page frontmatter to add the platform parameter. For example, the
`aide_conf.md` resource frontmatter has `platform = "linux"` in its page
frontmatter.

## Data Content

Hugo allows us to nest our data directory structure as much as necessary. You can add as many folders as necessary under `components/docs-chef-io/data/automate`.

```output
.
├── data
│   ├── automate
│   │   ├── cli_chef_automate
|   │   │   ├── command_one.yml
|   │   │   └── command_two.yml
```

## What Happens Behind the Scenes

The [Chef Documentation](https://docs.chef.io) site uses [Hugo modules](https://gohugo.io/hugo-modules/)
to load content directly from `habitat-sh/habitat/components/docs-chef-io`. Every time
`habitat-sh/habitat` is promoted to stable, Expeditor submits a PR to chef-web-docs to
update the version of the `habitat-sh/habitat` repository that Hugo uses to build Chef
Automate documentation on the [Chef Documentation](https://docs.chef.io) site.
This is handled by the Expeditor subscriptions in the `chef/chef-web-docs` GitHub repository.

## Sending documentation feedback

We love getting feedback. You can use:

- Email --- Send an email to docs@chef.io for documentation bugs,
  ideas, thoughts, and suggestions. This email address is not a
  support email address, however. If you need support, contact Chef
  support.
- Pull request --- Submit a PR to this repo using either of the two
  methods described above.
- GitHub issues --- Use the https://github.com/habitat-sh/habitat/issues.
  This is a good place for "important" documentation bugs that may need
  visibility among a larger group, especially in situations where a doc bug
  may also surface a product bug. You can also use [chef-web-docs
  issues](https://github.com/chef/chef-web-docs/issues), especially for
  docs feature requests and minor documentation bugs.
- https://discourse.chef.io/ --- This is a great place to interact with Chef and others.

## Questions

If you need tips for making contributions to our documentation, check out the
[instructions](https://docs.chef.io/style_guide.html).

If you see an error, open an [issue](https://github.com/chef/chef-web-docs/issues)
or submit a pull request.

If you have a question about the documentation, send an email to docs@chef.io.

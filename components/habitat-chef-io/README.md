# Chef Workstation Documentation

This folder contains the source for the [Chef Workstation documentation](https://docs.chef.io/workstation/)
which is deployed on the [Chef Documenation](https://docs.chef.io) site using a Hugo module.

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

The Chef documentation team can normally merge pull requests within seven days.
We'll fix build errors before we merge, so you don't have to
worry about passing all the CI checks, but it might add an extra
few days. The important part is submitting your change.

## Local Development Environment

The Chef Documentation website is built using [Hugo](https://gohugo.io/) and
[NPM](https://www.npmjs.com/). You will need Hugo 0.61 or higher installed and
running to build and view our documentation properly.

To install Hugo:

- On macOS run: `brew install hugo`
- On Windows run: `choco install hugo`

NPM is distributed with Node.js. To install Node.js:

- On macOS run: `brew install node`
- On Windows, download and run the installer from the [nodejs.org](https://nodejs.org) website.

### Preview Workstation Documentation

To build the docs and preview locally:

- Run `make serve`
- go to http://localhost:1313

The landing page shows navigation menu metadata and the left navigation menu
shows the menu weight for each page. You can use this information to add,
remove, or reorganize Workstation documentation in the menu. None of this will
appear on the [Chef Documentation](https://docs.chef.io) site when the workstation
content is updated.

While the Hugo server is running, any changes you make to content
in the `www/content` directory will be automatically compiled and updated in the
browser.

### Clean Your Local Environment

To clean your local development environment:

- Running `make clean` will delete the sass files, javascript, and fonts. These will
	be rebuilt the next time you run `make serve`.

- Running `make clean_all` will delete the node modules used to build this site
	in addition to the functions of `make clean` described above. Those node
	modules will be reinstalled the next time you run `make serve`.

## Creating New Pages

Please keep all your documentation in the `content/workstation` directory.
To add a new Markdown file, run the following command from the `www` directory:

```
hugo new content/workstation/<filename>.md
```

This will create a draft page with enough front matter to get you going.

Hugo uses [Goldmark](https://github.com/yuin/goldmark) which is a
superset of Markdown that includes GitHub styled tables, task lists, and
definition lists.

See our [Style Guide](https://docs.chef.io/style_guide/) for more information
about formatting documentation using Markdown.

## Workstation Page Menu

If you add content, it will not automatically show up in the left navigation menu.
Build the site locally (`make serve`) and see the landing page (`http://localhost:1313`).
Any page followed by `Workstation Menu: False` has not been added to the left navigation menu.

Each page needs a page title, an identifier, and a parent.

**Title**
The title is the name of the page as it appears in the left navigation menu.

**Parent**
The parent is the path to that page in the left navigation menu. For example, the
`knife serve` page is found by clicking on Chef Workstation, Chef Workstation Tools,
and then Knife. So it's parent is `chef_workstation/chef_workstation_tools/knife`.

**Identifier**
Each menu identifier must be unique. We use the menu parent value, followed by
the file name, followed by the page title.

**Menu Weight**
The menu weight is optional. If it isn't included, Hugo assigns each page a weight of 0
and pages with the same weight are put in alphabetical order. Pages with a higher weight
are lower in the menu.

Below is an example of a page menu entry:

```
[menu]
  [menu.workstation]
    title = "Page Menu Title"
    identifier = "chef_workstation/<file_name>.md Page Title"
    parent = "chef_workstation"
    weight = 10
```

## Workstation Menu Config

The framework for the workstation menu is located in the `config_workstation_menu.toml`
file. This defines the parent menu directories that each page can be added to.

In addition, you can add links to the Workstation menu that navigate to other pages on
the [Chef Documentation](https://docs.chef.io) site or to an external site. See
the example below.

```
[[menu.workstation]]
title = "Page Menu Title"
identifier = "chef_workstation/file_name.md Page Title"
parent = "chef_workstation"
url = "relative or absolute URL"
weight = 10
```

See the [Hugo menu documentation](https://gohugo.io/content-management/menus/)
for additional information about formatting a menu item.

Contact the documentation team if you need help adding a page to the menu.

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
to a shortcode file located in `chef-workstation/www/layouts/shortcodes`.

Each shortcode in the Chef Workstation documentation must be prefixed with `ws_`.
For example, `ws_shortcode_name.md`.

To add that shortcode to a page in `chef-workstation/www/content`, add the file name,
minus the .md suffix, wrapped in double curly braces and percent symbols to
the location in the Markdown page where you want that text included. For example,
if you want to add the text in `ws_shortcode_file_name.md` to a page, add
`{{% ws_shortcode_file_name %}}` to the text of that page and it will appear when
Hugo rebuilds the documentation.

**Shortcodes in lists**

Hugo doesn't handle shortcodes that are indented in a list item properly. It interprets
the text of the shortcode as a code block. More complicated shortcodes with
code blocks, notes, additional list items, or other formatting look pretty
bad. We've created a simple shortcode for handling shortcodes in lists or definition
lists called `shortcode_indent`.

To include a shortcode in a list or definition list, just add its file name
to the `shortcode` parameter of `shortcode_indent` without the .md suffix.

For example, if you wanted to add `shortcode_file_name.md` to a list:
``` md
1.  Here is some text introducing the shortcode, but it's not necessary.

    {{< shortcode_indent shortcode="shortcode_file_name" >}}
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

## Structure

### High Level
```
.
├── Makefile    # contains helpers to quickly start up the development environment
├── README.md
├── www        # the hugo site directory used for local development
```

### Local Content
```
.
├── site
│   ├── content
│   │   ├── workstation                 # where to keep markdown file documentation
│   ├── data
│   │   ├── chef-workstation            # where to keep structured data files used for data templates
│   ├── layouts
|   │   ├── shortcodes
|   │   │   ├── ws_<shortcode_name>.md  # how to name your workstation-specific shortcodes
|   ├── static
|   |   ├── images
|   |   |   ├── chef-workstation        # where to keep any images you need to reference in your documentation
|   |   ├── css
```

### What is happening behind the scenes

The [Chef Documentation](https://docs.chef.io) site uses [Hugo modules](https://gohugo.io/hugo-modules/)
to load content directly from the `www` directory in the `chef/chef-workstation`
repository. Every time `chef/chef-workstation` is promoted to stable, Expeditor
instructs Hugo to update the version of the `chef/chef-workstation` repository
that Hugo uses to build Chef Workstation documentation on the [Chef Documentation](https://docs.chef.io)
site. This is handled by the Expeditor subscriptions in the `chef/chef-web-docs` GitHub repository.

## Sending documentation feedback

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

## Questions

If you need tips for making contributions to our documentation, check out the
[instructions](https://docs.chef.io/style_guide.html).

If you see an error, open an [issue](https://github.com/chef/chef-web-docs/issues)
or submit a pull request.

If you have a question about the documentation, send an email to docs@chef.io. 
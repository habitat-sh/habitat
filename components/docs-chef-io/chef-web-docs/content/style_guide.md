+++
title = "Documentation Style Guide"
draft = false

aliases = ["/style_guide.html"]

[menu]
  [menu.overview]
    title = "Docs Style Guide"
    identifier = "overview/community/style_guide.md Docs Style Guide"
    parent = "overview/community"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/style_guide.md)

The Chef reference documentation is written using Markdown and built with Hugo.

We recommend that you use the conventions described in this guide when
contributing to the Chef reference documentation.

The HTML version of the doc set can be found at
[docs.chef.io](https://docs.chef.io).

## Building
To build the docs, run the command:

``` bash
make serve
```

## TOML Front Matter
Each page starts with [TOML front matter](https://gohugo.io/content-management/front-matter/) which contains metadata about the page and places it properly in the left navigation menu. Below is the TOML front matter for this page which you can use as a reference. Contact the Docs Team if you have any questions about properly formatting TOML front matter.

```toml
+++
title = "Documentation Style Guide"
description = "DESCRIPTION"
draft = false

aliases = "/style_guide.html"

[menu]
  [menu.infra]
    title = "Docs Style Guide"
    identifier = "overview/community/style_guide.md Docs Style Guide"
    parent = "overview/community"
    weight = 40
+++
```

### title
The title of the page. This will appear at the top of the page.

### draft
Set draft to `true` if you don't want Hugo to build the page.

### aliases
Add an alias if you want Hugo to automatically redirect the user from another page to the page you are writing.

### menu title
The title of the page as it would appear in the left navigation menu.

### menu identifier
The identifier of the page that you are writing. Each identifier must be unique.

The convention we've adopted is to use the identifier of the page's parent, a forward slash, then the page file, a space, and then the page title.

For example, this page's parent identifier is `overview/community`, the page file is `style_guide.md` and the page title is `Docs Style Guide`, so the full page identifier is `overview/community/style_guide.md Docs Style Guide`

### menu parent
The menu identifier of the page's parent.

The convention we've adopted is to append the different menu levels together, separated by a forward slash, and starting with the highest level. For example, this page is nested under Overview and then Community, so the page's parent identifier is `overview/community`.

### menu weight
The rank that the page will appear in the menu, incremented by 10. Higher numbers are lower in the menu.

## Section Headings

The following sections describe the section heading pattern that Chef is
using for topic titles, H1s, H2s, H3s and H4s.

As a general rule, limit the number of heading levels to no more than
two within a topic. There can be exceptions, especially if the document
is very large, but remember that HTML TOC structures usually have width
limitations (on the display side) and the more structure within a TOC,
the harder it can be for users to figure out what's in it.

Unless the topics are about installing things or about API endpoints,
the headings should never wrap. Keep them to a single line.

The width of heading adornment must be at least equal to the length of
the text in the heading and the same width for headings is used
everywhere. Consistent width is preferred.

### H1

The H1 heading is reserved for the page title which is created by the Hugo
page template. The Markdown file text should not have any H1 headings.

### H2

Use two hash characters (##) before the heading name to indicate H2 headings:

    ## H2 Heading

    This is the body.

### H3

Use three hash characters (###) before the heading name to indicate H3 headings:

    ### H3 Heading

    This is the body.

### H4

Use four hash characters (####) before the heading name to indicate H4 headings:

    #### H4 Heading

    This is the paragraph.

### Other headings
If you need more than four heading levels, use bold emphasis and then
white space to make the heading text stand out and separate the heading from the content:

    **heading name goes here**         # bold emphasis
                                      # blank line
    content, as normally authored.

## Lists

The following sections describe conventions for lists and tables in Chef
docs.

### Bulleted Lists

Bulleted lists break up text blocks and draw attention to a group of
items:

    - text goes here
    - text goes here

        - subitem text
        - subitem text

    - text goes here
    - text goes here

Use the dash symbol (-) for bulleted lists, even though Hugo
supports other symbols. Indent nested list items by **four** spaces.

### Numbered Lists

Numbered lists are created like this:

    1. text goes here
    1. text goes here
    1. text goes here

        1. sublist text
        1. sublist text

    1. text goes here

Start each ordered list item with the number 1 (1.). Hugo will generate the correct sequence of numbers in an ordered list regardless of the numbers that you use. Only using "1." will save you from having to re-number items if you add or remove an item later.

### Definition Lists

Definition lists are used to show the options available to a command
line tool:

    `--name-only`
    : Show only the names of modified files.

    `--name-status`
    : Show only the names of files with a status of `Added`, `Deleted`, `Modified`, or `Type Changed`.

## Tables

Create tables in Markdown like this:

    Chef Software | Description
    --------|------
    Infra Client | Awesome
    Infra Server | Fun
    Habitat | Super cool

Use three or more hyphens (---) to separate each column's header from the content of the table. Separate columns with a vertical bar or pipe (|).

## Inline Markup
Adding emphasis within text strings can be done using **bold** and
`code strings`.

### Bold
Use two asterisks (\*) to mark a text string as **bold**:

    **text goes here**

### Code Strings
Sometimes the name of a method or database field needs to be used inline
in a paragraph. Use **one** backquote to mark certain strings as `code`
within a regular string of text:

    `code goes here`

## Links

To make a link in Markdown put the page title in square brackets followed by the link in parentheses. For example this:

`[Ruby Programming Language](https://www.ruby-lang.org/)`

will produce this:

[Ruby Programming Language](https://www.ruby-lang.org/)

External links requires an HTTP address.

## Code Blocks
Code blocks are used to show code samples, such as those for Ruby, JSON,
and command-line strings.

### Ruby
Use this approach to show code blocks that use Ruby:

    ```ruby
    default["apache"]["dir"]          = "/etc/apache2"
    default["apache"]["listen_ports"] = [ "80","443" ]
    ```

### Bash
Use this approach to show code blocks that use any type of shell
command, such as for Knife or the Chef Infra Client or for any other
command-line example that may be required:

    ```bash
    $ knife data bag create admins
    ```

### Javascript (and JSON)
Use this approach to show code blocks that use any type of JavaScript,
including any JSON code sample:

    ```javascript
    {
      "id": "charlie",
      "uid": 1005,
      "gid":"ops",
      "shell":"/bin/zsh",
      "comment":"Crazy Charlie"
    }
    ```

### Literal
Literals should be used sparingly, but sometimes there is a need for a
block of text that doesn't work in a fenced code block, such as showing a directory structure, basic syntax, or pseudocode. To make a literal code block, indent the text by **four** spaces:

```

    a block of literal text indented three spaces
    with more
    text as required to
    complete the block of text.
    end.

```

## Repeating Text Blocks
Chef docs uses [shortcodes](https://gohugo.io/content-management/shortcodes/) to maintain text that appears in more than one location and must be consistent in every location.

### Writing a shortcode
All shortcode files are written in **Markdown** and stored in the `layouts/shortcodes` directory in the `chef-web-docs` repo.

### Adding a Shortcode to a Page

To include a shortcode in a Markdown file, wrap the name of the shortcode file, without the file type suffix, in double curly braces and percent characters. For example, if you wanted to add the `chef.md` shortcode to a page, add the following text to the Markdown page:

```
{{%/* chef */%}}
```

## Notes and Warnings

In general, notes and warnings are not the best way to present important
information. Before using them ask yourself how important the
information is. If you want the information to be returned in a search
result, then it is better for the information to have its own topic or
section heading. Notes and warnings have a different color than the surrounding text so they can be easily spotted within a doc. If notes and warnings must be used, the approach
for using them is as follows.

Notes and warnings are generated by bracketing the text of the note or warning in **info**, **warning** or **danger** shortcodes.

### Notes

```
{{</* note */>}}
This is a note.
{{</* /note */>}}
```

What a note looks like after it's built:

{{< note >}}

This is a note.

{{< /note >}}

### Warnings

Use sparingly so that when the user sees a warning it registers appropriately:

```
{{</* warning */>}}
This is a warning.
{{</* /warning */>}}
```

What a warning looks like after it's built:

{{< warning >}}
This is a warning.
{{< /warning >}}

### Danger

Danger should be used rarely and only when there are serious consequences for the user:

```
{{</* danger */>}}
This is a danger block.
{{</* /danger */>}}
```

This is what a danger block looks like after it's built:

{{< danger >}}
This is a danger block.
{{< /danger >}}


## Images

You have two options for formatting images in Hugo:

- Markdown syntax
- the `figure` shortcode

### Markdown syntax

To add an image in Markdown, use an exclamation point, square brackets around the alt text, parenthesis around the path to the image file starting in the `static` directory, and then optional hover text. For example:

```md
![Alt Text](/path/to/img.png "Optional Title")
```

### Figure Shortcode

The [figure shortcode](https://gohugo.io/content-management/shortcodes/#figure) allows you to specify the image size and include a title, a caption, and alternate text.

SVG images should be formatted using the `figure` shortcode.

``` md
{{</* figure src="/images/chef-logo.svg" title="Chef Logo" height="100" width="150" */>}}
```

Which looks like this:

{{< figure src="/images/chef-logo.svg" title="Chef Logo" height="100" width="150">}}

Raster images should be 96 dpi and no larger than 600 pixels wide.
This helps ensure that the image can be printed and/or built into other output
formats more easily; in some cases, separate 300 dpi files should be maintained
for images that require inclusion in formats designed for printing and/or presentations.

## Grammar

Chef does not follow a specific grammar convention. Be clear and
consistent as often as possible. Follow the established patterns in the
docs.

### Tautologies

A tautology, when used as a description for a component, setting,
method, etc. should be avoided. If a string is a tautology, some effort
should be made to make it not so. An example of a tautology is something
like "Create a new user" (by its very nature, a user created **is** a
new user) or (for a setting named `cidr_block`) "The CIDR block for the
VPC."

## Documentation Repo

The Chef reference documentation is located in: https://github.com/chef/chef-web-docs

-   The `chef-web-docs` repo contains a `content` directory
    which holds most the Markdown files in the doc set.
-   The `static/images` directory stores the image files used in the docs.
-   The `config.toml` tells Hugo how to build the navigation menus and contains other Hugo settings. Don't modify this file.

In the past, the `chef-web-docs` repo contained documentation for prior
versions of Chef components. Currently, the repo is limited to the
current major versions of Chef components.

### DCO Sign-off

Chef Software requires all contributors to include a [Developer Certificate of Origin](https://developercertificate.org/) (DCO) sign-off with their pull request as long as the pull request does not fall under the [Obvious Fix](#obvious-fix) rule. This attests that you have the right to submit the work that you are contributing in your pull request.

Our full DCO signoff policy is available here: https://github.com/chef/chef/blob/master/CONTRIBUTING.md#developer-certification-of-origin-dco

A proper DCO sign-off looks like this:
```
Signed-off-by: Julia Child <juliachild@chef.io>
```

You can add a DCO signoff to your pull request by adding it to the text of your commit message, or by using the `-s` or `--signoff` option when you make a commit.

If you forget to add a DCO sign-off before submitting a pull request, you can amend your commit by entering `git commit --amend -s`. After that you'll likely have to force push your commit to github by entering `git push -f`.

See this [blog post](https://blog.chef.io/2016/09/19/introducing-developer-certificate-of-origin/) to understand why Chef started using the DCO signoff.


### Obvious Fix

Small contributions, such as fixing spelling errors, where the content is small enough to not be considered intellectual property, can be submitted without signing the contribution for the DCO.

Changes that fall under our Obvious Fix policy include:

- Spelling / grammar fixes
- Typo correction, white space and formatting changes
- Comment clean up
- Bug fixes that change default return values or error codes stored in constants
- Adding logging messages or debugging output
- Changes to 'metadata' files like Gemfile, .gitignore, build scripts, etc.
- Moving source files from one directory or package to another

To invoke the Obvious Fix rule, simply add `Obvious Fix.` to your commit message.

See our Obvious Fix policy here: https://github.com/chef/chef/blob/master/CONTRIBUTING.md#chef-obvious-fix-policy


## Official Names

For Chef applications and components, use:

- Chef Automate
- Chef Habitat
- Chef Infra (formerly Chef)
- Chef Infra Client (Use Chef Client up to version 14.x)
- Chef Infra Server (Formerly Chef Server)
- Chef InSpec

## Deleting Pages or Making New Pages

If a new page is created or an old page is deleted, they must be added or removed from the `sitemap.md` page.

In addition, pages must be placed in the left navigation menu properly. This may involve moving other pages up or down in the left navigation menu by increasing or decreasing their menu weight which is specificed in TOML front matter of each page or possibly in the `config.toml` file.

Contact the documentation team if you have any questions about adding or removing pages.

## Contact

See our [feedback](/feedback/) page if you have any questions or comments for the documentation team.

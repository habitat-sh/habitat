# Habitat Documentation

The Habitat Documentation is deployed on https://docs.chef.io/habitat/ using Hugo
modules.

## The Fastest Way to Contribute

There are two steps to updating the Chef Habitat documentation:

1. Update the documentation in the `habitat-sh/habitat` repository.
1. Update the Chef Habitat repository module in `chef/chef-web-docs`.

### Update Habitat Documentation

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

We use [Hugo modules](https://gohugo.io/hugo-modules/) to build Chef's documentation
from multiple repositories. Expeditor will submit a pull request that updates the documentation
in `chef/chef-web-docs` the next time Habitat is promoted to stable.

The Docs Team can also update the Habitat documentation if changes need to be made
before the next promotion.

## Local Development Environment

We use [Hugo](https://gohugo.io/), [Go](https://golang.org/), [NPM](https://www.npmjs.com/),
[go-swagger](https://goswagger.io/install.html), and [jq](https://stedolan.github.io/jq/).
You will need Hugo 0.83.1 or higher installed and running to build and view our documentation properly.

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

## Preview Habitat Documentation

There are two ways to preview the documentation in `habitat-sh/habitat`:

- Submit a PR
- `make serve`

### Submit a PR

When you submit a PR to `habitat-sh/habitat`, Netlify will build the documentation
and add a notification to the GitHub pull request page. You can review your
documentation changes as they would appear on docs.chef.io.

### make serve

Running `make serve` will clone a copy of `chef/chef-web-docs` into `components/docs-chef-io`.
That copy will be configured to build the Habitat documentation from `components/docs-chef-io`
and live reload if any changes are made while the Hugo server is running.

- Run `make serve`
- go to http://localhost:1313

#### Clean Your Local Environment

If you have a local copy of chef-web-docs cloned into `components/docs-chef-io`,
running `make clean_all` will delete the SASS files, node modules, and fonts in
`components/docs-chef-io/chef-web-docs/themes/docs-new` used to
build the docs site in the cloned copy of chef-web-docs. Hugo will reinstall these
the next time you run `make serve`.

## Creating New Pages

Please keep all of the Habitat documentation in the `content/habitat` directory.
To add a new Markdown file, run the following command from the `components/docs-chef-io`
directory:

```
hugo new content/habitat/<filename>.md
```

This will create a draft page with enough front matter to get you going.

Hugo uses [Goldmark](https://github.com/yuin/goldmark) which is a
superset of Markdown that includes GitHub styled tables, task lists, and
definition lists.

See our [Style Guide](https://docs.chef.io/style_guide/) for more information
about formatting documentation using Markdown and Hugo.

## What Happens Behind the Scenes

The [Chef Documentation](https://docs.chef.io) site uses [Hugo modules](https://gohugo.io/hugo-modules/)
to pull content from `habitat-sh/habitat/components/docs-chef-io`. Every time
`habitat-sh/habitat` is promoted to stable, Expeditor submits a PR to chef-web-docs to
update the version of the `habitat-sh/habitat` repository that Hugo uses to build Chef
Habitat documentation on the [Chef Documentation](https://docs.chef.io) site.
This is handled by the Expeditor subscriptions in the `chef/chef-web-docs` repository.

There are two Habitat documentation pages that are handled differently, `habitat_cli.md`
and `service_templates.md`. Both of these pages are generated during the release pipeline
and then pushed up to https://packages.chef.io/files/stable/habitat/latest/generated-documentation.tar.gz.
Expeditor will copy those two pages from the tarball into the Habitat vendored content in `chef-web-docs`
each time it submits a pull request to update the Habitat documentation.

See:
- .expeditor/scripts/release_habitat/generate-cli-docs.js
- https://github.com/chef/chef-web-docs/blob/master/.expeditor/update_hugo_modules_project_promoted.sh
- https://github.com/habitat-sh/habitat/pull/7993

## Release Notes

Release notes allow product engineering to communicate the list of features that are shipping in the builds being promoted to `stable`. Remember release notes aren't changelogs! The audience is our end-users, not other engineers. If you need a quick primer on what goes into good release notes, take a look at these excellent articles:

- [The Good Docs Project](https://www.thegooddocsproject.dev/template/release-notes)
- [The Life-Changing Magic of Writing Release Notes](https://medium.com/@DigitalGov/the-life-changing-magic-of-writing-release-notes-4c460970565)
- [Let's All Appreciate These Great Release Notes Together](https://www.prodpad.com/blog/release-notes/)

Release notes are published from the [chef/chef-web-docs repository](https://github.com/chef/chef-web-docs/blob/main/content/release_notes/habitat.md).

## Documentation Feedback

We love getting feedback, questions, or comments.

**Email**

Send an email to Chef-Docs@progress.com for documentation bugs,
ideas, thoughts, and suggestions. This email address isn't a
support email address. If you need support, contact [Chef Support](https://www.chef.io/support/).

**GitHub issues**

Submit an issue to the [Habitat repo](https://github.com/habitat-sh/habitat/issues)
for "important" documentation bugs that may need visibility among a larger group,
especially in situations where a doc bug may also surface a product bug.

Submit an issue to [chef-web-docs](https://github.com/chef/chef-web-docs/issues) for
doc feature requests and minor documentation issues.


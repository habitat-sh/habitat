# Automate Documentation

The Automate Documentation is deployed on https://docs.chef.io/automate/ using Hugo
modules.

## The Fastest Way to Contribute

There are two steps to updating the Chef Automate documentation:

1. Update the documentation in the `chef/automate` repository.
1. Update the Chef Automate repository module in `chef/chef-web-docs`.

### Update Content in `chef/automate`

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

### Update the Automate Repository Module In `chef/chef-web-docs`

We use [Hugo modules](https://gohugo.io/hugo-modules/) and [vendoring](https://gohugo.io/hugo-modules/use-modules/#vendor-your-modules)
to build Chef's documentation from multiple repositories. The Hugo modules are
pinned to commits in each repository, and the documentation files from those
repositories are vendored in `chef/chef-web-docs`. When the documentation
is updated in a repository, those changes won't appear in https://docs.chef.io until the
Hugo modules are updated to the new commit and documentation files are updated
in the `_vendor` directory.

This can be useful if you want to update documentation in `chef/automate`, but
don't want it to appear on https://docs.chef.io until a later date.

To update the Hugo module for documentation in `chef/automate`:

1. Make sure your documentation changes are merged into master in `chef/automate`.
1. Contact your friendly local Docs Team who will update the Automate Hugo module for you.

Or, for the adventurous:

1. Make sure your documentation changes are merged into master in `chef/automate`.
1. On a local clone of `chef/chef-web-docs` run:
   1. `hugo mod get github.com/chef/automate/components/docs-chef-io`
   1. `hugo mod tidy`
   1. `hugo mod vendor`
1. Submit a pull request to `chef/chef-web-docs`.

This will updated the Automate files that are vendored in `chef-web-docs/_vendor/github.com/chef/automate/`,
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

## Preview Automate Documentation

There are three ways to preview the documentation in `chef/automate`:

- Submit a PR
- `make serve`
- Preview content from `chef/chef-web-docs`

### Submit a PR

When you submit a PR to `chef/automate`, Netlify will build the documentation
and add a notification to the GitHub pull request page. You can review your
documentation changes as they would appear on docs.chef.io.

### make serve

`make serve` will only preview the documentation that exists in `chef/automate`.
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
preview content from your local copy of `chef/automate` instead of from the
GitHub repo. This allows you to live reload documentation in `chef/automate` and
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

## Chef Automate Page Menu

Adding pages to a menu or modifying a menu should be handled by the Docs Team.

If you add content, it will not automatically show up in the left navigation menu.
Build the site locally (`make serve`) and see the landing page (`http://localhost:1313`).
Any page followed by `Automate Menu: False` has not been added to the left navigation menu.

Each page needs a page title, an identifier, and a parent.

**Title**
The title is the name of the page as it appears in the left navigation menu.

**Parent**
The parent is the path to that page in the left navigation menu. For example, the
`getting started` page is found by clicking on Chef Automate so it's parent is
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

## Automate Menu Config

The framework for the Automate menu is located in the `config.toml` file in
`chef/chef-web-docs` file. This defines the parent menu directories that each
page can be added to.

You can modify the menu to try out a different layout in the `config.toml` file in
`automate/components/docs-chef-io` and preview it locally, but that information
is not copied to the `config.toml` file in `chef/chef-web-docs` when the documentation
is build for docs.chef.io.

You can add links to the Automate menu that navigate to other pages on
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

### chef-automate CLI Page

There are two shortcodes that are used to generate the page content of the chef-automate CLI Page, `automate_cli_commands` and `automate_cli_status_errors`.
The shortcodes are in the "docs-new" theme on `chef/chef-web-docs` and both have a `data_path`
parameter which is the path to the data file or files that are used by the
shortcodes to generate that page's content.

In addition, the chef-automate CLI page has a custom layout that is used to generate
that page's table of contents. The name of layout is specified in the chef-automate CLI page's frontmatter
and the layout is in `layouts/partials`.

## API Documentation

To view the API documentation locally, navigate to the `components/docs-chef-io` directory, run the following command to start the development server:

```shell
make serve
```

To view the API docs in your browser, navigate to:

```shell
http://localhost:1313/automate/api
```

### Building API Docs

Chef Automate generates API documentation from proto files comments using

* [grpc-gateway protoc-gen-swagger](https://github.com/grpc-ecosystem/grpc-gateway/tree/master/protoc-gen-swagger) as the protoc compiler
* [OpenAPI 2.0 specification](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md) as the API schema
* [ReDoc](https://github.com/Redocly/redoc) as the front-end display

To build the API documentation during development:

1. Make any edits to the .proto files necessary. Note that an "Authorization Action:" section will be added automatically in the next step. See [this pull request](https://github.com/chef/automate/pull/2982) for details on how and what to document and things to watch out for.
2. Recompile the .proto files as described in the [Automate development guide](https://github.com/chef/automate/blob/master/dev-docs/DEV_ENVIRONMENT.md)
3. From the top of the `components/docs-chef-io` directory, sync the .swagger files generated with those in the docs component by running `make sync_swagger_files`
4. Run the hugo server and view the documentation in your browser
5. (Optional) From the top of the `components/docs-chef-io` directory, check for typos in documentation files by running `make spellcheck`

Rebuilding the API docs is necessary when adding documentation content to Automate's .proto files. Changing the code in the .proto files does not require rebuilding, but new or updated services, messages, or fields require documentation.
Our goal is to provide timely and accurate documentation for Chef Automate users.

You can build the API docs by running the following all-in-one command from the top-level `automate` directory:

`hab studio run "source .studiorc && compile_all_protobuf_components" && pushd components/docs-chef-io/ && make sync_swagger_files generate_swagger && make serve || popd`

Review the API docs in your browser:

`http://localhost:1313/docs/api`

#### Notes on building the docs

* If you end up with merge conflicts while writing API docs, don't attempt to resolve them in any generated files. It's better to resolve the conflicts in any human edited file and then regenerate the docs.
* The all-in-one build command above can fail somewhere in the middle and leave things in an inconsistent state.
It is safe to run this command multiple times in a row while fixing syntax errors in .proto files or similar, but since it changes directories, it can sometimes be necessary to change back to the root automate directory.

### API File Index

* **API Proto Files**: automate/components/automate-gateway/api/
* **Example API Proto File**: automate/components/automate-gateway/api/compliance/reporting/
* **Meta Swagger**: automate/components/docs-chef-io/data/automate/api-static/01-meta-swagger.json
* **Reporting Export**: automate/components/docs-chef-io/data/automate/api-static/01-reporting-export.swagger.json
* **Node Export**: automate/components/docs-chef-io/data/automate/api-static/05-node-export.swagger.json
* **Description**: automate/components/docs-chef-io/data/automate/api-static/02-meta-description.yaml
* **Tags**: automate/components/docs-chef-io/data/automate/api-static/03-tags.swagger.json
* **Compiled Swagger**: automate/components/docs-chef-io/static/automate/api-docs/all-apis.swagger.json

### Tags and Tag Groups

* Endpoints need to appear in the rendered documentation as human-readable groupings. We use group and endpoint tags to create an organized and meaningful presentation for the Chef Automate API.
* Endpoints must have a tag that is a member of a tag group in order to appear in the rendered documentation. Endpoints without tags that are members of a tag group will not show up.
* An endpoint's default tag is the parent service where it was defined.
* Every service must have a unique tag name. For services without unique names, follow the explicit tagging process. Without unique service tag names, all endpoints for all of these services appear together in the rendered documentation.

#### Automatic tagging

Set up automatic tagging in the `components/docs-chef-io/data/automate/api-static/03-tags.swagger.json` file.

* Add the service's name to the `tags` list in a `tagGroup` or create a new `tagGroup` with the service's name in the `tags` list.
* Make the service name human-friendly by adding an entry to the `tags` in with a custom `x-displayName`.

#### Explicit tagging

* Each endpoint needs a tag specified in its definition, by adding a block like this to the endpoint definition in the proto file:

    ```go
    option (grpc.gateway.protoc_gen_swagger.options.openapiv2_operation) = {
      tags: "My Custom Tag"
    };
    ```

* All of the tags for an individual service should be the same, unless multiple groupings are desired in which case multiple independent tags can be used.
* After this, the process described for automatic tagging can be used, substituting the custom tag defined in place of the service's name.

### Notes

* Proto files can use tabs and spaces. We have a defacto convention for using tabs since our protos are primarily used with Go code.
* Endpoints will not show up in the UI unless one or more of their tags are assigned to a tag group, but they will still appear in the Swagger file accessible through the docs in the browser.
* multiline fields are not allowed in proto files. This the main reason we settled on using the comment style of endpoint documentation.

#### API Style Guide

Add comments to protos file with standard Go block and inline comment syntax.

Block comments are usually placed before the start of a gRPC service definition. Block comments for API documentation are sometimes called "leading comments".

Inline are usually placed within the message formats. Inline comments for API documentation are sometimes called "following comments".

```go
/* This is a block comment.

Block comments can span multiple lines.

Block comments support Markdown */

// This is an inline comment.
```

Block comments support multiple lines:

```go
  /*
  Fetch a node

  Fetch a specific node by id.
  Does not support filtering, pagination or sorting.
  */
  rpc ListReportIds(Query) returns (ReportIds) {
```

Block comments supports common markdown elements, such as:

```go
  /*
  Code blocks support `highlighting`

  It also supports

  Using code blocks, if you need them.

  */
```

Block comments support markdown tables:

```go
  /*
  List reports

  Makes a list of reports. Adding a filter makes a list of all node reports that meet the filter criteria. Supports pagination, filtering, and sorting.

  | Sort parameter | Sort value |
  | --- | --- |
  | latest_report.controls.failed.critical | controls_sums.failed.critical |
  | latest_report.controls.failed.total | controls_sums.failed.total |
  | latest_report.end_time (default) | end_time |
  | latest_report.status | status |
  | node_name | node_name.lower |
  */
  rpc ListReports(Query) returns (Reports) {
```

#### Excluding an endpoint

You can exclude an endpoint from the swagger generation by importing this in your proto file:

```go
import "protoc-gen-swagger/options/annotations.proto";
```

And adding this annotation to the rpc in question:

```go
option (grpc.gateway.protoc_gen_swagger.options.openapiv2_operation) = {
  tags: "hidden";
};
```

### Service Documentation

Not in iteration #1.

### Resource Documentation

Given a GRPC endpoint defined like this in the proto files:

```go
>.rpc ListReports(Query) returns (Reports) {
>.>.option (google.api.http) = {
>.>.>.post: "/api/v0/compliance/reporting/reports"
>.>.>.body: "*"
>.>.};
>.};
```

* For the first iteration of the Chef Automate API docs, we need summaries and descriptions for all user-facing end points. This can be done two ways, in the comment style (preferred) or the attribute style.

* Comment style of endpoint documentation supports multiline fields. Attribute style does not.

* Descriptions support Github Flavored Markdown, which means that you can use lists, tables, code-blocks and more.

#### Comment Style Descriptions

```go
  /*
  List reports
  This is the description.
  The description can span multiple lines and can contain `Github Flavored Markdown`
  */
  rpc ListReports(Query) returns (Reports) {
    option (google.api.http) = {
      post: "/api/v0/compliance/reporting/reports"
      body: "*"
    };
    option (grpc.gateway.protoc_gen_swagger.options.openapiv2_operation) = {
      tags: "Reporting"
    };
  };
```

#### Attribute Style Descriptions

```go
rpc ListReports(Query) returns (Reports) {
    option (google.api.http) = {
      post: "/api/v0/compliance/reporting/reports"
      body: "*"
    };
    option (grpc.gateway.protoc_gen_swagger.options.openapiv2_operation) = {
      tags: "Reporting"
      summary: "List reports"
      description: "This is the description.\nThe description can span multiple lines and can contain Github Flavored Markdown."
    };
  };
```

Resource documentation provides RPC-level description. It is sometimes also called "method description" because it documents what you can do with a specific API HTTP Method.

Use block/leading comments for RPC-level/method documentation.

Compare the leading comments to the content of the API documentation for the `ReadNode(ID)` resource.

```go
  /*
  Fetch a node #(summary)

  Fetch a specific node by id. #(description)
  Does not support filtering, pagination or sorting.
  */
  rpc ReadNode(Id) returns (Node) {
```

The rendered documentation:

![View of rendered API docs](./static/automate/images/api-resource-definition.png)

Note that once you recompile the proto files as described earlier, the actual source comment above will be updated to include an authorization action, for any relevant methods.

```go
  /*
  Fetch a node #(summary)

  Fetch a specific node by id. #(description)
  Does not support filtering, pagination or sorting.

  Authorization Action:

  ```
  compliance:reportNodes:get
  ```

  */
  rpc ReadNode(Id) returns (Node) {
```

#### Message Level Descriptions

Document fields and parameters directly above the relevant code line, using `//` inline comments

### Message Documentation

Given messages defined like this in the proto files:

```go
message Dog {
  // The dog's name.
  string name = 1;
  // Intentionally blank.
  Food favorite_food = 2;
  // Types of food the dog is normally fed.
  repeated Food typical_foods = 3;
}
// A type of food eaten by a Dog
message Food {
  // The food's name.
  string name = 1;
  // Serving size in ounces.
  int32 serving_size =3;
}
```

* Use a period (`.`) at the end of a comment to set it as the `description` for a message field. Omit the period to set a comment as the `title` for a message field. Our convention is to set comments as descriptions in message fields by closing them with periods.
* Swagger relies on inheritance to document nested references. A parent message referenced in a child message inherits its description from the parent, not where it is referenced by the child (see the example of `favorite_food` above). The Chef API style uses `// Intentionally blank.` in the child message.
This convention makes the purpose of the comment clear, makes undocumented message code visible, and makes bad blank message comment lines stand out in the UI.

### Non-Proto File Sources for API Documentation

We use non-proto file sources to document API content in cases where proto files run up against the limitations of tool we use for the Swagger conversion and also in cases where expressing the content in proto files is cumbersome.
We support these cases by adding static Swagger JSON or YAML files that combine with the Swagger generated from the proto files.

#### When to use Non-Proto Files

* Endpoints that do not automatically generate Swagger files during proto compilation. An example of this is the ReportingService's Export endpoint, which derives its endpoint from outside of the proto files, and is excluded in the proto to Swagger conversion process. See: `components/docs-chef-io/data/automate/api-static/01-reporting-export.swagger.json`.
* Documentation data that extends the protobuf or Swagger specs, for example the ReDoc `X-tagGroup` that we use for tag groups and tag display names. See: `components/docs-chef-io/data/automate/api-static/03-tags.swagger.json`.
* Metadata that is specific to the API documentation. For example, the support page URL, the logo image, etc. See: `components/docs-chef-io/data/automate/api-static/00-meta.swagger.json`.
* Large sections of prose or formatted text. With a few exceptions, proto files do not support multiline strings, so it is simpler to write and edit long content outside of the proto files. See: `components/docs-chef-io/data/automate/api-static/02-meta-description.yaml`.

#### Creating Non-Proto File Documentation

* Create a new JSON or YAML file in the `components/docs-chef-io/data/automate/api-static/` directory and its contents will be added to the final Swagger JSON  file used to render the documentation.
* Files in this directory are sorted numerically so that the order in which things are applied is deterministic and predictable.
* The Swagger files are combined in an additive way. The static user-edited Swagger files are applied first, follwed by all of the Swagger files generated from compiling the proto files.
* The Chef API style prefers JSON files in nearly all cases, but uses YAML files to supply long text sections in the documentation.

## Grammar and Style

* Keep it short
* Use [active voice](https://www.grammarly.com/blog/active-vs-passive-voice/)
* Use [plain language](https://plainlanguage.gov/resources/checklists/checklist/)
* Use [present tense](https://www.grammarly.com/blog/simple-present/)
* Write [positively](https://www.plainlanguage.gov/guidelines/concise/use-positive-language/)

### Specific Tips

* Avoid "of the" for a [genitive case](https://www.learnenglish.de/grammar/casepossgen.html)
    * Example: "The node name" instead of "Name of the node". (A simple genitive)
    * Example: "The control's unique ID" instead of "The unique ID of the control".

Note: We have not yet settled on conventions for using the grammatically questionable "simple genitive" instead of the categorically correct posessive. Try finding what sounds best to your ears.

## External references

* [OpenAPI 2.0 (aka swagger) spec](https://github.com/OAI/OpenAPI-Specification/blob/3.0.0/versions/2.0.md)
* [An example proto file in the protoc-gen-swagger repo demonstrating how to use most features of the proto-to-swagger conversion](https://github.com/grpc-ecosystem/grpc-gateway/blob/master/examples/proto/examplepb/a_bit_of_everything.proto)
* [ReDoc extensions to the Swagger spec](https://github.com/Redocly/redoc/blob/master/docs/redoc-vendor-extensions.md)
* [An alternate Swagger spec](https://swagger.io/docs/specification/2-0/basic-structure/)
* [Documentation in a code comment of how protoc-gen-swagger converts comments in proto files to Swagger fields](https://github.com/grpc-ecosystem/grpc-gateway/blob/c3787b4d95d15f4c89484b44e108a6640c1943ca/protoc-gen-swagger/genswagger/template.go#L1294-L1305)

## What Is Happening Behind the Scenes

The [Chef Documentation](https://docs.chef.io) site uses [Hugo modules](https://gohugo.io/hugo-modules/)
to load content directly from `chef/automate/components/docs-chef-io`. Every time
`chef/automate` is promoted to stable, Expeditor submits a PR to chef-web-docs to
update the version of the `chef/automate` repository that Hugo uses to build Chef
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
- GitHub issues --- Use the https://github.com/chef/automate/issues.
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

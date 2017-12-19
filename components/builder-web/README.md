# Builder Web

This is the web application for Builder. It's a single-page application built with [Angular](https://angular.io/), [TypeScript](https://www.typescriptlang.org/), [Redux](http://redux.js.org/) and [Immutable.js](https://facebook.github.io/immutable-js/).

## Development Setup

### Configuration

Copy habitat.conf.sample.js to habitat.conf.js to enable runtime configuration in development.

The configuration file looks like this:

```js
habitatConfig({
    habitat_api_url: "http://localhost:9636",
    community_url: "https://www.habitat.sh/community",
    docs_url: "https://www.habitat.sh/docs",
    environment: "production",
    github_client_id: "0c2f738a7d0bd300de10",
    source_code_url: "https://github.com/habitat-sh/habitat",
    tutorials_url: "https://www.habitat.sh/tutorials",
    slack_url: "http://slack.habitat.sh/",
    youtube_url: "https://www.youtube.com/playlist?list=PL11cZfNdwNyOxlvI1Kq6ae8eVBl5S3IKk",
    events_url: "https://events.chef.io/events/categories/habitat/",
    roadmap_url: "https://ext.prodpad.com/ext/roadmap/d2938aed0d0ad1dd62669583e108357efd53b3a6",
    feature_requests_url: "https://portal.prodpad.com/24539",
    forums_url: "https://forums.habitat.sh/",
    version: "",
    www_url: "https://www.habitat.sh",
});
```
### Running the Builder API Service

While it's possible to run this application without a concurrently running Builder API service, you won't be able to perform the kinds of actions that rely on that API (like create an origin, list and browse packages, sign in and out, and so on). To stand up a Builder API service locally, see the [BUILDER_DEV](../../BUILDER_DEV.md) doc.

Also note that by default, the `bldr-run` task described in that document builds this app and starts a `web` process that serves it over port 3000. Since the dev-setup instructions below are also configured to use port 3000 (the OAuth app we use in development requires it), you could end up with a port conflict or a dev service running on a port other than 3000.

There are couple of things you can do to avoid this:

  * If you're running `bldr-run` in a container or VM, and you've mapped port 3000 from the guest onto your local machine, remove that mapping to allow the `web` process to continue running on port 3000 on the guest, and you to run on port 3000 locally without a conflict. This is a good option of you're a regular contributor to the UI. Some providers (e.g., VirtualBox) also allow you to change this mapping in the UI without requiring a restart.

  * Set up a custom GitHub OAuth application to run the dev service somewhere other than `localhost:3000`. See below for instructions on how to do that.

### Installing Node

We suggest using [NVM](https://github.com/creationix/nvm) (Node Version Manager) to install the version of Node specified in  [.nvmrc](.nvmrc). Follow [the instructions in the NVM docs](https://github.com/creationix/nvm#installation) to set that up.

Once NVM is installed (you can verify with `nvm --version`), `cd` into `components/builder-web` and run:

```
nvm install
```

When that completes, verify the installation:

```
node --version
```

... which should now match what's in `.nvmrc`.

### Running the `builder-web` server

To start the node web server on your local machine:

```
npm install
npm start
```

You should now be able to browse to the UI at `http://localhost:3000/#/pkgs`.

Note that browsing to `http://localhost:3000/` (i.e., at the root level) will activate the application's default route, which is configured to redirect signed-out users to the Habitat home page (http://habitat.sh), and various navigation links will operate similarly. If you plan on developing for both the Builder UI and the [Web site](../../www), consider changing some of your configuration entries to allow for easier navigation between the two:

```
...
community_url: "http://localhost:4567/community",
docs_url: "http://localhost:4567/docs",
home_url: "http://localhost:4567/",
tutorials_url: "http://localhost:4567/tutorials",
www_url: "http://localhost:4567",
...
```

See the [www README](../../www/README.md) for help setting it up.

## Setting up a Custom OAuth Application

By default, `builder-web` is configured to use a preconfigured dev github oauth application. This should suffice as long as you intend to use `http://localhost:3000` as the homepage. If you need to use an alternate host name or port, you will need to setup a separate oauth application and configure `builder-api` and `builder-sessionsrv` with its generated credentials.

To register a new oauth application, go to your github user account settings and navigate to `OAuth Applications` and then click on `Register a new application`.

It is important that the homepage is set to `http://<hostname>:<port>` and the Authorization callback URL is set to `http://<hostname>:<port>/#/sign-in`.

Set the `github.client_id` to the client ID assigned to the oauth application. If you are running the API services, add `config.toml` files for the `builder-sessionsrv` and `builder-api` services:

```
mkdir -p /hab/svc/builder-api
mkdir -p /hab/svc/builder-sessionsrv

cat <<-EOF > /hab/svc/builder-api/config.toml
[github]
client_id       = "<Client ID>"
client_secret   = "<Client Sescret>"
EOF

cat <<-EOF > /hab/svc/builder-sessionsrv/config.toml
[github]
client_id       = "<Client ID>"
client_secret   = "<Client Sescret>"
EOF
```

See the [Create Configuration Files](/BUILDER_DEV.md#create-configuration-files) section of the BUILDER_DEV doc for more information.

Once your GitHub OAuth application is created, and the Builder services have been configured with your client ID and secret, you can start the web application by exporting the `URL` environment variable with the value of the endpoint you want to run prior to running `npm start`:

```
export URL=http://123.45.7.8:5656
npm start
```

Remember to adjust your `habitat.conf.js` and oath application settings if you change the default endpoint.

## Tests

Run the unit tests with `npm test`. They also run in the background when running `npm start`.

Files ending with .test.ts and .spec.ts are unit tested. We use
[Karma](https://karma-runner.github.io/0.13/index.html) and [Jasmine](https://jasmine.github.io/).
See [app/util.test.ts](app/util.test.ts) for an example.

## Tasks

These are defined in [package.json](package.json) and can be run with `npm run
TASK_NAME`.

* `build`: Build the JS and CSS
* `build-css`
* `build-css-watch`: Build the CSS and watch for changes
* `build-js`
* `build-js-watch`
* `clean`: Remove files created by build tasks
* `clean-css`
* `clean-js`
* `lint`: Check TS and SCSS files for lint errors
* `lint-css`
* `lint-css-watch`
* `lint-js`
* `lint-js-watch`
* `repl`: Start a TypeScript REPL
* `start`: Watch for changes and start a development server running on port 3000
* `test`: Run the tests
* `test-watch`
* `test-unit`: Run the unit tests
* `test-unit-watch`
* `travis`: For running the build and tests on Travis CI

## Code Style Conventions

These are guidelines for how to structure and format code in the application.

* Four spaces for tabs.
* TypeScript is linted with [TSLint](http://palantir.github.io/tslint/) using
  additional rules from the [Angular Style Guide](https://angular.io/styleguide).
  The rules followed in this repository are in the [tslint.json](tslint.json) file.
  Check your code with `npm run lint-js`.
* SCSS is linted with [Sass Lint](https://github.com/sasstools/sass-lint). The
  rules followed in this repository are in the [.sass-lint.yml](.sass-lint.yml)
  file. Check your code with `npm run lint-css`.
* TypeScript files should be named the same name as their default export (or the
  main thing they are concerned with, if there is no default export), so if a
  file has `export default class AppComponent {}`, it should be named
  AppComponent.ts. If a module exports many things, it should given an
  appropriate name and use camelCase.
* Directories should be made for components and their associated files when
  there is more than one file that pertains to a component.
* Directories that end in -page/ and components that are SomethingPageComponent
  are "page components", meaning they represent something that functions as a
  page in the app. All of these should be used in the `RouteConfig` of the
  AppComponent.
* Directory names and SCSS file names should use snake-case.
* SCSS files should start with an underscore and use snake-case:
  \_my-thing.scss. (in Sass, files that start with underscore are partials and
  can be loaded into other files. [app/app.scss](app/app.scss) imports these
  files.)

## Tools

* [Visual Studio Code](https://code.visualstudio.com/) works very well with
  TypeScript. There's also a tslint extension.
* The [Redux Devtools Chrome extension](https://chrome.google.com/webstore/detail/redux-devtools/lmhkpmbekcpmknklioeibfkpmmfibljd?hl=en)
  will let you inspect the state and actions of the running app in Chrome.

## Production

To build the JavaScript and CSS files, run `npm run build`.

`npm run dist` will build these files and put them along with the index.html and
other needed files into the dist/ directory. A web server can serve the files in
the dist directory to run the app.

The app is deployed to production with the Builder API, with the configuration
in [/terraform](/terraform) and the Habitat plan in
[/components/builder-api/habitat](/components/builder-api/habitat).

## Additional Documentation

* [Why are there so many files and what do all of them mean?](doc/files.md)

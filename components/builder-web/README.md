# Habitat Web

This is the web application for the Habitat SaaS.

This is a single page app built using [Angular 2](https://angular.io/).

## Configuration

Copy habitat.conf.sample.js to habitat.conf.js to enable runtime configuration
in development.

The configuration file looks like:

```js
habitatConfig({
    habitat_api_url: "http://localhost:9636/v1",
    community_url: "https://www.habitat.sh/community",
    docs_url: "https://www.habitat.sh/docs",
    environment: "production",
    github_client_id: "0c2f738a7d0bd300de10",
    source_code_url: "https://github.com/habitat-sh/habitat",
    tutorials_url: "https://www.habitat.sh/tutorials",
    version: "",
    www_url: "https://www.habitat.sh",
});
```

## Installing Node

The stable LTS version of Node must be installed (specified in [.nvmrc](.nvmrc)). You can use nvm (Node Version Manager) to install the desired version:

```
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.1/install.sh | bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

nvm install $(cat .nvmrc)
```

## Github OAuth

By default, `builder-web` is configured to use a preconfigured dev github oauth application. This should suffice as long as you intend to use `http://localhost:3000` as the homepage. If you need to use an alternate host name or port, you will need to setup a separate oauth application and configure `builder-api` and `builder-sessionsrv` with its generated credentials.

To register a new oauth application, go to your github user account settings and navigate to `OAuth Applications` and then click on `Register a new application`.

It is important that the homepage is set to `http://<hostname>:<port>` and the Authorization callback URL is set to `http://<hostname>:<port>/#/sign-in`.

Set the `github_client_id` to the client ID assigned to the oauth application. If you are running the API services, add `config.toml` files for the `builder-sessionsrv` and `builder-api` services:

```
mkdir -p /hab/svc/builder-api
mkdir -p /hab/svc/builder-sessionsrv

cat <<-EOF > /hab/svc/builder-api/config.toml
[cfg.github]
client_id       = "<Client ID>"
client_secret   = "<Client Sescret>"
EOF

cat <<-EOF > /hab/svc/builder-sessionsrv/config.toml
[cfg.github]
client_id       = "<Client ID>"
client_secret   = "<Client Sescret>"
EOF
```

## Running the builder-api services

If you want a full depot and api services to run along with the website, you can start these services by running `make bldr-run`. Note that `make bldr-run` includes running the `builder-web` service so you can skip the `npm` commands below.

## Running `builder-web` server

To start the node web server:

```
npm install
npm start
```

The service can be reached at `http://localhost:3000` by default. This can be changed by exporting the `URL` environment variable with the value of the endpoint you want to run prior to running `npm start`:

```
export URL=http://123.45.7.8:5656
```

Remember to adjust your `habitat.conf.js` and oath application settings if you change the default endpoint.

### Tests

Run all the tests with `npm test`.

#### Unit Tests

Run the unit tests with `npm run test-unit`. They also run in the background
when running `npm start`.

Files ending with .test.ts are unit tested. We use
[Karma](https://karma-runner.github.io/0.13/index.html) and
[Jasmine](https://jasmine.github.io/).

See [app/util.test.ts](app/util.test.ts) for an example.

### Tasks

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

### Code Style Conventions

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

### Tools

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

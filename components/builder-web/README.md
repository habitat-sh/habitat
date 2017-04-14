# Habitat Web

This is the web application for the Habitat SaaS.

This is a single page app built using [Angular 2](https://angular.io/).

## Development

The stable LTS version of Node must be installed (specified in [.nvmrc](.nvmrc).

Run `npm install` to install dependencies.

To run a development web server, run `npm start`.

### Configuration

Copy habitat.conf.sample.js to habitat.conf.js to enable runtime configuration
in development.

The configuration file looks like:

```js
habitatConfig({
    habitat_api_url: "https://my-api-url:1234/v1",
    some_other-config_option: true,
});
```

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

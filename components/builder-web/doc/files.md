# Why are there so many files and what do all of them mean?

The list below shows important files in the app and how they are organized. This
is not a complete list, but shows examples of files that exist. Not all of these
are present when you first check out the app, but they may appear after running
some `npm` scripts (`npm install`, `npm start`, etc.) and using the app.

    app/                         # The app/ directory contains the code that makes up the app.
      explore-page/              #   The top level directories contain the app's components.
      package-page/              #   For example, the package-page/ directory has
        _package.scss            #     [Sass](http://sass-lang.com/) SCSS stylesheet,
        PackagePageComponent.ts  #     a component that represents a page,
        PackageListComponent.ts  #     and possibly more components that are used by that one.
      packages-page/             #   Directories that end in -page/ contain
      sign-in-page/              #   components that act like pages.
      side-nav/                  #   But other directories have components that do not.
      actions/                   #   [Actions](http://redux.js.org/docs/basics/Actions.html).
        index.ts                 #     The main action creators entry point.
        notifications.ts         #     Other action creators live here too.
      records/                   #   [Record](https://facebook.github.io/immutable-js/docs/#/Record) objects representing data types.
      reducers/                  #   [Reducers](http://rackt.org/redux/docs/basics/Reducers.html) for redux
      app.ts                     #   Each reducer is responsible for a key in the app state (defined in app/initialState.ts.)
      index.ts                   #   The root Reducer.
      notifications.ts           #
      app.scss                   #   Main entry point for the SCSS.
      AppComponent.ts            #   The top level component of the app.
      AppStore.ts                #   The [Store](http://redux.js.org/docs/basics/Store.html) for the app.
      boot.ts                    #   Main entry point for the TypeScript.
      config.ts                  #   Provides configuration values specified in habitat.conf.js
      depotApi.ts                #   API client for making requests to the depot.
      fakeApi.ts                 #   API client for making fake requests.
      initialState.ts            #   The initial state of the app.
      tests-entry.ts             #   Main entry point for the tests.
      util.test.ts               #   Files that end in .test.ts are tests.
      util.ts                    #   Utility functions.
    config/                      # Configuration files for Habitat package.
    default.toml                 # Default config file used in Habitat package.
    dist/                        # Where the app prepared for production distribution goes. `npm run dist` to create it. Ignored by git.
    doc/                         # Documentation.
      files.md                   #   This is the file you are reading now.
    fixtures/                    # Fixture data the app uses to make fake requests.
    hooks/                       # Hooks used in Habitat package.
    node_modules/                # Modules installed by [npm](https://www.npmjs.com/). Ignored by git.
    stylesheets/base/            # The SCSS files for the stylesheets.
    test/e2e/                    # End-to-end [Protractor](https://angular.github.io/protractor/#/) tests.
    typings/                     # Type definitions for [Typings](https://github.com/typings/typings). Ignored by git.
    vendor/typings/              # Type definitions that we maintain ourselves rather than using the ones Typings downloads.
    .gitignore                   # Tells git which files it should ignore.
    .nvmrc                       # Shows the version of [node](https://nodejs.org/en/) you should be using, possibly with [NVM](https://github.com/creationix/nvm).
    .sass-lint.yml               # [Configuration](https://github.com/brigade/scss-lint#configuration) for `scss-lint`.
    assets/                      # Static resources used by the app
      app.css                    #   The main CSS file loaded by the browser.
      app.css.map                #   The source map for that CSS, mapping to the SCSS versions.
      app.js                     #   The main JS file loaded by the browser.
      app.js.map                 #   The source map for that JS, mapping to the TypeScript versions.
      images/                    # Images.
    favicon.ico                  # The favicon.
    habitat.conf.js              # The Habitat configuration file.
    habitat.conf.sample.js       # A sample version of the Habitat configuration file.
    index.html                   # The main HTML document.
    karma.conf.js                # The [Karma](https://karma-runner.github.io/0.13/index.html) config file.
    npm-debug.log                # Logs from npm debug output. Ignored by git.
    npm-shrinkwrap.json          # [`npm-shrinkwrap`](https://docs.npmjs.com/cli/shrinkwrap) file for locking npm dependencies.
    package.json                 # A [package.json](https://docs.npmjs.com/files/package.json).
    plan.sh                      # Plan for creating Habitat package.
    protractor.conf.js           # Configuration for Protractor.
    README.md                    # The README.
    tsconfig.json                # [Configuration for TypeScript](https://github.com/Microsoft/TypeScript/wiki/tsconfig.json).
    tslint.json                  # Configuration for [TSLint](http://palantir.github.io/tslint/).
    typings.json                 # Configuration for Typings.
    webpack.config.js            # Configuration for [webpack](http://webpack.github.io/).

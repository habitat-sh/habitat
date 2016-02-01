# Why are there so many files and what do all of them mean?

The list below shows important files in the app and how they are organized. Not
all of these are present when you first check out the app, but they may appear
after running some `npm` scripts (`npm install`, `npm start`, etc.) and using the app.

    app/                         # The app/ directory contains the code that makes up the app.
      home-page/                      # The top level directories contain the app's components.
      package-page/              # For example, the package-page/ directory has
        _package.scss            #   [Sass](http://sass-lang.com/) SCSS stylesheet,
        PackagePageComponent.ts  #   a component that represents a page,
        PackageListComponent.ts  #   and possibly more components that are used by that one.
      packages-page/             # Directories that end in -page/ contain
      sign-in-page/              # components that act like pages.
      sign-up-form/              # But other directories have components that do not.
      actions.ts                 # [Actions](http://redux.js.org/docs/basics/Actions.html).
      app.scss                   # Main entry point for the SCSS.
      AppComponent.ts            # The top level component of the app.
      AppStore.ts                # The [Store](http://redux.js.org/docs/basics/Store.html) for the app.
      boot.ts                    # Main entry point for the TypeScript.
      query.ts                   # Object that lets you query for packages.
      rootReducer.ts             # The root [Reducer](http://rackt.org/redux/docs/basics/Reducers.html).
      util.ts                    # Utility functions.
    doc/                         # Documentation.
      files.md                   # This is the file you are reading now.
    dist/                        # Where the app prepared for production distribution goes. `npm run dist` to create it. Ignored by git.
    fixtures/                    # Fixture data the app uses to make fake requests.
    node_modules/                # Modules installed by [npm](https://www.npmjs.com/). Ignored by git.
    stylesheets/base/            # The SCSS files for the stylesheets.
    typings/                     # Type definitions for [TSD](http://definitelytyped.org/tsd/). Ignored by git.
    vendor/typings/              # Type definitions that we maintain ourselves rather than using the ones TSD downloads.
    .gitignore                   # Tells git which files it should ignore.
    .nvmrc                       # Shows the version of [node](https://nodejs.org/en/) you should be using, possibly with [NVM](https://github.com/creationix/nvm).
    .sass-lint.yml               # [Configuration](https://github.com/brigade/scss-lint#configuration) for `scss-lint`.
    app.css                      # The main CSS file loaded by the browser.
    app.css.map                  # The source map for that CSS, mapping to the SCSS versions.
    app.js                       # The main JS file loaded by the browser.
    app.js.map                   # The source map for that JS, mapping to the TypeScript versions.
    favicon.ico                  # The favicon.
    index.html                   # The main HTML document.
    npm-debug.log                # Logs from npm debug output. Ignored by git.
    npm-shrinkwrap.json          # [`npm-shrinkwrap`](https://docs.npmjs.com/cli/shrinkwrap) file for locking npm dependencies.
    package.json                 # A [package.json](https://docs.npmjs.com/files/package.json).
    README.md                    # The README.
    tsconfig.json                # [Configuration for TypeScript](https://github.com/Microsoft/TypeScript/wiki/tsconfig.json).
    tsd.json                     # Configuration for TSD.
    tslint.json                  # Configuration for [TSLint](http://palantir.github.io/tslint/).
    webpack.config.js            # Configuration for [webpack](http://webpack.github.io/).

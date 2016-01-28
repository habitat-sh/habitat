# bldr Web

This is the web application for the bldr SaaS.

This is a single page app built using [Angular 2](https://angular.io/).

## Development

Node.js 4.2.4 must be installed.

Run `npm install` to install dependencies.

To run a development web server, run `npm start`.

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

## "Production"

To build the JavaScript and CSS files, run `npm run build`.

`npm run dist` will build these files and put them along with the index.html and
other needed files into the dist/ directory. A web server can serve the files in
the dist directory to run the app.

## Additional Documentation

* [Why are there so many files and what do all of them mean?](doc/files.md)

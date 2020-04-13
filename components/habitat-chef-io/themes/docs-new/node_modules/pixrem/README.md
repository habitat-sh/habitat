# Pixrem

[![Build Status](https://travis-ci.org/robwierzbowski/node-pixrem.png?branch=master)](https://travis-ci.org/robwierzbowski/node-pixrem)

[PostCSS](https://github.com/ai/postcss) plugin that generates pixel fallbacks for rem units.

## Installation

`npm install --save pixrem`

## Usage

Pixrem is a CSS post-processor that, given CSS and a root em value, returns CSS with pixel unit fallbacks or replacements. It's based on [browser data](http://caniuse.com/rem) so only needed fallbacks will be added. Basically, it's for IE8 or less, and for IE9 & IE10 in the `font` shorthand property and in pseudo-elements.

### Example

```js
'use strict';
var fs      = require('fs');
var pixrem  = require('pixrem');
var postcss = require('postcss');

var css = fs.readFileSync('main.css', 'utf8');
var processedCss = postcss([pixrem]).process(css).css;

fs.writeFile('main.with-fallbacks.css', processedCss, function (err) {
  if (err) {
    throw err;
  }
  console.log('IE8, you\'re welcome.');
});
```

Pixrem takes this:

```css
.sky {
  margin: 2.5rem 2px 3em 100%;
  color: blue;
}

@media screen and (min-width: 20rem) {
  .leaf {
    margin-bottom: 1.333rem;
    font-size: 1.5rem;
  }
}
```

And returns this:

```css
.sky {
  margin: 80px 2px 3em 100%;
  margin: 2.5rem 2px 3em 100%;
  color: blue;
}

@media screen and (min-width: 20rem) {
  .leaf {
    margin-bottom: 1.333rem;
    font-size: 1.5rem;
  }
}
```

### Options

Type: `Object | Null`
Default: `{rootValue: 16, replace: false, atrules: false, html: true, browsers: 'ie <= 8', unitPrecision: 3}`

- `rootValue`     the root element font size. Can be `px`, `rem`, `em`, `%`, or unitless pixel value. Pixrem also tries to get the root font-size from CSS (`html` or `:root`) and overrides this option. Use `html` option to disable this behaviour.
- `replace`       replaces rules containing `rem`s instead of adding fallbacks.
- `atrules`       generates fallback in at-rules too (media-queries)
- `html`          overrides root font-size from CSS `html {}` or `:root {}`
- `browsers`      sets browser's range you want to target, based on [browserslist](https://github.com/ai/browserslist)
- `unitPrecision` control the significant digits after the decimal point

## Contribute

Report bugs and feature proposals in the [Github issue tracker](https://github.com/robwierzbowski/node-pixrem/issues). Run tests with `npm test`. In lieu of a formal styleguide, take care to maintain the existing coding style.

## License

[MIT](http://en.wikipedia.org/wiki/MIT_License)


[![Bitdeli Badge](https://d2weczhvl823v0.cloudfront.net/robwierzbowski/node-pixrem/trend.png)](https://bitdeli.com/free "Bitdeli Badge")


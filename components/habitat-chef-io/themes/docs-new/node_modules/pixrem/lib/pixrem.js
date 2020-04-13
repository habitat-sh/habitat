var calc         = require('reduce-css-calc');
var vendor       = require('postcss/lib/vendor');
var postcss      = require('postcss');
var browserslist = require('browserslist');

var REGEX = /(\d*\.?\d+)rem/ig;
var BASE_FONT_SIZE = 16;
var PROPS = /^(background-size|border-image|border-radius|box-shadow|clip-path|column|grid|mask|object|perspective|scroll|shape|size|stroke|transform)/;
var VALUES = /(calc|gradient)\(/;

module.exports = postcss.plugin('pixrem', function (opts) {

  opts = opts || {};

  return function (css, result) {

    var options = {};
    options.rootValue     = (opts.rootValue     !== undefined) ? opts.rootValue     : BASE_FONT_SIZE;
    options.replace       = (opts.replace       !== undefined) ? opts.replace       : false;
    options.atrules       = (opts.atrules       !== undefined) ? opts.atrules       : false;
    options.html          = (opts.html          !== undefined) ? opts.html          : true;
    options.unitPrecision = (opts.unitPrecision !== undefined) ? opts.unitPrecision : 3;
    options.browsers      = (opts.browsers      !== undefined) ? opts.browsers      : 'ie <= 8';
    options.browsers      = browserslist(options.browsers);

    var isIElte8, isIEgte9, isIE9_10;
    if (detectBrowser(options.browsers, 'ie <= 8')) {
      isIElte8 = true;
    }
    if (detectBrowser(options.browsers, 'ie >= 9')) {
      isIEgte9 = true;
    }
    if (detectBrowser(options.browsers, 'ie 9, ie 10')) {
      isIE9_10 = true;
    }
    // no IE versions needed, skip
    if (!isIElte8 && !isIEgte9 && !isIE9_10) { return; }

    if (options.html) {
      // First, check root font-size
      css.walkRules(function (rule) {
        if (rule.parent && rule.parent.type === 'atrule') { return; }
        if (/^(html|:root)$/.test(rule.selectors)) {
          rule.walkDecls(function (decl) {
            if (decl.prop === 'font-size') {
              options.rootValue = decl.value;
            } else if (decl.prop === 'font' && decl.value.match(/\d/)) {
              options.rootValue = decl.value.match(/.*?([\d\.]*(em|px|rem|%|pt|pc))/)[1];
            }
          });
        }
      });
    }

    css.walkRules(function (rule) {

      // if options.at-rules is false AND it's not IE9-10: skip @rules
      if (!options.atrules && !isIE9_10) {
        if (rule.type === 'atrule' || (rule.parent && rule.parent.type === 'atrule')) { return; }
      }

      var isPseudoElement = (rule.selector.search(/:(after|before)/gi) !== -1);

      rule.each(function (decl, i) {

        if (decl.type !== 'decl') { return; }

        var value = decl.value;

        if (value.indexOf('rem') !== -1) {

          var prop = vendor.unprefixed(decl.prop);
          var isFontShorthand = (prop === 'font');
          var isSpecialCaseIE9_10 = (isIE9_10 && (isPseudoElement || isFontShorthand));
          var isUseless = (VALUES.test(value) || PROPS.test(prop));
          var isNotUseless = ((isIElte8 || !isIE9_10) && !isUseless);

          if ( isSpecialCaseIE9_10 || isNotUseless ) {

            value = value.replace(REGEX, function ($1) {
              return rounded(parseFloat($1) * toPx(options.rootValue, decl, result), options.unitPrecision) + 'px';
            });

            if (options.replace) {
              decl.value = value;
            } else {
              var clone = decl.clone({ value: value });
              if (decl.raws.before) {
                clone.raws.before = decl.raws.before;
                decl.raws.before = reduceLineBreaks(decl.raws.before);
              }
              rule.insertBefore(i, clone);
            }

          }

        }

      });

    });

  };

});

// Detect if one browser from the browserQuery is in browsers
function detectBrowser (browsers, browserQuery) {
  var b = false;
  browserQuery = browserslist(browserQuery);
  for (var i = 0; i < browsers.length; i++) {
    for (var j = 0; j < browserQuery.length; j++) {
      if (browsers[i] === browserQuery[j]) {
        b = true;
        break;
      }
    }
    if (b) { break; }
  }
  return b;
}

// Return a unitless pixel value from any root font-size value.
function toPx (value, decl, result) {
  value = (typeof value === 'string' && value.indexOf('calc(') !== -1) ? calc(value) : value;
  var parts = /^(\d*\.?\d+)([a-zA-Z%]*)$/.exec(value);
  if (parts !== null) {
    var number = parts[1];
    var unit   = parts[2];

    if (unit === 'px' || unit === '') {
      return parseFloat(number);
    }
    else if (unit === 'em' || unit === 'rem') {
      return parseFloat(number) * BASE_FONT_SIZE;
    }
    else if (unit === '%') {
      return (parseFloat(number) / 100) * BASE_FONT_SIZE;
    } else {
      // other units: vw, ex, ch, etc...
      result.warn('Unit cannot be used for conversion, so 16px is used.');
      return BASE_FONT_SIZE;
    }
  } else {
    throw decl.error('Root font-size is invalid', {plugin: 'pixrem'});
  }
}

// Reduce line breaks
function reduceLineBreaks (value) {
  return value.replace(/(\r*\n|\r)+/g, '$1');
}

// Round values based on precision
// rounded down to match webkit and opera behavior:
// http://tylertate.com/blog/2012/01/05/subpixel-rounding.html
function rounded (value, precision) {
  precision = Math.pow(10, precision);
  return Math.floor(value * precision) / precision;
}
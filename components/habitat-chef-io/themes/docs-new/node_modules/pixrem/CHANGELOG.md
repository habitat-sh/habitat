## Release History

4.0.1, July 06, 2017

* Fix CI build, testing only Node.js 4, 6 and stable

4.0.0, July 06, 2017

* Update to PostCSS 6.0 (no longer support Node.js 0.12)
* Update to Browserslist 2.0

3.0.2, August 22, 2016
* Fix reduce-css-calc security bug

3.0.1, May 24, 2016
* Fix #54: fallback not added when ie9 and ie 10 are in scope

3.0.0, Sep 23, 2015

* Export module as a PostCSS plugin (#35)
* Follow PostCSS plugin guidelines
* Removed `rootValue` parameter, now defined in options (#40)
* Removed old API. Always use pixrem with PostCSS API.
* Added: `unitPrecision` for rounded values

2.0.1, Sep 17, 2015

* Fix NaNpx values (#45)

2.0.0, Aug 24, 2015

* Update to PostCSS 5.0

1.3.2, Aug 24, 2015

* Unpublished version

1.3.1, Jul 9, 2015

* Fixed: Replace `eachDecl` with `each` and `decl.type` check in process function

1.3.0, Jul 1, 2015

* Added: Use browserslist to generate rem fallbacks only when needed

1.2.4, Apr 17, 2015

* Fixed: generate fallbacks with a value starting with dot

1.2.3, Mar 27, 2015

* Fix: copy and reduce decl.before, only if defined

1.2.2, Mar 27, 2015

* Fix root-font size detection

1.2.1, Mar 23, 2015

* Reduce line-breaks when inserting clone node

1.2.0, Feb 19, 2015

* Add option `html` to disable root font-size detection
* Fix root-font size defined with `calc`
* Throw error when root font-size is invalid

1.1.1, Feb 5, 2015:

* Fix root font-size detection

1.1.0, Jan 25, 2015:

* PostCSS 4
* Expose postcss processor

1.0.0, Nov 26, 2014: 

* Generate rem fallbacks only when needed
* Updated to PostCSS v3.0
* Get root font-size from CSS

0.1.4, March 6, 2014: Code optimization from AI.  
0.1.3, Dec 14, 2013: Fix regex for < 0 values.  
0.1.1, 0.1.2, Dec 14, 2013: Documentation improvements.  
0.1.0, Dec 14, 2013: Initial release.  
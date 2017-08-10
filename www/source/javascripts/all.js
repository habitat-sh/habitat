//= require vendor/jquery.min
//= require vendor/foundation.min
//= require vendor/what-input.min
//= require vendor/js.cookie
//= require nav.js
//= require home.js
//= require community.js
//= require tutorials.js
//= require demo.js
//= require highlight.pack

$(document).ready(function() {
  $('pre code').each(function(i, block) {
    hljs.initHighlightingOnLoad();
  });
});

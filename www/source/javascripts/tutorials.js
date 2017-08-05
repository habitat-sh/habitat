$(function() {
  var items = $('.main-sidebar--list li');
  var active;

  items.each(function(i, o) {
    if (o.className.indexOf('is-active') > 0) {
      active = i;
    }
  });

  items.each(function(i, o) {
    if (i <= active) {
      $(o).addClass('in-progress');
    }
  });
});

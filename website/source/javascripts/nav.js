var navBreakpoint = 670;
var $navLinks = $('.main-nav--links');

$('.main-nav--toggle').click(function() {
  $navLinks.slideToggle();
});

$(window).resize(function() {
  if ($(window).width() > navBreakpoint) {
    if ($navLinks.is(':hidden')) {
      $navLinks.css('display', '');
    }
  }
});
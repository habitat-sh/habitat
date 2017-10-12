$(function () {
  const navBreakpoint = 710;
  const $mainNav = $('#main-nav');
  const $navLinks = $('.main-nav--links');
  const $navToggle = $('.main-nav--toggle');
  const currentPagePath = location.pathname;
  const navPageLinks = ['about', 'docs', 'tutorials', 'community'];
  const stickyBreakpoint = 120;
  const stickyVisibleBreakpoint = 160;

  var toggleStickyNav = function () {
    if ($(window).width() > navBreakpoint) {
      $mainNav.toggleClass('is-sticky', $(window).scrollTop() > stickyBreakpoint);
      $mainNav.toggleClass('is-visible', $(window).scrollTop() > stickyVisibleBreakpoint);
      $('#content-outer').toggleClass('has-sticky-nav', $(window).scrollTop() > stickyBreakpoint);
    } else {
      $mainNav.removeClass('is-visible');
      $mainNav.toggleClass('is-sticky', $(window).scrollTop() > 0);
    }
  };

  toggleStickyNav();

  $navToggle.click(function () {
    $navLinks.slideToggle();
    $mainNav.toggleClass('is-open');
  });

  for (var linkName in navPageLinks) {
    var linkNamePath = '/' + navPageLinks[linkName];

    if (currentPagePath == linkNamePath) {
      $('.main-nav--links a.' + navPageLinks[linkName]).addClass('is-current-page');
    }
  };

  $(window).resize(function () {
    toggleStickyNav();

    if ($(window).width() > navBreakpoint) {
      $navLinks.attr('style', '');
      $mainNav.removeClass('is-open');
    }
  });

  $(window).scroll(function () {
    toggleStickyNav();
  });
});

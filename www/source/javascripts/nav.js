const navBreakpoint = 769;
const $mainNav = $('#main-nav');
const $navLinks = $('.main-nav--links');
const $navToggle = $('.main-nav--toggle');
const currentPagePath = location.pathname;
const navPageLinks = ['about', 'docs', 'tutorials', 'community', 'blog'];
const stickyBreakpoint = 280;
const stickyVisibleBreakpoint = 300;

(function nav($, cookies) {
  var profile;

  $(document).ready(function() {
    var signedOutElements = $(".signed-out");
    var signedInElements = $(".signed-in");
    var logo = $(".main-nav--logo a");
    var avatar = signedInElements.find(".avatar");
    var dropdown = signedInElements.find(".dropdown");
    var username = dropdown.find(".username");
    var signOutLink = dropdown.find(".sign-out");

    if (token()) {
      $.get("https://api.github.com/user?access_token=" + token())
        .then(function(p) {
          signIn(p);
        }, function(err) {
          console.error(err);
          signOut();
        });
    }
    else {
      signOut();
    }

    avatar.click(function(e) {
      e.stopPropagation();
      dropdown.toggle();
    });

    signOutLink.click(function() {
      signOut();
    });

    logo.click(function(e) {
      if (signedIn()) {
        e.preventDefault();
        location.href = logo.data("builder-url");
      }
    });

    $(document).click(function() {
      dropdown.hide();
    });

    function token() {
      return cookies.get("gitHubAuthToken");
    }

    function signedIn() {
      return !!token();
    }

    function signIn(profile) {
      avatar.find("img").attr("src", profile.avatar_url);
      username.text(profile.login);
      signedInElements.css("display", "inline-block");
    }

    function signOut() {
      cookies.remove("gitHubAuthState", { domain: cookieDomain() });
      cookies.remove("gitHubAuthToken", { domain: cookieDomain() });
      cookies.remove("featureFlags", { domain: cookieDomain() });
      signedOutElements.css("display", "inline-block");
      signedInElements.hide();
    }

    function cookieDomain() {
        let delim = ".";
        return location.hostname
            .split(delim)
            .splice(-2)
            .join(delim);
    }
  });
})($, Cookies);

var toggleStickyNav = function() {
  if ($mainNav.is(":not(.has-sidebar)")) {

    // We only apply the sticky nav
    if ($(window).width() <= navBreakpoint) {
      $mainNav.removeClass('is-visible');
      $mainNav.toggleClass('is-sticky', $(window).scrollTop() > 0);
    }
  }
};

toggleStickyNav();

$navToggle.click(function() {
  $navLinks.slideToggle();
  $mainNav.toggleClass('is-open');
});

for (var linkName in navPageLinks) {
  var linkNamePath = navPageLinks[linkName],
      currentPageRoot = currentPagePath.split('/')[1];
  if (currentPageRoot == linkNamePath) {
    $('.main-nav--links a.' + navPageLinks[linkName]).addClass('is-current-page');
  }
};

$(window).resize(function() {
  toggleStickyNav();

  if ($(window).width() > navBreakpoint) {
    $navLinks.attr('style', '');
    $mainNav.removeClass('is-open');
  }
});

$(window).scroll(function() {
  toggleStickyNav();
});

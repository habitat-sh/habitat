(function (win, doc,$) {
  $(doc).foundation();

  $(".prose > :header").each(function () {
    $(this).append("<a href=\"\#" + $(this).attr("id") + "\"><i class=\"fas fa-link\"></i></a>");
  });

  // Adjust the alignment on the dropdown when the header is minimized
  $("nav[data-sticky]").on("sticky.zf.stuckto:top", function () {
    $("#getting-started-dropdown").css("margin-top", "-50px");
  });

  $("nav[data-sticky]").on("sticky.zf.unstuckfrom:top", function () {
    $("#getting-started-dropdown").css("margin-top", "-1px");
  });

  // Workaround Foundation sticky bug
  // https://github.com/zurb/foundation-sites/issues/9892
  $(window).on('sticky.zf.unstuckfrom:bottom', function(e) {
    if (Foundation.MediaQuery.is('small only')) {
      $(e.target).removeClass('is-anchored is-at-bottom').css('top', 'auto');
    }
  });

  // Menu behavior and positioning
  if ($('nav[data-product="automate"]')) {
    var menu = '#menu nav';
    var menuItems = '#header-menu .menu-items > li';
    var menuContent = '.menu-content';
    var menuContentItems = '.menu-content .item';
    var hoverTimeout;

    $(menuItems)
      .mouseover(function () {
        if (Foundation.MediaQuery.atLeast('medium')) {
          clearTimeout(hoverTimeout);

          var item = showMenuItem($(this).index());
          setMenuPosition();
          showMenu();

          item
            .parents(menuContent)
            .css('height', item.outerHeight());
        }
      })
      .click(function(e) {
        if ($(this).hasClass('has-content')) {
          e.preventDefault();
          showMenuItem($(this).index());
          showMenu();
        }
      });

    $('.back a').click(function(e) {
      e.preventDefault();
      hideMenu();
    });

    $('nav header, .menu-content').mouseleave(function () {
      if (Foundation.MediaQuery.atLeast('medium')) {
        hideMenu();
      }
    });

    $(doc)
      .scroll(function() { setMenuPosition(!isMobile()); })
      .resize(function() { setMenuPosition(true); })

    $(win)
      .on('changed.zf.mediaquery', function() { setMenuPosition(false); });

    function setMenuPosition(hide) {
      var top = 0;

      if (!Foundation.MediaQuery.is('small only')) {
        top = $('#header-menu').height()
      }

      $(menuContent)
        .css('top', top);

      if (isMobile()) {
        $(menuContent)
          .css('height', $(win).height() - top);
      }

      if (hide) {
        hideMenu();
      }
    }

    function showMenu() {
      setMenuPosition();
      $(menu).addClass('open');
    }

    function showMenuItem(index) {
      return $(menuContentItems)
        .css('display', 'none')
        .eq(index)
        .css('display', 'block');
    }

    function hideMenu() {
      hoverTimeout = setTimeout(function() { $(menu).removeClass('open'); }, 100);
    }

    function isMobile() {
      return Foundation.MediaQuery.is('small only');
    }

    setMenuPosition();
  }

}(window, document, jQuery));

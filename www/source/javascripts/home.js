var homepageScripts = function() {
  var adjustParentHeight = function($elements, $parent) {
    var maxElementHeight = 0;
    var currentElementHeight;

    $elements.each(function() {
      currentElementHeight = $(this).outerHeight(true);

      if (currentElementHeight > maxElementHeight) {
        maxElementHeight = currentElementHeight;

        $parent.css("height", maxElementHeight);
      }
    });
  };

  // Testimonials slider
  const $testimonials = $(".testimonial");
  const testimonialsSlider = ".testimonials-slider";
  const $testimonialText = $(".testimonial--blurb");
  var $currentSlide, testimonialHeight, currentTestimonialHeight;

  adjustParentHeight($testimonials, $(testimonialsSlider));

  $(window).resize(function() {
    adjustParentHeight($testimonials, $(testimonialsSlider));
  });

  $('.testimonials--nav--dot').click(function() {
    var posClass = $(this).attr("class").split(' ')[1];

    $('.testimonial, .testimonials--nav--dot').removeClass('is-active');
    $('.' + posClass).addClass('is-active');
  })

  setInterval(function() {
    $currentSlide = $(".testimonial.is-active");

    $(testimonialsSlider + " .is-active").removeClass("is-active");

    if ($currentSlide.hasClass("first")) {
      $(testimonialsSlider + " .second").addClass("is-active");
    } else if ($currentSlide.hasClass("second")) {
      $(testimonialsSlider + " .third").addClass("is-active");
    } else if ($currentSlide.hasClass("third")) {
      $(testimonialsSlider + " .fourth").addClass("is-active");
    } else if ($currentSlide.hasClass("fourth")) {
      $(testimonialsSlider + " .fifth").addClass("is-active");
    } else if ($currentSlide.hasClass("fifth")) {
      $(testimonialsSlider + " .sixth").addClass("is-active");
    } else if ($currentSlide.hasClass("sixth")) {
      $(testimonialsSlider + " .first").addClass("is-active");
    }

  }, 15000);

  // Sub-hero logo sliders
  var lastScrollPosition = 0;

  var hasScrollBar = function($element, $parent) {
    return $element.width() > $parent.width();
  };

  var elementIsVisible = function($element) {
    var windowScrollBottom = $(window).scrollTop() + $(window).height();
    var elementBottomPosition = $element.offset().top + $element.outerHeight();

    return windowScrollBottom > elementBottomPosition;
  };

  var canScrollLeft = function($element, $parent) {
    return $parent.scrollLeft() < $element.width();
  };

  var animateScroll = function($element) {
    var $image = $element.children(".home--sub-hero--logo");
    var currentScrollPosition = $(window).scrollTop();
    var elementScrollShift = currentScrollPosition < lastScrollPosition ? 0 : 1;

    lastScrollPosition = currentScrollPosition;

    if (hasScrollBar($image, $element) && elementIsVisible($element) && canScrollLeft($image, $element)) {
      $element.scrollLeft($element.scrollLeft() + elementScrollShift);
    }
  };

  $(window).scroll(function() {
    animateScroll($(".home--sub-hero--logos.containers"));
    animateScroll($(".home--sub-hero--logos.applications"));
  });
};

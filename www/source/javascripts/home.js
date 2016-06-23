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

  // Feature Slider
  const homepageSlides = ["plan-and-config", "build-packages"];
  const $sliderButtons = $(".slider--nav button");
  const $slides = $(".slide");
  const $slidesWrap = $(".slides");
  var slideHeight;

  $("#slide--plan-and-config").fadeIn();

  $sliderButtons.click(function() {
    var $button = $(this);
    $(".slider--nav .is-active").removeClass("is-active")
    $button.addClass("is-active");

    for (var slide in homepageSlides) {
      if ($button.hasClass(homepageSlides[slide]) && $("#slide--" + homepageSlides[slide]).is(":hidden")) {
        $(".slide").fadeOut();
        $("#slide--" + homepageSlides[slide]).fadeIn();
      }
    }

    adjustParentHeight($slides, $slidesWrap);
  });

  adjustParentHeight($slides, $slidesWrap);

  $(window).resize(function() {
    adjustParentHeight($slides, $slidesWrap);
  });

  // Production Icons
  const icons = ["lock", "search", "settings", "file", "health"];
  const $productionIcons = $(".production--icon");

  $productionIcons.click(function() {
    $icon = $(this);
    var $iconText;

    for (var iconName in icons) {
      $iconText = $(".production--icon-text." + icons[iconName]);

      if ($icon.hasClass(icons[iconName]) && !$iconText.hasClass("is-active")) {
        $(".production--graphic .is-active").removeClass("is-active");
        $icon.addClass("is-active");
        $iconText.addClass("is-active");
      }
    }
  });

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
};

var homepageScripts = function() {
  window.adjustParentHeight = function($elements, $parent) {
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

  var mainCarousel = new Carousel($('.home--hero-carousel-slide'), $('.home--hero-carousel-nav-item'));
  var testimonialCarousel = new Carousel($('.home--testimonial-carousel-slide'), $('.home--testimonial-carousel-nav-item'));

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

  $(function() {
    var htmlDecode = function(input) {
      var doc = new DOMParser().parseFromString(input, "text/html");
      return doc.documentElement.textContent;
    }
    var makeCommunityEvent = function(e) {
      if (e === undefined) {
        return e;
      }

      var eventType;

      if (e["event_category"]) {
        var parts = e["event_category"].split(",");
        var index = parts.indexOf("Habitat");

        if (index >= 0) {
          parts.splice(index, 1);
        }

      }

      var eventLink = htmlDecode(e["guid"]);
      var eventName = e["event_name"];
      var eventDateUrl = "https://events.chef.io/events/s/category/habitat/?scope[0]=" + e["start_date"] + "&scope[1]=" + e["start_date"];
      var eventDateEncoded = encodeURI(eventDateUrl);
      var eventDate = e["start_date"];
      var eventLocation = "";

      if (e.event_location && e.event_location.city && e.event_location.country) {
        eventLocation = e.event_location.city + ", " + e.event_location.country;
      }
      var weekdayNames = ["Monday", "Tuesday", "Thursday", "Friday", "Saturday", "Sunday"]
      var monthNames = [ "January", "February", "March", "April", "May", "June",
          "July", "August", "September", "October", "November", "December" ];
      var newDate = new Date(e["start"] * 1000);
      var formattedDate = weekdayNames[newDate.getDay()] + ', '  + monthNames[newDate.getMonth()] + ' ' + newDate.getDate();
      var month = monthNames[newDate.getMonth()];

      var t = $("#homepage-event-template").clone().show().html();
      var template = t.replace("{{ event_link }}", eventLink)
                      .replace("{{ event_name }}", eventName)
                      .replace("{{ day_of_month }}", newDate.getDate())
                      .replace("{{ month_name }}", month)
                      .replace("{{ event_date_url }}", eventDateEncoded)
                      .replace("{{ event_date }}", formattedDate)
                      .replace("{{ event_location }}", eventLocation);

      return template;
    }

    if ($(".homepage--events--content").length) {
      $.getJSON("https://events.chef.io/wp-json/events/category/habitat", function(data) {
        if (Array.isArray(data)) {
          for (var i=0; i < 1; i+=1) {
            var row = $("<div>", {
              class: "home--hero--highlight--wrap"
            });

            var eventHome = makeCommunityEvent(data[i]);

            row.append(eventHome);

            $("div.homepage--events--content").append(row);
          }
        }
      });
    }
  });
};

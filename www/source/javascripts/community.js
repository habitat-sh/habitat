$(function() {
  var htmlDecode = function(input) {
    var doc = new DOMParser().parseFromString(input, "text/html");
    return doc.documentElement.textContent;
  }

  console.log("Testing, hellos...");

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

      eventType = parts[0];
    } else {
      eventType = "Conference";
    }

    var eventLink = htmlDecode(e["guid"]);
    var eventName = e["event_name"];
    var omg = "<div>" + e["post_content"] + "</div>";
    var eventDescription = $(omg).text().replace(/(\r\n)|(\n)|(\r)/g, "<br>");
    var eventDateUrl = "https://events.chef.io/events/s/category/habitat/?scope[0]=" + e["start_date"] + "&scope[1]=" + e["start_date"];
    var eventDateEncoded = encodeURI(eventDateUrl);
    var eventDate = e["start_date"];
    var eventLocation = "";

    if (e.event_location && e.event_location.city && e.event_location.country) {
      eventLocation = e.event_location.city + ", " + e.event_location.country;
    }

    var t = $("#community-event-template").clone().show().html();
    var template = t.replace("{{ event_type }}", eventType)
                    .replace("{{ event_link }}", eventLink)
                    .replace("{{ event_name }}", eventName)
                    .replace("{{ event_description }}", eventDescription)
                    .replace("{{ event_date_url }}", eventDateEncoded)
                    .replace("{{ event_date }}", eventDate)
                    .replace("{{ event_location }}", eventLocation);

    return template;
  }

  if ($(".community--events--content").length) {
    $.getJSON("https://events.chef.io/wp-json/events/category/habitat", function(data) {
      if (Array.isArray(data)) {
        for (var i=0; i < data.length; i+=3) {
          var row = $("<div>", {
            class: "row"
          });

          var first = makeCommunityEvent(data[i]);
          var second = makeCommunityEvent(data[i+1]);
          var third = makeCommunityEvent(data[i+2]);

          row.append(first);
          if (second) {
            row.append(second);
          }
          if (third) {
            row.append(third);
          }

          $("div.community--events--content").append(row);
        }
      }
    });
  }
});

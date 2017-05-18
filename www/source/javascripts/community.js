$(function() {
  if ($(".community-events")) {
    var url = DEPOT_API + "/community-events";

    $.ajax(url, {
      success: function(data, status, xhr) {
        if (typeof data === "object" && typeof data.length === "number") {
          data.forEach(function(e) {
            var ev = "<p>";
            ev += e.title;
            ev += "<br>";
            ev += e.link;
            ev += "<br>";
            ev += e.pub_date;
            ev += "<br>";
            ev += e.description;
            ev += "</p>";

            $(".community-events").append(ev);
          });
        }
      }
    });
  }
});

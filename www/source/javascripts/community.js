var community_events = function(data) {
    if ($(".community-events")) {
        if (Array.isArray(data)) {
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

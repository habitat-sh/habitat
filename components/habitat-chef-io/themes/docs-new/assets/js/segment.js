(function($) {
  var segment_map = {
    "www.chef.sh" : "sYovbR5fTkQ7mhPPGELL9LERcLqtGWxp",
    "chef-www-acceptance.cd.chef.co" : "CsRaWq2uKeUOlNhIr5cO8WuJfEqHZsKs",
    "automate.chef.io" : "ynCGZG2VmLeCOdrHU86VNFR55SEk4taC",
    "automate-www-acceptance.cd.chef.co" : "kSzzeQTmlbEQ9FmYOcRMzwQtmQ5U28AW"
  };

  // Find the segment ID for the hostname
  var segment_id = segment_map[location.hostname];

  // Un-minified version of the segment javascript provided by segment
  var analytics = window.analytics = window.analytics || [];
    if (!analytics.initialize)
        if (analytics.invoked) window.console && console.error && console.error("Segment snippet included twice.");
        else {
            analytics.invoked = !0;
            analytics.methods = ["trackSubmit", "trackClick", "trackLink", "trackForm", "pageview", "identify", "reset", "group", "track", "ready", "alias", "debug", "page", "once", "off", "on"];
            analytics.factory = function(t) {
                return function() {
                    var e = Array.prototype.slice.call(arguments);
                    e.unshift(t);
                    analytics.push(e);
                    return analytics
                }
            };
            for (var t = 0; t < analytics.methods.length; t++) {
                var e = analytics.methods[t];
                analytics[e] = analytics.factory(e)
            }
            analytics.load = function(t, e) {
                var n = document.createElement("script");
                n.type = "text/javascript";
                n.async = !0;
                n.src = ("https:" === document.location.protocol ? "https://" : "http://") + "cdn.segment.com/analytics.js/v1/" + t + "/analytics.min.js";
                var o = document.getElementsByTagName("script")[0];
                o.parentNode.insertBefore(n, o);
                analytics._loadOptions = e
            };
            analytics.SNIPPET_VERSION = "4.1.0";

            // Here are the modifications from the original script.
            // We make sure that we are running on a recognized FQDN.
            // If we are, we load the analytics. If we are not, we do not.
            if ( segment_id ) {
              analytics.load(segment_id);
              analytics.page();
            } else {
              console.log("We have no Segment tracking data for this domain.");
            }
        }
}(jQuery));

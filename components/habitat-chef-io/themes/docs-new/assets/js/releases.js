// Fetch the given URL parameters
$.urlParam = function(name){
  var results = new RegExp('[\?&]' + name + '=([^]*)').exec(window.location.href);
  if (results==null){
      return null;
  }
  else{
      return results[1] || 0;
  }
}

// Find the index for the object
function findWithAttr(array, attr, value) {
  for(var i = 0; i < array.length; i += 1) {
      if(array[i][attr] === value) {
          return i;
      }
  }
  return -1;
}

// Load the Release Notes into the page
function loadReleaseNotesContents(releases, version) {
  if ( version == null ) {
    var index = releases.length - 1;
  } else {
    var index = findWithAttr(releases, "version", version);
  }

  var converter = new showdown.Converter();
  $.get(releases[index]["_links"]["release_notes"], function(rawReleaseNotes) {
    var html = converter.makeHtml(rawReleaseNotes);

    var friendlyDate = new Date(releases[index]["release_date"]);
    var options = { year: 'numeric', month: 'long', day: 'numeric' };

    $("#main-content-col").html(html);
    $("#main-content-col").prepend("<p><i>Released on " + friendlyDate.toLocaleString('en-US', options) + "</i></p>");
    $("#main-content-col").prepend("<h1>Version " + releases[index]["version"] + "</h1>");
  });
}

function loadNavigationBar(releases, version) {
  var groupedReleases = {};

  for ( var i = 0; i < releases.length; i++) {
    var date = new Date(releases[i]["release_date"]);
    var formattedMonth = ("0" + (date.getMonth() + 1)).slice(-2);
    var navDate = date.getFullYear() + "-" + formattedMonth;

    groupedReleases[navDate] = ( typeof groupedReleases[navDate] != 'undefined' && groupedReleases[navDate] instanceof Array ) ? groupedReleases[navDate] : []
    groupedReleases[navDate].push(releases[i]);
  }

  $(".nav").append("<ul id=\"release-notes-menu\" class=\"vertical menu accordion-menu\" data-accordion-menu></ul>");
  $.each(groupedReleases, function(navDate, releases) {
    if ( releases.length > 0 ) {
      var releaseList = "";
      var classes = "menu vertical nested";
      var li = "<li>";

      $.each(releases, function(_, release) {
        if ( version == release["version"] ) {
          li = '<li class="active">';
          classes = "menu vertical nested is-active";
        } else {
          li = "<li>";
        }

        releaseList += li + "<a href=\"?v=" + release["version"] + "\">Version " + release["version"] + "</a></li>";
      });

      $("#release-notes-menu").prepend("<li><a>" + navDate + "</a><ul class=\""+ classes + "\">" + releaseList + "</ul></li>");
    }
  });

  if ( version == null ) {
    var lrLi = '<li class="active">';
  } else {
    var lrLi = '<li>'
  }

  $("#release-notes-menu").prepend(lrLi + "<a href=\"/release-notes/\">Latest Release</a></li>");
  $(".nav").foundation();
}

function loadReleaseNotesPage(product) {
  var version = $.urlParam('v');

  // Right now the only site that uses this is automate, and for them we always
  // want to use the 'current' channel
  var channel = "current";

  $.getJSON({
    type: "GET",
    headers: {"Accept": "application/json"},
    url: "https://packages.chef.io/releases/" + channel + "/" + product + ".json"
  }).done(function (releases) {
    loadNavigationBar(releases, version);
    loadReleaseNotesContents(releases, version);
  });
}

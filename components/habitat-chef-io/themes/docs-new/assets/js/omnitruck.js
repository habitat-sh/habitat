(function($) {
  var parser = new UAParser();

  // https://developers.whatismybrowser.com/useragents/explore/operating_system_name/
  // Useful for testing
  // var uastring = "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1";
  // parser.setUA(uastring);

  var ua = parser.getResult();

  // Need to validate these mappings
  function calcMachine(arch) {
    switch(arch) {
      case "amd64":
        return "x86_64";
      case "ia32":
        return "i386";
      default: // 68k, arm, arm64, ia64, irix, irix64, mips, mips64, pa-risc, ppc, sparc, sparc64
        return undefined;
    }
  }

  function calcWindowsPV(version) {
    switch(version) {
      // NT 6.1
      case "7":
      case "2008r2":
        return "2008r2";
      // NT 6.2
      case "8":
      case "2012":
        return "2012";
      // NT 6.3
      case "8.1":
      case "2012r2":
        return "2012r2";
      // NT 10
      case "10":
      case "2016":
        return "2016";
      default:
        return undefined;
    }
  }

  if ($("a[data-omnitruck-download]").length > 0) {
    var downloadable = true;
    var element = $("a[data-omnitruck-download]").first();

    switch(ua.os.name) {
      case "Mac OS":
        platform = "mac_os_x";
        platform_version = ua.os.version;
        machine = "x86_64";
        break;
      case "Windows":
        platform = "windows";
        platform_version = calcWindowsPV(ua.os.version);
        machine = calcMachine(ua.cpu.architecture);
        break;

      default:
        downloadable = false;
        break;
    }

    var product = element.attr("data-omnitruck-download")
    var channel = element.attr("data-omnitruck-channel") || "stable"

    if (downloadable == false || platform == undefined || platform_version == undefined || machine == undefined) {
      element.attr("href", "https://downloads.chef.io/" + product + "/" + channel);
    } else {
      $.getJSON({
        type: "GET",
        headers: {"Accept": "application/json"},
        url: "https://omnitruck.chef.io/"+ channel + "/" + product + "/metadata?p=" + platform + "&pv=" + platform_version + "&m=" + machine
      }).done(function (data) {
        element.attr("href", data["url"]);
      });
    }
  }
} (jQuery));

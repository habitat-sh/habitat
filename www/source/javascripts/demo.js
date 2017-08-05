//= require vendor/ansi_up
//= require vendor/underscore.min
//= require vendor/codemirror/codemirror
//= require vendor/codemirror/matchbrackets
//= require vendor/codemirror/shell
//= require vendor/josh/killring
//= require vendor/josh/history
//= require vendor/josh/readline
//= require vendor/josh/shell
//= require vendor/josh/pathhandler

/*global $, ansi_up, CodeMirror, Josh */

// Without this, you cannot open the keyboard on mobile devices since josh.js
// does not use actual HTML input elements.
$("#mobile-keyboard-trigger").click(function() {
  $(this).focus();
});

(function () {
    function format(text) {
        return "<pre>" + ansi_up.ansi_to_html(ansi_up.escape_for_html(text)) +
            "</pre>";
    }

    function getResponse(name) {
        return $.get("/tutorials/get-started/demo/responses/" + name + ".txt");
    }

    function getExample(name, callback) {
        getResponse(name).then(function (txt) {
                        inStudio = false;
                        callback(format(txt));
                        shell.setPrompt(rootPrompt);
                        shell.onNewPrompt(function(callback) {
                            promptCounter += 1;
                            callback(rootPrompt);
                        });
        });
    }

    var editor;
    var editorEl = document.getElementById("try-habitat-editor");
    var history = new Josh.History({ key: 'helloworld.history'});
    var inStudio = false;
    var promptCounter = 1;
    var rootPrompt = "user@workstation-machine:~$";
    var shell = Josh.Shell({ history: history });
    var studioPrompt = "<span class='ansi-green'>[</span>" +
        "<span class='ansi-cyan'>" + promptCounter +
        "</span><span class='ansi-green'>][habitat:</span>" +
        "<span class='ansi-magenta'>/src</span>" +
        "<span class='ansi-green'>:</span>0<span class='ansi-green'>]$</span>";

    shell.setCommandHandler("exit", {
        exec: function(cmd, args, callback) {
            inStudio = false;
            shell.setPrompt(rootPrompt);
            shell.onNewPrompt(function(callback) {
                callback(rootPrompt);
            });
            $("#shell-cli").parent().empty();
            callback();
        }
    });

    // studio commands
    shell.setCommandHandler("studio", {
        exec: function(cmd, args, callback) {

            // studio enter
            if (args[0] === "enter") {
                getResponse("studio-enter").then(function (txt) {
                    inStudio = true;
                    callback(format(txt));
                    shell.setPrompt(studioPrompt);
                    shell.onNewPrompt(function(callback) {
                        promptCounter += 1;
                        callback(studioPrompt);
                    });
                });

            // studio help
            } else if (args[0] === "help") {
                getResponse("studio-help").then(function (txt) {
                    callback(format(txt));
                });

            // studio <unsupported>
            } else {
                getResponse("studio-help").then(function (txt) {
                    callback(format(txt) + "<br>In this shell, only the " +
                        "<em>enter</em> subcommand is available. See " +
                        "<a target='_blank' href='#'>the documentation</a> " +
                        "to see what you can use in an actual shell.<br><br>");
                });
            }
        },
        completion: function(cmd, arg, line, callback) {
            callback(shell.bestMatch(arg, ["enter", "build", "help", "new",
                "rm", "run", "version"]));
        }
    });

    shell.setCommandHandler("curl", {
         exec: function(cmd, args, callback) {

            if (args.join(" ") === "http://172.17.0.2:9631/services/postgresql/default/health") {
               getExample("curl-health-check-postgres", callback);

            } else if (args.join(" ") === "http://172.17.0.3:9631/services/myrubyapp/default/health"){
               getExample("curl-health-check-ruby", callback);

            } else {
                getResponse("hab-sup-config-help").then(function (txt) {
                    callback(format(txt));
                });
            }
         }
    });

    // hab commands
    shell.setCommandHandler("hab", {
        exec: function(cmd, args, callback) {

          switch(args[0]) {
             case "start":

                if (args.join(" ") ===  "start example/myrubyapp") {
                    getExample("hab-start-service", callback);
                } else if (args.join(" ") ===  "start example/myrubyapp --peer 172.17.0.2 --bind database:postgresql.default") {
                    getExample("hab-bind", callback);
                } else if (args.join(" ") ===  "start core/postgresql -t leader") {
                    getExample("hab-start-first-node", callback);

                    } else if (args.join(" ") ===  "start core/postgresql -t leader --peer 172.17.0.2") {
                        getExample("hab-start-additional-node", callback);

                        $(".node-status").html("connected").parent().addClass("updated");
                        $(".button-badge, .full-output").show();
                    } else {
                    getResponse("hab-start-help").then(function (txt) {
                        callback(format(txt));
                    });
                }
                break;
            case "config":

            // Apply service group configuration
            // inject the config.toml into the group
            if (args.join(" ") === "config apply --peer 172.17.0.2 myrubyapp.default 1 update_config.toml") {

                    getExample("hab-config-apply", callback);

                    $(".node-status").html("change applied").parent().addClass("updated");
                    $(".button-badge, .full-output").show();
                } else
                   getResponse("hab-config-apply-help").then(function (txt) {
                       callback(format(txt));
                   });
                break;
             case "sup":
                // Find out the status of the service from the supervisor
                if (args.join(" ") === "sup status core/postgresql") {
                  getExample("hab-monitor-postgres", callback);
                } else if(args.join(" ") === "sup status example/myrubyapp") {
                  getExample("hab-monitor-myrubyapp", callback);
                } else {
                  getResponse("hab-sup-config-help").then(function (txt) {
                      callback(format(txt));
                  });
                }
                break;
             default:
               // the user entered `hab` followed by an unsupported subcommand let's show them `hab help` for a list of available emulator subcommands and link to docs for the full subcommand list
               getResponse("hab-help").then(function (txt) {
                   callback(format(txt) + "Note: In this demo shell, only a " +
                       "few subcommands are available.<br>See the " +
                       "<a target='_blank' href='/docs/overview/'>Habitat documentation</a> " +
                       "for a more complete liste of features.<br><br>");
               });
          }
       },
    });
    shell.setPrompt(rootPrompt);
    shell.activate();

    // Switching windows when adding services
    $(".window-buttons .button").click(function(event) {
        var targetID = $(this).attr("data-target");

        // set button classes
        $(".window-buttons .button").removeClass("active");
        $(this).addClass("active");

        // show/hide windows
        $(".window-node").hide();
        $("#" + targetID).show();
    });

    // Hack to allow pasting.
    // See https://github.com/sdether/josh.js/issues/28
    //$("#shell-panel").pastableNonInputable();
    $("#shell-panel").on("pasteText", function (event, data) {
        shell.addText(data.text);
    });
}());

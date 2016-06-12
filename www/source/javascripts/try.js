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
        return $.get("/try/responses/" + name + ".txt");
    }

    function success(display) {
        $("#success .button").removeClass("secondary").addClass("cta");
        if(display === "hide") {
          $("#shell-cli").hide();
        }
    }

    var editor;
    var editorEl = document.getElementById("try-habitat-editor");
    var history = new Josh.History({ key: 'helloworld.history'});
    var inStudio = false;
    var promptCounter = 1;
    var rootPrompt = "root@769165f9f0b6:/src#";
    var shell = Josh.Shell({ history: history });
    var step = parseInt($("#try-habitat-progress").data("step"), 10);
    var studioPrompt = "<span class='ansi-green'>[</span>" +
        "<span class='ansi-cyan'>" + promptCounter +
        "</span><span class='ansi-green'>][habitat:</span>" +
        "<span class='ansi-magenta'>/src</span>" +
        "<span class='ansi-green'>:</span>0<span class='ansi-green'>]$</span>";
    var initialPrompt = step === 0 ? rootPrompt : studioPrompt;

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
                    if (step === 1) { success(); }
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

    // hab commands
    shell.setCommandHandler("hab", {
        exec: function(cmd, args, callback) {

            // hab start [subcommand]
            if (args[0] === "start") {

                // Start a service
                if (args[1] === "core/redis") {
                  getResponse("hab-start-service").then(function (txt) {
                      inStudio = true;
                      callback(format(txt));
                      shell.setPrompt(studioPrompt);
                      shell.onNewPrompt(function(callback) {
                          promptCounter += 1;
                          callback(studioPrompt);
                      });

                      if (step === 1) { success(); }
                  });

                // Adding a leader/follower topology
                } else if (args[1] + ' ' + args[2] === "-t leader") {
                  if (args[3] === 'core/redis') {

                    // adding the first/leader node
                    if ((args[4] == null) && (step === 6)) {
                      getResponse("hab-start-first-node").then(function (txt) {
                          inStudio = true;
                          callback(format(txt));
                          shell.setPrompt(studioPrompt);
                          shell.onNewPrompt(function(callback) {
                              promptCounter += 1;
                              callback(studioPrompt);
                          });

                          if (step === 6) { success('hide'); }
                      });

                    // adding an additional/follower node
                    } else if (args[4] + ' ' + args[5] === "--peer 172.17.0.4") {
                      getResponse("hab-start-additional-node").then(function (txt) {
                          inStudio = true;
                          callback(format(txt));
                          shell.setPrompt(studioPrompt);
                          shell.onNewPrompt(function(callback) {
                              promptCounter += 1;
                              callback(studioPrompt);
                          });
                          //change button text to reflect resulting status
                          //show badge on window buttons
                          //show full updated cli output
                          if (step === 7) {
                            success();

                            $(".node-status").html("connected").parent().addClass("updated");
                            $(".button-badge, .full-output").show();
                          }
                      });

                    // they could be on step 6 or 7 since the command/subcommand is the same
                    } else if (step === 7) {
                      getResponse("hab-follower-help").then(function (txt) {
                          callback(format(txt));
                      });

                    } else {
                      getResponse("hab-leader-help").then(function (txt) {
                          callback(format(txt));
                      });
                    };
                  } else {
                    getResponse("hab-leader-help").then(function (txt) {
                        callback(format(txt));
                    });
                  }
                // if user tries to start anything else, then show the 'hab start' help
                } else {
                  getResponse("hab-start-help").then(function (txt) {
                      callback(format(txt));
                  });
                };

            // hab sup [subcommand]
            } else if (args[0] === "sup") {

              // Ask what is configurable
              if (args.join(" ") === "sup config core/redis") {
                getResponse("hab-sup-config-service").then(function (txt) {
                    inStudio = true;
                    callback(format(txt));
                    shell.setPrompt(studioPrompt);
                    shell.onNewPrompt(function(callback) {
                        promptCounter += 1;
                        callback(studioPrompt);
                    });

                    if (step === 2) { success(); }
                });

              // if user tries to sup anything else, then show the 'hab sup' help
              } else {
                getResponse("hab-sup-config-help").then(function (txt) {
                    callback(format(txt));
                });
              };

            // hab inject (alias)
            } else if (args[0] === "config") {
              // inject the config.toml into the group
              if (args.join(" ") === "config apply redis.default 1 /tmp/config.toml --peer 172.17.0.4") {
                getResponse("hab-config-apply").then(function (txt) {
                    inStudio = true;
                    callback(format(txt));
                    shell.setPrompt(studioPrompt);
                    shell.onNewPrompt(function(callback) {
                        promptCounter += 1;
                        callback(studioPrompt);
                    });
                    // step is pulled from the progress bar attribute in the DOM
                    if (step === 5) {
                      success();

                      //change button text to reflect resulting status
                      //show badge on window buttons
                      //show full updated cli output
                      $(".node-status").html("change applied").parent().addClass("updated");
                      $(".button-badge, .full-output").show();
                    }
                });

              // show the 'hab inject' help
              } else {
                getResponse("hab-config-apply-help").then(function (txt) {
                    callback(format(txt));
                });
              };

            // hab -strategy
            } else if (args[0] === "-strategy") {
              // TODO strategy
              if (args.join(" ") === "-strategy") {
                getResponse("hab-strategy").then(function (txt) {
                    inStudio = true;
                    callback(format(txt));
                    shell.setPrompt(studioPrompt);
                    shell.onNewPrompt(function(callback) {
                        promptCounter += 1;
                        callback(studioPrompt);
                    });
                    // step is pulled from the progress bar attribute in the DOM
                    if (step === 8) { success(); }
                });

              // show the 'hab inject' help
              } else {
                getResponse("hab-help").then(function (txt) {
                    callback(format(txt));
                });
              };

              // hab bind
              } else if (args[0] === "bind") {
                // TODO bind
                if (args.join(" ") === "bind") {
                  getResponse("hab-bind").then(function (txt) {
                      inStudio = true;
                      callback(format(txt));
                      shell.setPrompt(studioPrompt);
                      shell.onNewPrompt(function(callback) {
                          promptCounter += 1;
                          callback(studioPrompt);
                      });
                      // step is pulled from the progress bar attribute in the DOM
                      if (step === 9) { success(); }
                  });

                // show the 'hab inject' help
                } else {
                  getResponse("hab-help").then(function (txt) {
                      callback(format(txt));
                  });
                };

            // show hab help
            } else if (args[0] === "help") {
                getResponse("hab-help").then(function (txt) {
                    callback(format(txt));
                });
            } else {
                // the user entered `hab` followed by an unsupported subcommand let's show them `hab help` for a list of available emulator subcommands and link to docs for the full subcommand list
                getResponse("hab-help").then(function (txt) {
                    callback(format(txt) + "Note: In this demo shell, only a " +
                        "few subcommands are available.<br>See the " +
                        "<a target='_blank' href='#'>Habitat documentation</a> " +
                        "for a more complete liste of features.<br><br>");
                });
            }
        },
        completion: function(cmd, arg, line, callback) {
            callback(shell.bestMatch(arg, ["start"]));
        }
    });

    // Configuration change through environment variable
    shell.setCommandHandler('HAB_REDIS="tcp-backlog=128"', {
        exec: function(cmd, args, callback) {

            if (args[0] === "hab") {

              // Reconfigure service via environment variable
              if (args.join(" ") === "hab start core/redis") {
                getResponse("hab-start-env-var").then(function (txt) {
                    inStudio = true;
                    callback(format(txt));
                    shell.setPrompt(studioPrompt);
                    shell.onNewPrompt(function(callback) {
                        promptCounter += 1;
                        callback(studioPrompt);
                    });

                    if (step === 3) { success(); }
                });

            // if user tries to start anything else, then show the 'hab start' help
          } else if ((args[1] === "start") && (args[2] !== "core/redis")) {
              getResponse("hab-start-help").then(function (txt) {
                  callback(format(txt));
              });

            } else {
                getResponse("hab-help").then(function (txt) {
                    callback(format(txt));
                });
              };


            // show hab help if they wander
            } else if (args[0] === "help") {
                getResponse("hab-start-help").then(function (txt) {
                    callback(format(txt));
                });
            } else {
                // the user entered an unsupported subcommand
                getResponse("hab-start-help").then(function (txt) {
                  callback(format(txt) + "Note: In this demo shell, only a " +
                      "few subcommands are available.<br>See the " +
                      "<a target='_blank' href='#'>Habitat documentation</a> " +
                      "for a more complete liste of features.<br><br>");
                });
            }
        },
        completion: function(cmd, arg, line, callback) {
            callback(shell.bestMatch(arg, ["start"]));
        }
    });

    shell.setPrompt(initialPrompt);
    shell.activate();

    // Text Editor steps via CodeMirror
    if (editorEl) {
        editor = CodeMirror.fromTextArea(editorEl, {
            mode: "shell",
            lineNumbers: true,
            matchBrackets: true,
        });

        editor.on("changes", function (instance, changes) {
            if (step === 4 &&
                instance.getValue().match(/tcp-backlog\s\=\s*128\s*/)) {
                success();
            }
        });
    }

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
    $("#shell-panel").pastableNonInputable();
    $("#shell-panel").on("pasteText", function (event, data) {
        shell.addText(data.text);
    });
}());

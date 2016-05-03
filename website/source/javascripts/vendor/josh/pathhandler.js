/* ------------------------------------------------------------------------*
 * Copyright 2013-2014 Arne F. Claassen
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0

 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *-------------------------------------------------------------------------*/

var Josh = Josh || {};
(function(root, $, _) {
  Josh.PathHandler = function(shell, config) {
    config = config || {};
    var _console = config.console || (Josh.Debug && root.console ? root.console : {
      log: function() {
      }
    });
    var _shell = shell;
    _shell.templates.not_found = _.template("<div><%=cmd%>: <%=path%>: No such file or directory</div>");
    _shell.templates.ls = _.template("<div><% _.each(nodes, function(node) { %><span><%=node.name%>&nbsp;</span><% }); %></div>");
    _shell.templates.pwd = _.template("<div><%=node.path %>&nbsp;</div>");
    _shell.templates.prompt = _.template("<%= node.path %> $");
    var _original_default = _shell.getCommandHandler('_default');
    var self = {
      current: null,
      pathCompletionHandler: pathCompletionHandler,
      commandAndPathCompletionHandler: commandAndPathCompletionHandler,
      getNode: function(path, callback) {
        callback();
      },
      getChildNodes: function(node, callback) {
        callback([]);
      },
      getPrompt: function() {
        return _shell.templates.prompt({node: self.current});
      }
    };

    _shell.setCommandHandler("ls", {
      exec: ls,
      completion: pathCompletionHandler
    });
    _shell.setCommandHandler("pwd", {
      exec: pwd,
      completion: pathCompletionHandler
    });
    _shell.setCommandHandler("cd", {
      exec: cd,
      completion: pathCompletionHandler
    });
    _shell.setCommandHandler("_default", {
      exec: _original_default.exec,
      completion: commandAndPathCompletionHandler
    });
    _shell.onNewPrompt(function(callback) {
      callback(self.getPrompt());
    });

    function commandAndPathCompletionHandler(cmd, arg, line, callback) {
      _console.log("calling command and path completion handler w/ cmd: '"+cmd+"', arg: '"+arg+"'");
      if(!arg) {
        arg = cmd;
      }
      if(arg[0] == '.' || arg[0] == '/') {
        return pathCompletionHandler(cmd, arg, line, callback);
      }
      return _original_default.completion(cmd, arg, line, callback);
    }

    function pathCompletionHandler(cmd, arg, line, callback) {
      _console.log("completing '" + arg + "'");
      if(!arg) {
        _console.log("completing on current");
        return completeChildren(self.current, '', callback);
      }
      if(arg[arg.length - 1] == '/') {
        _console.log("completing children w/o partial");
        return self.getNode(arg, function(node) {
          if(!node) {
            _console.log("no node for path");
            return callback();
          }
          return completeChildren(node, '', callback);
        });
      }
      var partial = "";
      var lastPathSeparator = arg.lastIndexOf("/");
      var parent = arg.substr(0, lastPathSeparator + 1);
      partial = arg.substr(lastPathSeparator + 1);
      if(partial === '..' || partial === '.') {
        return callback({
          completion: '/',
          suggestions: []
        });
      }
      _console.log("completing children via parent '" + parent+"'  w/ partial '"+partial+"'");
      return self.getNode(parent, function(node) {
        if(!node) {
          _console.log("no node for parent path");
          return callback();
        }
        return completeChildren(node, partial, function(completion) {
          if(completion && completion.completion == '' && completion.suggestions.length == 1) {
            return callback({
              completion: '/',
              suggestions: []
            });
          }
          return callback(completion);
        });
      });
    }

    function completeChildren(node, partial, callback) {
      self.getChildNodes(node, function(childNodes) {
        callback(_shell.bestMatch(partial, _.map(childNodes, function(x) {
          return x.name;
        })));
      });
    }

    function cd(cmd, args, callback) {
      self.getNode(args[0], function(node) {
        if(!node) {
          return callback(_shell.templates.not_found({cmd: 'cd', path: args[0]}));
        }
        self.current = node;
        return callback();
      });
    }

    function pwd(cmd, args, callback) {
      callback(_shell.templates.pwd({node: self.current}));
    }

    function ls(cmd, args, callback) {
      _console.log('ls');
      if(!args || !args[0]) {
        return render_ls(self.current, self.current.path, callback);
      }
      return self.getNode(args[0], function(node) {
        render_ls(node, args[0], callback);
      });
    }

    function render_ls(node, path, callback) {
      if(!node) {
        return callback(_shell.templates.not_found({cmd: 'ls', path: path}));
      }
      return self.getChildNodes(node, function(children) {
        _console.log("finish render: " + node.name);
        callback(_shell.templates.ls({nodes: children}));
      });
    }

    return self;
  };
})(this, $, _);
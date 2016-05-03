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
Josh.Version = "0.2.10";
(function(root) {
  Josh.Keys = {
    Special: {
      Backspace: 8,
      Tab: 9,
      Enter: 13,
      Pause: 19,
      CapsLock: 20,
      Escape: 27,
      Space: 32,
      PageUp: 33,
      PageDown: 34,
      End: 35,
      Home: 36,
      Left: 37,
      Up: 38,
      Right: 39,
      Down: 40,
      Insert: 45,
      Delete: 46
    }
  };

  Josh.ReadLine = function(config) {
    config = config || {};

    // instance fields
    var _console = config.console || (Josh.Debug && root.console ? root.console : {
      log: function() {
      }
    });
    var _history = config.history || new Josh.History();
    var _killring = config.killring || new Josh.KillRing();
    var _boundToElement = config.element ? true : false;
    var _element = config.element || root;
    var _active = false;
    var _onActivate;
    var _onDeactivate;
    var _onCompletion;
    var _onEnter;
    var _onChange;
    var _onCancel;
    var _onEOT;
    var _onClear;
    var _onSearchStart;
    var _onSearchEnd;
    var _onSearchChange;
    var _inSearch = false;
    var _searchMatch;
    var _lastSearchText = '';
    var _text = '';
    var _cursor = 0;
    var _lastCmd;
    var _completionActive;
    var _cmdQueue = [];
    var _suspended = false;
    var _cmdMap = {
      complete: cmdComplete,
      done: cmdDone,
      noop: cmdNoOp,
      history_top: cmdHistoryTop,
      history_end: cmdHistoryEnd,
      history_next: cmdHistoryNext,
      history_previous: cmdHistoryPrev,
      end: cmdEnd,
      home: cmdHome,
      left: cmdLeft,
      right: cmdRight,
      cancel: cmdCancel,
      'delete': cmdDeleteChar,
      backspace: cmdBackspace,
      kill_eof: cmdKillToEOF,
      kill_wordback: cmdKillWordBackward,
      kill_wordforward: cmdKillWordForward,
      yank: cmdYank,
      clear: cmdClear,
      search: cmdReverseSearch,
      wordback: cmdBackwardWord,
      wordforward: cmdForwardWord,
      yank_rotate: cmdRotate
  };
    var _keyMap = {
      'default': {
        8: cmdBackspace,    // Backspace
        9: cmdComplete,     // Tab
        13: cmdDone,        // Enter
        27: cmdEsc,         // Esc
        33: cmdHistoryTop,  // Page Up
        34: cmdHistoryEnd,  // Page Down
        35: cmdEnd,         // End
        36: cmdHome,        // Home
        37: cmdLeft,        // Left
        38: cmdHistoryPrev, // Up
        39: cmdRight,       // Right
        40: cmdHistoryNext, // Down
        46: cmdDeleteChar,  // Delete
        10: cmdNoOp,        // Pause
        19: cmdNoOp,        // Caps Lock
        45: cmdNoOp         // Insert
      },
      control: {
        65: cmdHome,          // A
        66: cmdLeft,          // B
        67: cmdCancel,        // C
        68: cmdDeleteChar,    // D
        69: cmdEnd,           // E
        70: cmdRight,         // F
        80: cmdHistoryPrev,   // P
        78: cmdHistoryNext,   // N
        75: cmdKillToEOF,     // K
        89: cmdYank,          // Y
        76: cmdClear,         // L
        82: cmdReverseSearch  // R
      },
      meta: {
        8: cmdKillWordBackward, // Backspace
        66: cmdBackwardWord,    // B
        68: cmdKillWordForward, // D
        70: cmdForwardWord,     // F
        89: cmdRotate           // Y
      }
    };

    // public methods
    var self = {
      isActive: function() {
        return _active;
      },
      activate: function() {
        _active = true;
        if(_onActivate) {
          _onActivate();
        }
      },
      deactivate: function() {
        _active = false;
        if(_onDeactivate) {
          _onDeactivate();
        }
      },
      bind: function(key, action) {
        var k = getKey(key);
        var cmd = _cmdMap[action];
        if(!cmd) {
          return;
        }
        _keyMap[k.modifier][k.code];
      },
      unbind: function(key) {
        var k = getKey(key);
        delete _keyMap[k.modifier][k.code];
      },
      attach: function(el) {
        if(_element) {
          self.detach();
        }
        _console.log("attaching");
        _console.log(el);
        _element = el;
        _boundToElement = true;
        addEvent(_element, "focus", self.activate);
        addEvent(_element, "blur", self.deactivate);
        subscribeToKeys();
      },
      detach: function() {
        removeEvent(_element, "focus", self.activate);
        removeEvent(_element, "blur", self.deactivate);
        _element = null;
        _boundToElement = false;
      },
      onActivate: function(completionHandler) {
        _onActivate = completionHandler;
      },
      onDeactivate: function(completionHandler) {
        _onDeactivate = completionHandler;
      },
      onChange: function(changeHandler) {
        _onChange = changeHandler;
      },
      onClear: function(completionHandler) {
        _onClear = completionHandler;
      },
      onEnter: function(enterHandler) {
        _onEnter = enterHandler;
      },
      onCompletion: function(completionHandler) {
        _onCompletion = completionHandler;
      },
      onCancel: function(completionHandler) {
        _onCancel = completionHandler;
      },
      onEOT: function(completionHandler) {
        _onEOT = completionHandler;
      },
      onSearchStart: function(completionHandler) {
        _onSearchStart = completionHandler;
      },
      onSearchEnd: function(completionHandler) {
        _onSearchEnd = completionHandler;
      },
      onSearchChange: function(completionHandler) {
        _onSearchChange = completionHandler;
      },
      getLine: function() {
        return {
          text: _text,
          cursor: _cursor
        };
      },
      setLine: function(line) {
        _text = line.text;
        _cursor = line.cursor;
        refresh();
      }
    };

    // private methods
    function addEvent(element, name, callback) {
      if(element.addEventListener) {
        element.addEventListener(name, callback, false);
      } else if(element.attachEvent) {
        element.attachEvent('on' + name, callback);
      }
    }

    function removeEvent(element, name, callback) {
      if(element.removeEventListener) {
        element.removeEventListener(name, callback, false);
      } else if(element.detachEvent) {
        element.detachEvent('on' + name, callback);
      }
    }

    function getKeyInfo(e) {
      var code = e.keyCode || e.charCode;
      var c = String.fromCharCode(code);
      return {
        code: code,
        character: c,
        shift: e.shiftKey,
        control: e.controlKey,
        alt: e.altKey,
        isChar: true
      };
    }

    function getKey(key) {
      var k = {
        modifier: 'default',
        code: key.keyCode
      };
      if(key.metaKey || key.altKey) {
        k.modifier = 'meta';
      } else if(key.ctrlKey) {
        k.modifier = 'control';
      }
      if(key['char']) {
        k.code = key['char'].charCodeAt(0);
      }
      return k;
    }

    function queue(cmd) {
      if(_suspended) {
        _cmdQueue.push(cmd);
        return;
      }
      call(cmd);
    }

    function call(cmd) {
      _console.log('calling: ' + cmd.name + ', previous: ' + _lastCmd);
      if(_inSearch && cmd.name != "cmdKeyPress" && cmd.name != "cmdReverseSearch") {
        _inSearch = false;
        if(cmd.name == 'cmdEsc') {
          _searchMatch = null;
        }
        if(_searchMatch) {
          if(_searchMatch.text) {
            _cursor = _searchMatch.cursoridx;
            _text = _searchMatch.text;
            _history.applySearch();
          }
          _searchMatch = null;
        }
        if(_onSearchEnd) {
          _onSearchEnd();
        }
      }
      if(!_inSearch && _killring.isinkill() && cmd.name.substr(0, 7) != 'cmdKill') {
        _killring.commit();
      }
      _lastCmd = cmd.name;
      cmd();
    }

    function suspend(asyncCall) {
      _suspended = true;
      asyncCall(resume);
    }

    function resume() {
      var cmd = _cmdQueue.shift();
      if(!cmd) {
        _suspended = false;
        return;
      }
      call(cmd);
      resume();
    }

    function cmdNoOp() {
      // no-op, used for keys we capture and ignore
    }

    function cmdEsc() {
      // no-op, only has an effect on reverse search and that action was taken in call()
    }

    function cmdBackspace() {
      if(_cursor == 0) {
        return;
      }
      --_cursor;
      _text = remove(_text, _cursor, _cursor + 1);
      refresh();
    }

    function cmdComplete() {
      if(!_onCompletion) {
        return;
      }
      suspend(function(resumeCallback) {
        _onCompletion(self.getLine(), function(completion) {
          if(completion) {
            _text = insert(_text, _cursor, completion);
            updateCursor(_cursor + completion.length);
          }
          _completionActive = true;
          resumeCallback();
        });
      });
    }

    function cmdDone() {
      if(!_text) {
        return;
      }
      var text = _text;
      _history.accept(text);
      _text = '';
      _cursor = 0;
      if(!_onEnter) {
        return;
      }
      suspend(function(resumeCallback) {
        _onEnter(text, function(text) {
          if(text) {
            _text = text;
            _cursor = _text.length;
          }
          if(_onChange) {
            _onChange(self.getLine());
          }
          resumeCallback();
        });
      });

    }

    function cmdEnd() {
      updateCursor(_text.length);
    }

    function cmdHome() {
      updateCursor(0);
    }

    function cmdLeft() {
      if(_cursor == 0) {
        return;
      }
      updateCursor(_cursor - 1);
    }

    function cmdRight() {
      if(_cursor == _text.length) {
        return;
      }
      updateCursor(_cursor + 1);
    }

    function cmdBackwardWord() {
      if(_cursor == 0) {
        return;
      }
      updateCursor(findBeginningOfPreviousWord());
    }

    function cmdForwardWord() {
      if(_cursor == _text.length) {
        return;
      }
      updateCursor(findEndOfCurrentWord());
    }

    function cmdHistoryPrev() {
      if(!_history.hasPrev()) {
        return;
      }
      getHistory(_history.prev);
    }

    function cmdHistoryNext() {
      if(!_history.hasNext()) {
        return;
      }
      getHistory(_history.next);
    }

    function cmdHistoryTop() {
      getHistory(_history.top);
    }

    function cmdHistoryEnd() {
      getHistory(_history.end);
    }

    function cmdDeleteChar() {
      if(_text.length == 0) {
        if(_onEOT) {
          _onEOT();
          return;
        }
      }
      if(_cursor == _text.length) {
        return;
      }
      _text = remove(_text, _cursor, _cursor + 1);
      refresh();
    }

    function cmdCancel() {
      if(_onCancel) {
        _onCancel();
      }
    }

    function cmdKillToEOF() {
      _killring.append(_text.substr(_cursor));
      _text = _text.substr(0, _cursor);
      refresh();
    }

    function cmdKillWordForward() {
      if(_text.length == 0) {
        return;
      }
      if(_cursor == _text.length) {
        return;
      }
      var end = findEndOfCurrentWord();
      if(end == _text.length - 1) {
        return cmdKillToEOF();
      }
      _killring.append(_text.substring(_cursor, end))
      _text = remove(_text, _cursor, end);
      refresh();
    }

    function cmdKillWordBackward() {
      if(_cursor == 0) {
        return;
      }
      var oldCursor = _cursor;
      _cursor = findBeginningOfPreviousWord();
      _killring.prepend(_text.substring(_cursor, oldCursor));
      _text = remove(_text, _cursor, oldCursor);
      refresh();
    }

    function cmdYank() {
      var yank = _killring.yank();
      if(!yank) {
        return;
      }
      _text = insert(_text, _cursor, yank);
      updateCursor(_cursor + yank.length);
    }

    function cmdRotate() {
      var lastyanklength = _killring.lastyanklength();
      if(!lastyanklength) {
        return;
      }
      var yank = _killring.rotate();
      if(!yank) {
        return;
      }
      var oldCursor = _cursor;
      _cursor = _cursor - lastyanklength;
      _text = remove(_text, _cursor, oldCursor);
      _text = insert(_text, _cursor, yank);
      updateCursor(_cursor + yank.length);
    }

    function cmdClear() {
      if(_onClear) {
        _onClear();
      } else {
        refresh();
      }
    }

    function cmdReverseSearch() {
      if(!_inSearch) {
        _inSearch = true;
        if(_onSearchStart) {
          _onSearchStart();
        }
        if(_onSearchChange) {
          _onSearchChange({});
        }
      } else {
        if(!_searchMatch) {
          _searchMatch = {term: ''};
        }
        search();
      }
    }

    function updateCursor(position) {
      _cursor = position;
      refresh();
    }

    function addText(c) {
      _text = insert(_text, _cursor, c);
      ++_cursor;
      refresh();
    }

    function addSearchText(c) {
      if(!_searchMatch) {
        _searchMatch = {term: ''};
      }
      _searchMatch.term += c;
      search();
    }

    function search() {
      _console.log("searchtext: " + _searchMatch.term);
      var match = _history.search(_searchMatch.term);
      if(match != null) {
        _searchMatch = match;
        _console.log("match: " + match);
        if(_onSearchChange) {
          _onSearchChange(match);
        }
      }
    }

    function refresh() {
      if(_onChange) {
        _onChange(self.getLine());
      }
    }

    function getHistory(historyCall) {
      _history.update(_text);
      _text = historyCall();
      updateCursor(_text.length);
    }

    function findBeginningOfPreviousWord() {
      var position = _cursor - 1;
      if(position < 0) {
        return 0;
      }
      var word = false;
      for(var i = position; i > 0; i--) {
        var word2 = isWordChar(_text[i]);
        if(word && !word2) {
          return i + 1;
        }
        word = word2;
      }
      return 0;
    }

    function findEndOfCurrentWord() {
      if(_text.length == 0) {
        return 0;
      }
      var position = _cursor + 1;
      if(position >= _text.length) {
        return _text.length - 1;
      }
      var word = false;
      for(var i = position; i < _text.length; i++) {
        var word2 = isWordChar(_text[i]);
        if(word && !word2) {
          return i;
        }
        word = word2;
      }
      return _text.length - 1;
    }

    function isWordChar(c) {
      if(c == undefined) {
        return false;
      }
      var code = c.charCodeAt(0);
      return (code >= 48 && code <= 57)
        || (code >= 65 && code <= 90)
        || (code >= 97 && code <= 122);
    }

    function remove(text, from, to) {
      if(text.length <= 1 || text.length <= to - from) {
        return '';
      }
      if(from == 0) {

        // delete leading characters
        return text.substr(to);
      }
      var left = text.substr(0, from);
      var right = text.substr(to);
      return left + right;
    }

    function insert(text, idx, ins) {
      if(idx == 0) {
        return ins + text;
      }
      if(idx >= text.length) {
        return text + ins;
      }
      var left = text.substr(0, idx);
      var right = text.substr(idx);
      return left + ins + right;
    }

    function subscribeToKeys() {

      // set up key capture
      _element.onkeydown = function(e) {
        e = e || window.event;

        // return as unhandled if we're not active or the key is just a modifier key
        if(!_active || e.keyCode == 16 || e.keyCode == 17 || e.keyCode == 18 || e.keyCode == 91) {
          return true;
        }

        // check for some special first keys, regardless of modifiers
        _console.log("key: " + e.keyCode);
        var cmd = _keyMap['default'][e.keyCode];
        // intercept ctrl- and meta- sequences (may override the non-modifier cmd captured above
        var mod;
        if(e.ctrlKey && !e.shiftKey && !e.altKey && !e.metaKey) {
          mod = _keyMap.control[e.keyCode];
          if(mod) {
            cmd = mod;
          }
        } else if((e.altKey || e.metaKey) && !e.ctrlKey && !e.shiftKey) {
          mod = _keyMap.meta[e.keyCode];
          if(mod) {
            cmd = mod;
          }
        }
        if(!cmd) {
          return true;
        }
        queue(cmd);
        e.preventDefault();
        e.stopPropagation();
        e.cancelBubble = true;
        return false;
      };

      _element.onkeypress = function(e) {
        if(!_active) {
          return true;
        }
        var key = getKeyInfo(e);
        if(key.code == 0 || e.defaultPrevented || e.metaKey || e.altKey || e.ctrlKey) {
          return false;
        }
        queue(function cmdKeyPress() {
          if(_inSearch) {
            addSearchText(key.character);
          } else {
            addText(key.character);
          }
        });
        e.preventDefault();
        e.stopPropagation();
        e.cancelBubble = true;
        return false;
      };
    }
    if(_boundToElement) {
      self.attach(_element);
    } else {
      subscribeToKeys();
    }
    return self;
  };
})(this);

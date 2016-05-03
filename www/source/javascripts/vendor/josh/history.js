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
(function (root) {
  Josh.History = function (config) {
    config = config || {};

    var _console = Josh.Debug && root.console ? root.console : {log: function() {}};
    var _history = config.history || [''];
    var _cursor = config.cursor || 0;
    var _searchCursor = _cursor;
    var _lastSearchTerm = '';
    var _storage = config.storage || root.localStorage;
    var _key = config.key || 'josh.history';

    if (_storage) {
      try {
        var data = _storage.getItem(_key);
      } catch(e) {
        _console.log("Error accessing storage");
      }
      if (data) {
        _history = JSON.parse(data);
        _searchCursor = _cursor = _history.length - 1;
      } else {
        save();
      }
    }
    function save() {
      if (_storage) {
        try {
          _storage.setItem(_key, JSON.stringify(_history));
        } catch(e) {
          _console.log("Error accessing storage");
        }
      }
    }

    function setHistory() {
      _searchCursor = _cursor;
      _lastSearchTerm = '';
      return _history[_cursor];
    }

    return {
      update:function (text) {
        _console.log("updating history to " + text);
        _history[_cursor] = text;
        save();
      },
      accept:function (text) {
        _console.log("accepting history " + text);
        var last = _history.length - 1;
        if (text) {
          if (_cursor == last) {
            _console.log("we're at the end already, update last position");
            _history[_cursor] = text;
          } else if (!_history[last]) {
            _console.log("we're not at the end, but the end was blank, so update last position");
            _history[last] = text;
          } else {
            _console.log("appending to end");
            _history.push(text);
          }
          _history.push('');
        }
        _searchCursor = _cursor = _history.length - 1;
        save();
      },
      items:function () {
        return _history.slice(0, _history.length - 1);
      },
      clear:function () {
        _history = [_history[_history.length - 1]];
        save();
      },
      hasNext:function () {
        return _cursor < (_history.length - 1);
      },
      hasPrev:function () {
        return _cursor > 0;
      },
      prev:function () {
        --_cursor;
        return setHistory();
      },
      next:function () {
        ++_cursor;
        return setHistory();
      },
      top:function () {
        _cursor = 0;
        return setHistory();
      },
      end:function () {
        _cursor = _history.length - 1;
        return setHistory();
      },
      search:function (term) {
        if (!term && !_lastSearchTerm) {
          return null;
        }
        var iterations = _history.length;
        if (term == _lastSearchTerm) {
          _searchCursor--;
          iterations--;
        }
        if (!term) {
          term = _lastSearchTerm;
        }
        _lastSearchTerm = term;
        for (var i = 0; i < iterations; i++) {
          if (_searchCursor < 0) {
            _searchCursor = _history.length - 1;
          }
          var idx = _history[_searchCursor].indexOf(term);
          if (idx != -1) {
            return {
              text:_history[_searchCursor],
              cursoridx:idx,
              term:term
            };
          }
          _searchCursor--;
        }
        return null;
      },
      applySearch:function () {
        if (_lastSearchTerm) {
          _console.log("setting history to position" + _searchCursor + "(" + _cursor + "): " + _history[_searchCursor]);
          _cursor = _searchCursor;
          return _history[_cursor];
        }
        return null;
      }
    };
  };
})(this);


var Josh = Josh || {};
(function(root) {
  Josh.KillRing = function(config) {
    config = config || {};

    var _console = Josh.Debug && root.console ? root.console : {log: function() {
    }};
    var _ring = config.ring || [];
    var _cursor = config.cursor || 0;
    var _uncommitted = false;
    var _yanking = false;
    if(_ring.length == 0) {
      _cursor = -1;
    } else if(_cursor >= _ring.length) {
      _cursor = _ring.length - 1;
    }
    var self = {
      isinkill: function() {
        return _uncommitted;
      },
      lastyanklength: function() {
        if(!_yanking) {
          return 0;
        }
        return _ring[_cursor].length;
      },
      append: function(value) {
        _yanking = false;
        if(!value) {
          return;
        }
        if(_ring.length == 0 || !_uncommitted) {
          _ring.push('');
        }
        _cursor = _ring.length - 1;
        _console.log("appending: " + value);
        _uncommitted = true;
        _ring[_cursor] += value;
      },
      prepend: function(value) {
        _yanking = false;
        if(!value) {
          return;
        }
        if(_ring.length == 0 || !_uncommitted) {
          _ring.push('');
        }
        _cursor = _ring.length - 1;
        _console.log("prepending: " + value);
        _uncommitted = true;
        _ring[_cursor] = value + _ring[_cursor];
      },
      commit: function() {
        _console.log("committing");
        _yanking = false;
        _uncommitted = false;
      },
      yank: function() {
        self.commit();
        if(_ring.length == 0) {
          return null;
        }
        _yanking = true;
        return _ring[_cursor];
      },
      rotate: function() {
        if(!_yanking || _ring.length == 0) {
          return null;
        }
        --_cursor;
        if(_cursor < 0) {
          _cursor = _ring.length - 1;
        }
        return self.yank();
      },
      items: function() {
        return _ring.slice(0);
      },
      clear: function() {
        _ring = [];
        _cursor = -1;
        _yanking = false;
        _uncommited = false;
      }
    };
    return self;
  }
})(this);
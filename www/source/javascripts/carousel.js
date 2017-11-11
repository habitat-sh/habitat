function Carousel($slides, $nav, activeClass, intervalLength, smallBreakpoint) {
  this.dom = {
    slideParent: $slides.parent(),
    slides: $slides,
    navParent: $nav.parent(),
    nav: $nav,
    window: $(window)
  }
  this.current = 0;
  this.len = this.dom.slides.length;

  this.smallBreakpoint = smallBreakpoint || 640;
  this.activeClass = activeClass || 'is-active';
  this.intervalLength = intervalLength || 10000;

  this.dom.window.resize(this.resizeParent.bind(this));

  this.timeout;
  this.init();
}

Carousel.prototype.init = function() {
  $(this.dom.nav).each(function(idx, el) {
    $(el).attr('data-idx', idx).on('click', this.rotate.bind(this, idx));
  }.bind(this));

  $(this.dom.slides).mouseenter(this.stop.bind(this)).mouseleave(this.start.bind(this));

  this.resizeParent();
  this.rotate(0, true);
}

Carousel.prototype.start = function(delay) {
  this.stop();
  this.timeout = setTimeout(this.rotate.bind(this), delay || 1)
}

Carousel.prototype.stop = function() {
  clearTimeout(this.timeout);
}

Carousel.prototype.rotate = function(target, initialRotate) {
  $(this.dom.slides[this.current]).removeClass(this.activeClass);
  $(this.dom.nav[this.current]).removeClass(this.activeClass);

  var next = target !== undefined ? target : this.current + 1 === this.len ? 0 : this.current + 1;

  $(this.dom.slides[next]).addClass(this.activeClass);
  $(this.dom.nav[next]).addClass(this.activeClass);

  this.dom.slideParent.attr('data-currentIdx', next);
  this.dom.navParent.attr('data-currentIdx', next);
  this.current = next;

  this.start(target !== undefined && initialRotate !== true ? this.intervalLength * 6 : this.intervalLength);
}

Carousel.prototype.resizeParent = function() {
  var height = 0;

  this.dom.slides.each(function() {
    if ($(this).height() > height) {
      height = $(this).height();
    }
  })
  this.dom.slideParent.height(height);
  this.dom.slides.height(height);
}
function Carousel($slides, $nav, activeClass, intervalLength) {
  this.dom = {
    slideParent: $slides.parent(),
    slides: $slides,
    navParent: $nav.parent(),
    nav: $nav
  }
  this.current = 0;
  this.len = this.dom.slides.length;

  this.activeClass = activeClass || 'is-active';
  this.intervalLength = intervalLength || 10000;

  $(window).resize(this.resizeParent.bind(this));

  this.interval;
  this.init();
}

Carousel.prototype.init = function() {
  $(this.dom.nav).each(function(idx, el) {
    $(el).attr('data-idx', idx).on('click', this.rotate.bind(this, idx));
  }.bind(this));

  this.resizeParent();
  this.rotate(0);
}

Carousel.prototype.startInterval = function() {
  clearInterval(this.interval);
  this.interval = setInterval(this.rotate.bind(this), this.intervalLength);
}

Carousel.prototype.rotate = function(target) {
  if (target !== undefined) {
    this.startInterval();
  }

  $(this.dom.slides[this.current]).removeClass(this.activeClass);
  $(this.dom.nav[this.current]).removeClass(this.activeClass);

  var next = target !== undefined ? target : this.current + 1 === this.len ? 0 : this.current + 1;

  $(this.dom.slides[next]).addClass(this.activeClass);
  $(this.dom.nav[next]).addClass(this.activeClass);

  this.dom.slideParent.attr('data-currentIdx', next);
  this.dom.navParent.attr('data-currentIdx', next);
  this.current = next;
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
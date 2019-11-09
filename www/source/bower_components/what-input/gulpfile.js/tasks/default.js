var gulp = require('gulp');
var browserSync = require('browser-sync').create();

gulp.task('default', ['clean', 'scripts'], function() {
  browserSync.init({
    server: {
      baseDir: './'
    }
  });

  gulp.watch(['./what-input.js', './polyfills/*.js'], ['scripts']).on('change', browserSync.reload);
});

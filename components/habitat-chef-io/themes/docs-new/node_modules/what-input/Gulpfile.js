/*
 * load plugins
 */

const pkg = require('./package.json')

const banner = [
  '/**',
  ' * <%= pkg.name %> - <%= pkg.description %>',
  ' * @version v<%= pkg.version %>',
  ' * @link <%= pkg.homepage %>',
  ' * @license <%= pkg.license %>',
  ' */',
  ''
].join('\n')

// gulp
const gulp = require('gulp')

// load all plugins in "devDependencies" into the letiable $
const $ = require('gulp-load-plugins')({
  pattern: ['*'],
  scope: ['devDependencies']
})

/*
 * clean task
 */

gulp.task('clean', () => {
  return $.del(['**/.DS_Store', './build/*', './dist/*'])
})

/*
 * scripts tasks
 */

gulp.task('scripts', () => {
  return gulp
    .src(['./src/scripts/what-input.js'])
    .pipe($.standard())
    .pipe(
      $.standard.reporter('default', {
        breakOnError: false,
        quiet: true
      })
    )
    .pipe(
      $.webpackStream({
        module: {
          loaders: [
            {
              test: /.jsx?$/,
              loader: 'babel-loader',
              exclude: /node_modules/,
              query: {
                presets: ['env']
              }
            }
          ]
        },
        output: {
          chunkFilename: '[name].js',
          library: 'whatInput',
          libraryTarget: 'umd',
          umdNamedDefine: true
        }
      })
    )
    .pipe($.rename('what-input.js'))
    .pipe($.header(banner, { pkg: pkg }))
    .pipe(gulp.dest('./dist/'))
    .pipe(gulp.dest('./build/scripts/'))
    .pipe($.sourcemaps.init())
    .pipe($.uglify())
    .pipe(
      $.rename({
        suffix: '.min'
      })
    )
    .pipe($.header(banner, { pkg: pkg }))
    .pipe($.sourcemaps.write('./maps'))
    .pipe(gulp.dest('./dist/'))
    .pipe($.notify('Build complete'))
})

/*
 * stylesheets
 */

gulp.task('styles', () => {
  let processors = [
    $.autoprefixer({
      browsers: ['last 3 versions', '> 1%', 'ie >= 10']
    }),
    $.cssMqpacker({
      sort: true
    })
  ]

  return gulp
    .src(['./src/styles/index.scss'])
    .pipe(
      $.plumber({
        errorHandler: $.notify.onError('Error: <%= error.message %>')
      })
    )
    .pipe($.sourcemaps.init())
    .pipe($.sassGlob())
    .pipe($.sass())
    .pipe($.postcss(processors))
    .pipe(
      $.cssnano({
        minifySelectors: false,
        reduceIdents: false,
        zindex: false
      })
    )
    .pipe($.sourcemaps.write('maps'))
    .pipe(gulp.dest('./build/styles'))
    .pipe($.browserSync.stream())
    .pipe($.notify('Styles task complete'))
})

/*
 * images task
 */

gulp.task('images', () => {
  return gulp.src(['./src/images/**/*']).pipe(gulp.dest('./build/images'))
})

/*
 * markup task
 */

gulp.task('markup', () => {
  return gulp.src(['./src/markup/*']).pipe(gulp.dest('./build'))
})

/*
 * deploy task
 */

gulp.task('deploy', () => {
  return gulp.src('./build/**/*').pipe($.ghPages())
})

/*
 * default task
 */

gulp.task('default', () => {
  $.runSequence('clean', ['markup', 'scripts', 'styles', 'images'], () => {
    $.browserSync.init({
      server: {
        baseDir: './build/'
      }
    })

    gulp
      .watch(
        ['./src/scripts/what-input.js', './src/scripts/polyfills/*.js'],
        ['scripts']
      )
      .on('change', $.browserSync.reload)

    gulp.watch(['./src/styles/{,*/}{,*/}*.scss'], ['styles'])

    gulp
      .watch(['./src/markup/*.html'], ['markup'])
      .on('change', $.browserSync.reload)
  })
})

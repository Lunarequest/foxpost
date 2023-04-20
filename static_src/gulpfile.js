const gulp = require('gulp');
const concat = require('gulp-concat');
var postcss = require('gulp-postcss');
var autoprefixer = require('autoprefixer');
var cssnano = require('cssnano');

gulp.task('bundle-css', () => {
  var plugins = [
    autoprefixer(),
    cssnano()
  ];
  return gulp.src('src/css/*.css')
    .pipe(concat('bundle.css'))
    .pipe(postcss(plugins))
    .pipe(gulp.dest('../static/css'));
});

gulp.task('default', gulp.parallel('bundle-css'));

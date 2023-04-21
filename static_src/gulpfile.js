const gulp = require('gulp');
const concat = require('gulp-concat');
var postcss = require('gulp-postcss');
var autoprefixer = require('autoprefixer');
var cssnano = require('cssnano');
const swc = require('gulp-swc');

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

gulp.task('transpile-ts', () => {
  return gulp.src('src/js/*.ts')
        .pipe(swc())
        .pipe(gulp.dest('../static/js'))
})

gulp.task('default', gulp.parallel('bundle-css','transpile-ts'));

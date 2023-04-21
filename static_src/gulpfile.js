const gulp = require('gulp');
const concat = require('gulp-concat');
const postcss = require('gulp-postcss');
const autoprefixer = require('autoprefixer');
const cssnano = require('cssnano');
const ts = require('gulp-typescript');
const terser = require('terser');
const gulpTerser = require('gulp-terser');

gulp.task('bundle-css', () => {
  const plugins = [
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
    .pipe(ts({
      target: 'ES6',
      lib: [
        "ES6",
        "dom"
      ],
      module: "commonjs",
    }))
    .pipe(gulpTerser({},terser.minify))
    .pipe(gulp.dest('../static/js'))
})

gulp.task('default', gulp.parallel('bundle-css', 'transpile-ts'));

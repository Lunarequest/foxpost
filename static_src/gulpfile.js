const gulp = require('gulp');
const concat = require('gulp-concat');
const postcss = require('gulp-postcss');
const autoprefixer = require('autoprefixer');
const cssnano = require('cssnano');
const terser = require('terser');
const gulpTerser = require('gulp-terser');
const gulpEsbuild = require('gulp-esbuild');
const tailwindcss = require('tailwindcss');

gulp.task('bundle-css', () => {
  const plugins = [
    tailwindcss(),
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
    .pipe(gulpEsbuild({
        bundle: true,
        minify: true
    }))
    .pipe(gulpTerser({},terser.minify))
    .pipe(gulp.dest('../static/js'))
})

gulp.task('default', gulp.parallel('bundle-css', 'transpile-ts'));

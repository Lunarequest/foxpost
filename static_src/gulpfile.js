const gulp = require("gulp");
const concat = require("gulp-concat");
const postcss = require("gulp-postcss");
const autoprefixer = require("autoprefixer");
const cssnano = require("cssnano");
const terser = require("terser");
const gulpTerser = require("gulp-terser");
const gulpEsbuild = require("gulp-esbuild");
const tailwindcss = require("tailwindcss");
const sass = require("gulp-sass")(require("sass"));
const production = process.env.NODE_ENV === "production";

const plugins = [autoprefixer(), cssnano()];

gulp.task("build-tailwind", () => {
  plugins.unshift(tailwindcss("./tailwind.config.js"));
  return gulp
    .src("src/css/*.scss")
    .pipe(sass().on("error", sass.logError))
    .pipe(
      postcss(plugins),
    )
    .pipe(gulp.dest("src/css/"));
});

gulp.task("bundle-css", () => {
  return gulp
    .src("src/css/*.css")
    .pipe(concat("bundle.css"))
    .pipe(postcss(plugins))
    .pipe(gulp.dest("../static/css"));
});

gulp.task("transpile-ts", () => {
  return gulp
    .src("src/js/*.ts")
    .pipe(
      gulpEsbuild({
        bundle: true,
        minify: production,
      }),
    )
    .pipe(gulpTerser({}, terser.minify))
    .pipe(gulp.dest("../static/js"));
});

gulp.task(
  "default",
  gulp.parallel("transpile-ts", gulp.series("build-tailwind", "bundle-css")),
);

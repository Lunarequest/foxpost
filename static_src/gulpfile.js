const gulp = require("gulp");
const postcss = require("gulp-postcss");
const autoprefixer = require("autoprefixer");
const cssnano = require("cssnano");
const gulpEsbuild = require("gulp-esbuild");
const tailwindcss = require("tailwindcss");
const sass = require("gulp-sass")(require("sass"));
const { wasmLoader } = require("esbuild-plugin-wasm");

const production = process.env.NODE_ENV === "production";

const plugins = [
	tailwindcss("./tailwind.config.js"),
	autoprefixer(),
	cssnano(),
];

gulp.task("build-css", () => {
	return gulp
		.src("src/css/bundle.scss")
		.pipe(sass().on("error", sass.logError))
		.pipe(postcss(plugins))
		.pipe(gulp.dest("../static/css/"));
});

gulp.task("transpile-ts", () => {
	return gulp
		.src("src/js/*.ts")
		.pipe(
			gulpEsbuild({
				target: "esnext",
				format: "esm",
				bundle: true,
				minify: production,
				plugins: [wasmLoader({ mode: "embedded" })],
			}),
		)
		.pipe(gulp.dest("../static/js"));
});

gulp.task("default", gulp.parallel("transpile-ts", "build-css"));

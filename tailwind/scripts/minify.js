const { minify } = require('terser');
const { opendir, readFile, writeFile } = require('fs/promises');
const postcss = require('postcss');
const cssnano = require('cssnano');
const autoprefixer = require('autoprefixer');

async function jsminify() {
  const dir = await opendir("../static/js");
  let dirent;
  for await (dirent of dir) {
    if (dirent.isFile()) {
      let code = await readFile(`../static/js/${dirent.name}`);
      let result = await minify(code.toString('utf-8'));
      await writeFile(`../static/js/${dirent.name}`, result.code);
    }
  }
}

async function cssminify() {
  const dir = await opendir("../static/css");
  let dirent;
  for await (dirent of dir) {
    if (dirent.isFile()) {
      let file = `../static/css/${dirent.name}`;
      let code = await readFile(file);
      let result = await postcss([cssnano, autoprefixer]).process(code, { from: file });
      await writeFile(file, result.css);
      if (result.map) {
        writeFile(
          `${file}.map`, result.map.toString(),
        )
      }
    }
  }
}



jsminify();
cssminify();

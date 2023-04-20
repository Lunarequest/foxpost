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


jsminify();

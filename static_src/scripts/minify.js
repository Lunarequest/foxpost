const { minify } = require("terser");
const { opendir, readFile, writeFile } = require("fs/promises");

async function jsminify() {
	const dir = await opendir("../static/js");
	let dirent;
	for await (dirent of dir) {
		if (dirent.isFile()) {
			const code = await readFile(`../static/js/${dirent.name}`);
			const result = await minify(code.toString("utf-8"));
			await writeFile(`../static/js/${dirent.name}`, result.code);
		}
	}
}

jsminify();

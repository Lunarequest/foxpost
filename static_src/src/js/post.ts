import hljs from "highlight.js";
import mermaid from "mermaid";
import { render_webmentions } from "webmention-io";

mermaid.initialize({
	startOnLoad: true,
	theme: "dark",
});

hljs.highlightAll();

window.onload = function () {
	init();
};

function init() {
	const webmentions_div = document.getElementById("webmentions");
	console.log("loading webmetions");
	if (webmentions_div) {
		const page_url = webmentions_div.getAttribute("data-page-url");
		if (page_url) {
			render_webmentions(page_url)
				.then((webmentions: string) => {
					webmentions_div.innerHTML = webmentions;
				})
				.catch((error: String) => {
					if (error !== "Error: there are no replys") {
						console.log(error);
					}
				});
		}
	}
}

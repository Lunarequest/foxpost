import hljs from "highlight.js";
import mermaid from "mermaid";

// run functions on loading of js
hljs.highlightAll();
mermaid.initialize({
	startOnLoad: true,
	theme: "dark",
});

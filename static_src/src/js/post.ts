import hljs from "highlight.js";
import mermaid from "mermaid";
import DOMPurify from "dompurify";

type User = {
	id: string,
	name: string,
	username: string,
	host: string | null,
	avatarUrl: string,
	avatarBlurhash: string,
	avatarColor: string | null,
	isAdmin: boolean,
	speakAsCat: boolean,
	emojis: Array<string>,
	onlineStatus: string,
	driveCapacityOverrideMb: string | null
}

type CalckeyEntry =
	{
		id: string,
		createdAt: string,
		userId: string,
		user: User,
		text: string,
		cw: string | null,
		visibility: string,
		renoteCount: number,
		repliesCount: number,
		emojis: Array<String>,
		fileIds: [],
		files: [],
		replyId: string,
		renoteId: string | null,
		url: string,
	};

async function init() {
	const webmentions_div = document.getElementById("webmentions");
	if (webmentions_div) {
		console.log("loading comments");
		const noteid = webmentions_div.getAttribute("data-noteid");
		if (noteid) {
			let payload = {
				"noteId": noteid,
				"limit": 100
			}

			let output = ""

			let resp = await fetch(`https://social.nullrequest.com/api/notes/replies`, {
				method: "POST",
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(payload)
			});

			const content: Array<CalckeyEntry> = await resp.json();

			content.forEach((entry) => {
				let host;
				if (entry.user.host) {
					host = entry.user.host
				} else {
					host = "https://social.nullrequest.com";
				}
				let entry_html = `"<li><a class="reaction" rel="nofollow ugc" title="${entry.user.name} replied" href="${host}"><img src="${entry.user.avatarUrl}" loading="lazy" decoding="async" alt="${entry.user.name}'s avatar"><a>💬 <a/></a><a class="source" rel="nofollow ugc" href="${entry.url}">@${entry.user.name}@${host}</a> ${entry.text}</li>",`
				output = output + `<ul>${entry_html}</ul>`;
			});

			const purify = DOMPurify(window);
			webmentions_div.innerHTML = purify.sanitize(`<li>${output}</li>`);
		}
	}
}

// run functions on loading of js
mermaid.initialize({
	startOnLoad: true,
	theme: "dark",
});

hljs.highlightAll();
init();

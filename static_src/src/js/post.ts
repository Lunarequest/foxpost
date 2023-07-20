import hljs from "highlight.js";
import mermaid from "mermaid";
import DOMPurify from "dompurify";

const HOST = "https://social.nullrequest.com/";
const re = /@[A-Za-z0-9_.]+@(((?!-))(xn--|_)?[a-z0-9-]{0,61}[a-z0-9]{1,1}\.)*(xn--)?([a-z0-9][a-z0-9\-]{0,60}|[a-z0-9-]{1,30}\.[a-z]{2,})/g;

type Emoji = {
	id: string,
	url: string;
	name: string;
};

type User = {
	username: string,
	avatarUrl: string,
	emojis: Emoji[]
}

type UserDetails = {
	host: string | null
}

type Reply = {
	id: string,
	text: string,
	user: User,
	emojis: Emoji[],
	createdAt: string,
}

function escapeHtml(unsafe: string) {
	return unsafe
		.replace(/&/g, "&amp;")
		.replace(/</g, "&lt;")
		.replace(/>/g, "&gt;")
		.replace(/"/g, "&quot;")
		.replace(/'/g, "&#039;");
}

function init() {
	const comments = document.getElementById("comments");
	console.log("loading comments")
	if (comments) {
		const id = comments.getAttribute("data-noteid");
		if (comments) {
			comments.innerHTML = "Loading";
			const data = {
				noteId: id,
				limit: 100,
				depth: 100
			};
			fetch("https://social.nullrequest.com/api/notes/children", {
				method: "POST",
				headers: {
					Accept: "application/json",
					"Content-Type": "application/json",
				},
				body: JSON.stringify(data),
			})
				.then(function (response) {
					return response.json();
				})
				.then(function (data: Reply[]) {
					if (
						data &&
						Array.isArray(data) &&
						data.length > 0
					) {
						if (comments) {
							data.sort((a, b) => Date.parse(a.createdAt) - Date.parse(b.createdAt));
							data.reverse();
							comments.innerHTML = "";
							data.forEach(async function (reply) {
								reply.user.username = escapeHtml(
									reply.user.username,
								);

								reply.text = escapeHtml(reply.text);

								const array = [...reply.text.matchAll(re)];


								reply.emojis.forEach((emoji: Emoji) => {
									reply.text =
										reply.text.replace(
											`:${emoji.name}:`,
											`<img src="${escapeHtml(
												emoji.url,
											)}" alt="Emoji ${emoji.name}" height="20" width="20" />`,
										);
								});

								reply.user.emojis.forEach((emoji: Emoji) => {
									reply.user.username =
										reply.user.username.replace(
											`:${emoji.name}:`,
											`<img src="${escapeHtml(
												emoji.url,
											)}" alt="Emoji ${emoji.name}" height="20" width="20" />`,
										);
								});

								if (array.length > 0) {
									let fedi_username = array[0][0];
									let split = fedi_username.split("@")
									let username = split[1];
									let domain = split[2];
									let ref = `https://${domain}/@${username}`;

									reply.text = reply.text.replace(array[0][0], `<a class="MastodonUsername" href=${ref}>@${username}</a>`);
								}

								let payload = {
									"query": reply.user.username,
									"offset": 0,
									"limit": 1,
									"origin": "combined",
									"detail": false
								};

								let userdetails = await fetch(`${HOST}api/users/search`, {
									method: "POST",
									headers: {
										Accept: "application/json",
										"Content-Type": "application/json",
									},
									body: JSON.stringify(payload),
								});
								let json: UserDetails[] = await userdetails.json();
								let user = json[0];
								let host;
								
								if (user.host) {
									host = user.host;
								} else {
									host = HOST.replace("https://", "").replace("/", "");
								}

								const mastodonComment = `<div class="mastodon-comment">
					   <div class="avatar">
						 <img src="${escapeHtml(reply.user.avatarUrl)}" height=60 width=60 alt="">
					   </div>
					   <div class="content">
						 <div class="author">
						   <a href="${HOST}/@${reply.user.username}" rel="nofollow">
							 <span>${reply.user.username}</span>
							 <span class="MastoHost">${host}</span>
						   </a>
						   <a class="date" href="${HOST}/notes/${reply.id}" rel="nofollow">
							 ${reply.createdAt.substr(0, 10)}
							</a>
						 </div>
						 <a class="date" href="${HOST}/notes/${reply.id}" rel="nofollow">
						 <div class="mastodon-comment-content">${reply.text}</div> 
						 </a>
					   </div>

					 </div>`;
								if (comments) {
									comments.appendChild(
										DOMPurify.sanitize(mastodonComment, {
											RETURN_DOM_FRAGMENT: true,
										}),
									);
								}
							});
						} else {
							const comments = document.getElementById("comments");
							if (comments) {
								comments.innerHTML = "<p>Not comments found</p>";
							}
						}
					}
				});
		}
	};
}

// run functions on loading of js
hljs.highlightAll();
mermaid.initialize({
	startOnLoad: true,
	theme: "dark",
});
init();

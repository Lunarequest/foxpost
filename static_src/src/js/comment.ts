import DOMPurify from "dompurify";

const HOST = "https://social.nullrequest.com/";
const re =
	/@[A-Za-z0-9_.]+@(((?!-))(xn--|_)?[a-z0-9-]{0,61}[a-z0-9]{1,1}\.)*(xn--)?([a-z0-9][a-z0-9\-]{0,60}|[a-z0-9-]{1,30}\.[a-z]{2,})/g;

type Emoji = {
	id: string;
	url: string;
	name: string;
};

type User = {
	username: string;
	name: string;
	avatarUrl: string;
	emojis: Emoji[];
};

type UserDetails = {
	host: string | null;
};

type Reply = {
	id: string;
	text: string;
	user: User;
	emojis: Emoji[];
	createdAt: string;
	cw: null | string;
};

function escapeHtml(unsafe: string): string {
	return unsafe
		.replace(/&/g, "&amp;")
		.replace(/</g, "&lt;")
		.replace(/>/g, "&gt;")
		.replace(/"/g, "&quot;")
		.replace(/'/g, "&#039;");
}

function replace_emoji(input: string, emojis: Emoji[]): string {
	let output = input;
	emojis.forEach((emoji: Emoji) => {
		output = output.replace(
			`:${emoji.name}:`,
			`<img class="emoji" src="${escapeHtml(emoji.url)}" alt="Emoji ${
				emoji.name
			}" height="20" width="20" />`,
		);
	});
	return output;
}

async function init() {
	const comments = document.getElementById("comments");
	console.log("loading comments");
	if (comments) {
		const id = comments.getAttribute("data-noteid");
		const output: {
			html: DocumentFragment;
			date: string;
		}[] = [];
		const payload = {
			noteId: id,
			limit: 100,
			depth: 100,
		};

		const parent_resp = await fetch(
			"https://social.nullrequest.com/api/notes/show",
			{
				method: "POST",
				headers: {
					Accept: "application/json",
					"Content-Type": "application/json",
				},
				body: JSON.stringify(payload),
			},
		);

		const parent_reply: Reply = await parent_resp.json();
		const host = HOST.replace("https://", "").replace("/", "");
		let mastodonComment;
		if (parent_reply.cw) {
			mastodonComment = `<div class="mastodon-comment">
			<div class="avatar">
			  <img src="${escapeHtml(parent_reply.user.avatarUrl)}" width=60 alt="">
			</div>
			<div class="content">
			  <div class="author">
				<a href="${HOST}/@${parent_reply.user.username}" rel="nofollow">
				  <span>@${parent_reply.user.username}</span>
				  <span class="MastoHost">${host}</span>
				</a>
				<a class="date" href="${HOST}/notes/${parent_reply.id}" rel="nofollow">
				  ${parent_reply.createdAt.substr(0, 10)}
				 </a>
			  </div>
			  <a class="date" href="${HOST}/notes/${parent_reply.id}" rel="nofollow">
			  <details>
				  <summary>
					Content Warning: ${parent_reply.cw}
				</summary>
				<div class="mastodon-comment-content"><p>${parent_reply.text}<p></div>
			  </details> 
			  </a>
			</div>

		  </div>`;
		} else {
			mastodonComment = `<div class="mastodon-comment">
		   <div class="avatar">
			 <img src="${escapeHtml(parent_reply.user.avatarUrl)}" width=60 alt="">
		   </div>
		   <div class="content">
			 <div class="author">
			   <a href="https://${host}/@${parent_reply.user.username}" rel="nofollow">
				 <span>@${parent_reply.user.username}</span>
				 <span class="MastoHost">${host}</span>
			   </a>
			   <a class="date" href="${HOST}/notes/${parent_reply.id}" rel="nofollow">
				 ${parent_reply.createdAt.substr(0, 10)}
				</a>
			 </div>
			 <a class="date" href="${HOST}/notes/${parent_reply.id}" rel="nofollow">
			 <div class="mastodon-comment-content"><p>${parent_reply.text}<p></div> 
			 </a>
		   </div>

		 </div>`;
		}

		output.push({
			html: DOMPurify.sanitize(mastodonComment, {
				RETURN_DOM_FRAGMENT: true,
			}),
			date: parent_reply.createdAt,
		});
		const resp = await fetch(
			"https://social.nullrequest.com/api/notes/children",
			{
				method: "POST",
				headers: {
					Accept: "application/json",
					"Content-Type": "application/json",
				},
				body: JSON.stringify(payload),
			},
		);

		const data: Reply[] = await resp.json();
		if (data && Array.isArray(data) && data.length > 0) {
			if (comments) {
				if (data.length === 0) {
					comments.innerHTML = "There are no comments maybe make one <3";
					return;
				}
				comments.innerHTML = "Loading Comments";

				for (let i = 0; i < data.length; i++) {
					const reply = data[i];
					reply.user.username = escapeHtml(reply.user.username);

					reply.text = escapeHtml(reply.text);

					reply.user.name = escapeHtml(reply.user.name);

					const array = [...reply.text.matchAll(re)];

					reply.text = replace_emoji(reply.text, reply.emojis);
					reply.user.username = replace_emoji(
						reply.user.username,
						reply.user.emojis,
					);

					if (array.length > 0) {
						const fedi_username = array[0][0];
						const split = fedi_username.split("@");
						const username = split[1];
						const domain = split[2];
						const ref = `https://${domain}/@${username}`;

						reply.text = reply.text.replace(
							array[0][0],
							`<a class="MastodonUsername" href=${ref}>@${username}</a>`,
						);
					}

					const payload = {
						query: reply.user.username,
						offset: 0,
						limit: 1,
						origin: "combined",
						detail: false,
					};

					const userdetails = await fetch(`${HOST}api/users/search`, {
						method: "POST",
						headers: {
							Accept: "application/json",
							"Content-Type": "application/json",
						},
						body: JSON.stringify(payload),
					});
					const json: UserDetails[] = await userdetails.json();
					const user = json[0];
					let host;
					let mastodonComment: string;

					if (user.host) {
						host = user.host;
					} else {
						host = HOST.replace("https://", "").replace("/", "");
					}
					if (reply.cw) {
						mastodonComment = `<div class="mastodon-comment">
						<div class="avatar">
						  <img src="${escapeHtml(reply.user.avatarUrl)}" width=60 alt="">
						</div>
						<div class="content">
						  <div class="author">
							<a href="https://${host}/@${reply.user.username}" rel="nofollow">
							  <span>@${reply.user.username}</span>
							  <span class="MastoHost">${host}</span>
							</a>
							<a class="date" href="${HOST}/notes/${reply.id}" rel="nofollow">
							  ${reply.createdAt.substr(0, 10)}
							 </a>
						  </div>
						  <a class="date" href="${HOST}/notes/${reply.id}" rel="nofollow">
						  <details>
						  	<summary>
								Content Warning: ${reply.cw}
							</summary>
							<div class="mastodon-comment-content"><p>${reply.text}<p></div>
						  </details> 
						  </a>
						</div>
 
					  </div>`;
					} else {
						mastodonComment = `<div class="mastodon-comment">
					   <div class="avatar">
						 <img src="${escapeHtml(reply.user.avatarUrl)}" width=60 alt="">
					   </div>
					   <div class="content">
						 <div class="author">
						   <a href="https://${host}/@${reply.user.username}" rel="nofollow">
							 <span>@${reply.user.username}</span>
							 <span class="MastoHost">${host}</span>
						   </a>
						   <a class="date" href="${HOST}/notes/${reply.id}" rel="nofollow">
							 ${reply.createdAt.substr(0, 10)}
							</a>
						 </div>
						 <a class="date" href="${HOST}/notes/${reply.id}" rel="nofollow">
						 <div class="mastodon-comment-content"><p>${reply.text}<p></div> 
						 </a>
					   </div>

					 </div>`;
					}

					output.push({
						html: DOMPurify.sanitize(mastodonComment, {
							RETURN_DOM_FRAGMENT: true,
						}),
						date: reply.createdAt,
					});
				}
			} else {
				return;
			}
		}
		output.sort((a, b) => Date.parse(a.date) - Date.parse(b.date));
		comments.innerHTML = "";
		for (let i = 0; i < output.length; i++) {
			comments.appendChild(output[i].html);
		}
	}
}

init();

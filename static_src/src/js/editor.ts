import EasyMDE from 'easymde';

declare global {
	interface PostForm {
		title: String,
		description: String | null,
		content: string | null,
		tags: string,
		draft: boolean,
	 }
}

function getData(url: string) {
	return new Promise<string>((resolve, _reject) => {
		fetch(url)
			.then(response => {
				return response.text();
			}).then(data => {
				resolve(data);
			})
	})
}

const easyMDE = new EasyMDE({
	element: document.getElementById("editor")!,
	autosave: {
		enabled: true,
		delay: 1000,
		uniqueId: 'blog-post',
	},
	indentWithTabs: true,
	renderingConfig: {
		codeSyntaxHighlighting: true,
	},
	tabSize: 4,
});

getData(`/api/posts/${sessionStorage.getItem("slug")}`).then(data => {
	let content = data;
	easyMDE.value(content);
});

const editor = document.getElementById("editor_form") as HTMLFormElement;
editor.addEventListener('submit', function (e) {
	update_or_new_post(e);
});

async function update_or_new_post(e: Event) {
	e.preventDefault();
	const btn_submit = document.getElementById("submit") as HTMLButtonElement;
	btn_submit.disabled = true;
	setTimeout(() => btn_submit.disabled = false, 2000);
	let slug = localStorage.getItem("slug");
	let form = new FormData(editor);
	let json = Object.fromEntries(form);
	if (json.draft == "not_draft") {
		json.draft = JSON.stringify(false);
	} else {
		json.draft = JSON.stringify(true);
	}
	if (sessionStorage.getItem("slug")) {
		let slug = sessionStorage.getItem("slug");
		json["slug"] = JSON.stringify(slug);
		fetch(`/api/posts/${slug}/update`, {
			method: 'POST',
			mode: "same-origin",
			cache: "no-cache",
			credentials: "same-origin",
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(json)
		}).then(function (response) {
			return response.json();
		}).then(function (resp) {
			if ('Errors' in resp) {
				alert(resp.Errors);
			} else {
				alert("post has been successfully updated")
			}
		})
	} else {
		fetch("/api/posts/new", {
			method: 'POST',
			mode: "same-origin",
			cache: "no-cache",
			credentials: "same-origin",
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(json)
		}).then(function (response) {
			return response.json();
		}).then(function (resp) {
			if ('slug' in resp) {
				sessionStorage.setItem('slug', resp.slug);
				alert("post has been created")
			} else {
				alert(resp.Errors)
			}
		})
	}
}

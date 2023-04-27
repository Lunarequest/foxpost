import EasyMDE from "easymde";

declare global {
	type PostForm = {
		slug: string;
		title: string;
		description: string | null;
		content: string | null;
		tags: string;
		draft: boolean | string | null;
	};
}

async function getData(url: string): Promise<string> {
	return await new Promise<string>((resolve, _reject) => {
		fetch(url)
			.then(async (response) => {
				return await response.text();
			})
			.then((data) => {
				resolve(data);
			})
			.catch((e) => {
				alert(e);
			});
	});
}

const editorelem = document.getElementById("editor");

if (editorelem) {
	const easyMDE = new EasyMDE({
		element: editorelem,
		autosave: {
			enabled: true,
			delay: 1000,
			uniqueId: "blog-post",
		},
		indentWithTabs: true,
		renderingConfig: {
			codeSyntaxHighlighting: true,
		},
		tabSize: 4,
	});

	getData(`/api/posts/${sessionStorage.getItem("slug")}`).then((data) => {
		const content = data;
		easyMDE.value(content);
	});

	const editor = document.getElementById("editor_form") as HTMLFormElement;
	editor.addEventListener("submit", function (e) {
		update_or_new_post(e);
	});

	async function update_or_new_post(e: Event) {
		e.preventDefault();
		const btn_submit = document.getElementById("submit") as HTMLButtonElement;
		btn_submit.disabled = true;
		setTimeout(() => (btn_submit.disabled = false), 2000);
		const form = new FormData(editor);
		const json = {
			title: form.get("title"),
			description: form.get("description"),
			tags: form.get("tags"),
			draft: form.get("draft"),
			content: form.get("content"),
		} as PostForm;

		if (json.draft === "not_draft") {
			json.draft = false;
		} else {
			json.draft = true;
		}
		if (sessionStorage.getItem("slug")) {
			const slug = sessionStorage.getItem("slug") as string;
			json.slug = slug;
			console.log(json);
			fetch(`/api/posts/${slug}/update`, {
				method: "POST",
				mode: "same-origin",
				cache: "no-cache",
				credentials: "same-origin",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(json),
			})
				.then(async function (response) {
					return await response.json();
				})
				.then(function (resp) {
					if ("Errors" in resp) {
						alert(resp.Errors);
					} else {
						alert("post has been successfully updated");
					}
				});
		} else {
			fetch("/api/posts/new", {
				method: "POST",
				mode: "same-origin",
				cache: "no-cache",
				credentials: "same-origin",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(json),
			})
				.then(async function (response) {
					return await response.json();
				})
				.then(function (resp) {
					if ("slug" in resp) {
						sessionStorage.setItem("slug", resp.slug);
						alert("post has been created");
					} else {
						alert(resp.Errors);
					}
				});
		}
	}
}

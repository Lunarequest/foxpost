function getData(url) {
	return new Promise((resolve, reject) => {
		fetch(url)
			.then(response => {
				return response.text();
			}).then(data => {
				resolve(data);
			})
	})
}

const easyMDE = new EasyMDE({
	element: document.getElementById("editor"),
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

const editor = document.getElementById("editor_form");
editor.addEventListener('submit', function (e) {
	update_or_new_post(e);
});

async function update_or_new_post(e) {
	e.preventDefault();
	const btn_submit = document.getElementById("submit");
	btn_submit.disabled = true;
	setTimeout(() => btn_submit.disabled = false, 2000);
	slug = localStorage.getItem("slug");
	form = new FormData(editor);
	json = Object.fromEntries(form);
	if (json.draft == "not_draft") {
		json.draft = false;
	} else {
		json.draft = true;
	}
	if (sessionStorage.getItem("slug")) {
		slug = sessionStorage.getItem("slug");
		json["slug"] = slug;
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

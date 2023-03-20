window.onload = function () {
	let copyright = document.getElementById("copyright");
	let year = new Date().getFullYear();
	copyright.innerText = `${year} Luna Dragon`;
}

const button = document.querySelector('#menu-button');
const menu = document.querySelector('#menu');

button.addEventListener('click', () => {
	menu.classList.toggle('hidden');
});


function share_on_mastodon(_e) {
	content = encodeURIComponent(document.getElementById("content").value);
	url = document.getElementById("instance_url").value;
	if (!url) {
		url = "https://hachyderm.io"
	}

	masto_url = url + "/share?text=" + content;

	window.open(masto_url, '_blank');
}

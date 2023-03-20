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
document.onload = enableMastodonShare();

/* Call this on document.ready() */
function enableMastodonShare() {
	var eles = document.getElementById('sharemastodon');
	eles.style = ''
}

/* Generate a share link for the user's Mastodon domain */
function MastodonShare(e) {
	console.log(e);
	src = e.getAttribute("data-src");

	// Get the Mastodon domain
	domain = prompt("Enter your Mastodon domain", "mastodon.social");

	if (domain == "" || domain == null) {
		return;
	}

	// Build the URL
	url = "https://" + domain + "/share?text=" + src;

	// Open a window on the share page
	window.open(url, '_blank');
}

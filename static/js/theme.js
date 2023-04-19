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

const icon = document.getElementById("darkmode_icon");
if (localStorage.theme === 'light' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: light)').matches)) {
	document.documentElement.classList.remove('dark');
	icon.className = "fa-solid fa-moon";
} else {
	document.documentElement.classList.add('dark');
	icon.className = "fa-solid fa-sun";
}

const toggle = document.getElementById("dark-mode-toggle");

function setTheme(mode) {
	if (mode === "dark") {
		document.documentElement.classList.add('dark');
		icon.classList.add('fa-sun');
		icon.classList.remove('fa-moon');
	} else if (mode === "light") {
		document.documentElement.classList.remove('dark');
		icon.classList.add('fa-moon');
		icon.classList.remove('fa-sun');
	}
	localStorage.setItem('theme', mode)
}

toggle.addEventListener('click', () => {
	console.log('click');
	if (icon.classList.contains('fa-moon')) {
		setTheme("dark");
	} else if (icon.classList.contains('fa-sun')) {
		setTheme("light");
	}
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

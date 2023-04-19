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

const theme_railscasts = document.getElementById("railscasts");
const theme_tomrrow = document.getElementById("tomorrow");
const toggle = document.getElementById("dark-mode-toggle");

if (localStorage.theme === 'light' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: light)').matches)) {
	document.documentElement.classList.remove('dark');
	toggle.innerHTML = '<i id="darkmode_icon" class="fa-solid fa-moon" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
} else {
	document.documentElement.classList.add('dark');
	toggle.innerHTML = '<i id="darkmode_icon" class="fa-solid fa-sun" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
}

function setTheme(mode) {
	if (mode === "dark") {
		document.documentElement.classList.add('dark');
		toggle.innerHTML = '<i id="darkmode_icon" class="fa-solid fa-sun" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
		if (theme_railscasts && theme_tomrrow) {
			theme_railscasts.removeAttribute('disabled');
			theme_tomrrow.setAttribute('disabled', 'disabled');
		}
	} else if (mode === "light") {
		document.documentElement.classList.remove('dark');
		toggle.innerHTML = '<i id="darkmode_icon" class="fa-solid fa-moon" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
		if (theme_railscasts && theme_tomrrow) {
			theme_tomrrow.removeAttribute('disabled');
			theme_railscasts.setAttribute('disabled', 'disabled');
		}
	}
	localStorage.setItem('theme', mode)
}

toggle.addEventListener('click', () => {
	console.log('click');
	let icon = document.getElementById('darkmode_icon');
	if (icon.classList.contains('fa-moon') || localStorage.getItem('theme') == 'light') {
		setTheme("dark");
	} else if (icon.classList.contains('fa-sun') || localStorage.getItem('theme') == 'dark') {
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

"use strict";

const theme_railscasts = document.getElementById("railscasts");
const theme_tomrrow = document.getElementById("tomorrow");
const toggle = document.getElementById("dark-mode-toggle");
const button = document.querySelector('#menu-button');
const menu = document.querySelector('#menu');

function setTheme(mode) {
	if (mode === "dark") {
		document.documentElement.classList.add('dark');
		if (toggle) {
			toggle.innerHTML = '<i id="darkmode_icon" class="fa-solid fa-sun" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
		}
		if (theme_railscasts && theme_tomrrow) {
			theme_railscasts.removeAttribute('disabled');
			theme_tomrrow.setAttribute('disabled', 'disabled');
		}
	} else if (mode === "light") {
		document.documentElement.classList.remove('dark');
		if (toggle) {
			toggle.innerHTML = '<i id="darkmode_icon" class="fa-solid fa-moon" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
		}
		if (theme_railscasts && theme_tomrrow) {
			theme_tomrrow.removeAttribute('disabled');
			theme_railscasts.setAttribute('disabled', 'disabled');
		}
	}
	localStorage.setItem('theme', mode)
}

if (button) {
	button.addEventListener('click', () => {
		if (menu) {
			menu.classList.toggle('hidden');
		}
	});
}

window.onload = function () {
	let copyright = document.getElementById("copyright");
	if (copyright) {
		let year = new Date().getFullYear();
		copyright.innerText = `${year} Luna Dragon`;
	}
}

if (toggle) {
	if (localStorage.theme === 'light' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: light)').matches)) {
		document.documentElement.classList.remove('dark');
		toggle.innerHTML = '<i id="darkmode_icon" class="fa-solid fa-moon" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
	} else {
		document.documentElement.classList.add('dark');
		toggle.innerHTML = '<i id="darkmode_icon" class="fa-solid fa-sun" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
	}

	toggle.addEventListener('click', () => {
		console.log('click');
		let icon = document.getElementById('darkmode_icon');
		if (icon) {
			if (icon.classList.contains('fa-moon') || localStorage.getItem('theme') == 'light') {
				setTheme("dark");
			} else if (icon.classList.contains('fa-sun') || localStorage.getItem('theme') == 'dark') {
				setTheme("light");
			}
		}
	});
}




function share_on_mastodon() {
	let contentelm = document.getElementById("content");
	let instance_urlform = document.getElementById("instance_url");
	if (contentelm && instance_urlform) {
		let content = encodeURIComponent(contentelm.value);

		let url = instance_urlform.value;
		if (!url) {
			url = "https://hachyderm.io"
		}

		let masto_url = url + "/share?text=" + content;

		window.open(masto_url, '_blank');
	}
}

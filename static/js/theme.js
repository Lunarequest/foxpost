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
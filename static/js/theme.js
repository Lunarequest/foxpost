window.onload = function () {
    let copyright = document.getElementById("copyright");
    let year = new Date().getFullYear();
    copyright.innerText = `${year} Luna Dragon`;
}
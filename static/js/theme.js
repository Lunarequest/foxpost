window.onload = function () {
    let copyright = document.getElementById("copyright");
    let year = new Date().getFullYear();
    copyright.innerText = `${year} Luna Dragon`;
    var toggle = document.getElementById('dark-mode-toggle')

    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
        setTheme(localStorage.getItem('dark-mode-storage') || 'dark')
    } else {
        setTheme(localStorage.getItem('dark-mode-storage') || 'light')
    }

    toggle.addEventListener('click', () => {
        console.log('click');
        if (toggle.className === 'fas fa-moon') {
            setTheme('dark')
        } else if (toggle.className === 'fas fa-sun') {
            setTheme('light')
        }
    })

    function setTheme(mode) {
        localStorage.setItem('dark-mode-storage', mode);
        if (mode === 'dark') {
            document.body.classList.remove("frappe");
            document.body.classList.add("mocha");
        } else if (mode === 'light') {
            document.body.classList.remove("mocha");
            document.body.classList.add("frappe");
            toggle.className = 'fas fa-moon'
        }
    }
}
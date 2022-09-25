const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry)=>{
        if (entry.isIntersecting) {
            entry.target.classList.add('show');
        }
    });
});

const hiddenElemnents = document.querySelectorAll('.hidden');
hiddenElemnents.forEach((el) => observer.observe(el));
const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry)=>{
        if (entry.isIntersecting) {
            entry.target.classList.add('show');
        }else{
            entry.target.classList.remove('show');
        }
    });
});

const hiddenElemnents = document.querySelectorAll('.hidden');
hiddenElemnents.forEach((el) => observer.observe(el));
let gnoming = false;

function gnome(btn) {
    btn.remove();

    if (gnoming) {
        return;
    } else {
        gnoming = true;
    }

    const gnome = document.querySelector('video#gnome');
    gnome.play();
    gnome.classList.add('playing');
    gnome.onended = () => {
        gnoming = false;
        gnome.classList.remove('playing')
    };
}

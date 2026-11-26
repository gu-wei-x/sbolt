const navigationToggle = document.querySelector(".navigation-toggle");
if (navigationToggle) {
    navigationToggle.hidden = false;
    navigationToggle.addEventListener("click", () => {
        const expanded = navigationToggle.getAttribute("aria-expanded") === "true";
        navigationToggle.setAttribute("aria-expanded", String(!expanded));
    });
    navigationToggle.addEventListener("keydown", (event) => {
        if (event.key === "Escape") {
            navigationToggle.setAttribute("aria-expanded", "false");
        }
    });
}

for (const form of document.querySelectorAll("form[data-confirm]")) {
    form.addEventListener("submit", (event) => {
        if (!window.confirm(form.dataset.confirm)) event.preventDefault();
    });
}

for (const dialog of document.querySelectorAll("dialog[open]")) {
    dialog.removeAttribute("open");
    dialog.showModal();
    dialog.addEventListener("cancel", (event) => {
        event.preventDefault();
        window.location.assign(dialog.dataset.close);
    });
    dialog.addEventListener("keydown", (event) => {
        if (event.key === "Escape") {
            event.preventDefault();
            window.location.assign(dialog.dataset.close);
        }
    });
}



window.addEventListener("load", function() {
    console.log("Body creation");
    let div = document.createElement("div");
    document.body.appendChild(div);
    for (var i = 0; i < 100; ++i) {
        var star = document.createElement("div");
        $(star).addClass("star");
        star.style.left = Math.floor(Math.random() * 100) + "vw";
        star.style.top = Math.floor(Math.random() * 100) + "vh";
        star.style.animation = (Math.random()*2 + 0.5) + "s ease 1s alternate infinite fade";
        div.appendChild(star);
    }
});

$(document).ready(function() {
    $(".entry").each(function() {
        var button = document.createElement("div");
        var text = document.createTextNode(">");
        $(button).addClass("entry_button");
        $(button).prepend(text);
        var parent = this;
        var open = true;
        button.style.textContent = "^";
        $(button).click(function() {
            open = !open;
            text.textContent = open ? ">" : "^";
            $(parent).toggleClass("collapsed");
        })
        $(this).prepend(button);
    });
});
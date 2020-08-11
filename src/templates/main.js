var tabChangedEvent = new Event('tabChanged');

function openPage(pageName, button) {
    // Hide all elements with class="tabcontent" by default */
    var i, tabcontent, tablinks;
    tabcontent = document.getElementsByClassName("tabcontent");
    for (i = 0; i < tabcontent.length; i++) {
        tabcontent[i].style.display = "none";
    }
    tabcontent = document.getElementsByClassName("tabcontent_default");
    for (i = 0; i < tabcontent.length; i++) {
        tabcontent[i].style.display = "none";
    }

    // Get all elements with class="tablinks" and remove the class "active"
    tablinks = document.getElementsByClassName("tablink");
    for (i = 0; i < tablinks.length; i++) {
        tablinks[i].className = tablinks[i].className.replace(" active", "");
    }

    // Add an "active" class to the button that opened the tab
    button.className += " active";

    // Show the specific tab content
    document.getElementById(pageName).style.display = "inline";

    window.dispatchEvent(tabChangedEvent);
}

function handleTree() {
    var toggler = document.getElementsByClassName("tree_caret");
    var i;

    for (i = 0; i < toggler.length; i++) {
        toggler[i].addEventListener("click", function() {
            this.parentElement.querySelector(".tree_nested").classList.toggle("tree_active");
            this.classList.toggle("tree_caret-down");
        });
    }
}

window.onload = function () {
    // Get the element with id="defaultOpen" and click on it
    document.getElementById("defaultOpen").click();

    handleTree();
}
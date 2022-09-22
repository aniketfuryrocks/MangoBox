var button = document.getElementById("btn1");
var text = document.getElementById("demo");
button.addEventListener("click", function () {
    let inputVal = document.getElementById("input").value;
    if (inputVal === "omkar") {
        text.style.display = "none";
        chrome.tabs.update({ url: "https://www.youtube.com" });
    } else {
        toggleText();
    }

});


function toggleText() {

    if (text.style.display === "none") {
        text.style.display = "block";
    }
}



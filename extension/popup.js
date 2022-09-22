var button = document.getElementById("btn1");
button.addEventListener("click", function () {
    chrome.tabs.update({ url: "https://www.sollet.io/" });

});



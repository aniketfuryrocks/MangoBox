let store = 0;
var button = document.getElementById("btn1");
var text = document.getElementById("demo");

chrome.storage.sync.get(['key'], function(result) {
    if (result.key === "omkar") {
        store = 1;

        window.location.href = "chat.html";
    }
});

if (store == 0) {
    button.addEventListener("click", function() {
        let inputVal = document.getElementById("input").value;
        if (inputVal === "omkar") {
            text.style.display = "none";

            chrome.storage.sync.set({ key: inputVal }, function() {
                console.log('Value is set to ' + value);
            });

            window.location.href = "chat.html";
        } else {
            toggleText();
        }

    });
}

function toggleText() {

    if (text.style.display === "none") {
        text.style.display = "block";
    }
}




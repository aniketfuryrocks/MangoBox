const chats = document.getElementById("chats");
const chat_input_box = document.getElementById("chat-input-box");
const chat_input_send_bt = document.getElementById("chat-input-send-bt")

chat_input_send_bt.addEventListener('click', _ => {
    const chat_msg = chat_input_box.value;
    
    console.log(chat_msg);

    chat_input_box.value = "";
});

connect_to_server();

function connect_to_server() {
    const socket = new WebSocket('ws://0.0.0.0:8080/chat');

    socket.addEventListener('open', _ => {
        console.log("Mango > Connected to Server: Hot Reload Enabled");
    });

    socket.addEventListener('close', _ => {
        console.error("Mango > Disconnected from server");
        if (confirm('Disconnected from server. Refresh ?'))
            location.reload()
    });

    socket.addEventListener('message', event => {
        const data = JSON.parse(event.data);
        if (data.event === "FileChange")
            location.reload()
        console.log('Mango > Message from server ', data);
    });

    socket.addEventListener('error', event => {
        console.error(event);
    });
}

const chats = document.getElementById("chats");
const chat_input_box = document.getElementById("chat-input-box");
const chat_input_send_bt = document.getElementById("chat-input-send-bt")

disable_input(true);

// connect to server
const socket = connect_to_server();
chat_input_send_bt.addEventListener('click', _ => {
    const chat_msg = chat_input_box.value;

    socket.send(JSON.stringify({
        ty: "Msg",
        data: chat_msg // room
    }));

    add_my_msg(chat_msg);

    chat_input_box.value = "";
});

function add_my_msg(msg) {
    const msg_el = document.createElement('div');
    msg_el.classList.add('my-chat');
    msg_el.innerText = msg;
    chats.appendChild(msg_el);
}

function add_client_msg(msg) {
    const msg_el = document.createElement('div');
    msg_el.classList.add('client-chat');
    msg_el.innerText = msg;
    chats.appendChild(msg_el);
}

function makeid(length) {
    var result           = '';
    var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var charactersLength = characters.length;
    for ( var i = 0; i < length; i++ ) {
      result += characters.charAt(Math.floor(Math.random() * 
 charactersLength));
   }
   return result;
}
// initiate connection and listen to events
function connect_to_server() {
    const socket = new WebSocket(`ws://0.0.0.0:8080/chat?q=${makeid(5)}`);

    socket.addEventListener('open', _ => {
        console.log("Mango > Connected To Server");

        // join room general
        socket.send(JSON.stringify({
            ty: "Join",
            data: "general" // room
        }));

        disable_input(false);
    });

    socket.addEventListener('close', _ => {
        console.error("Mango > Disconnected from server");
        if (confirm('Disconnected from server. Refresh ?'))
            location.reload()
    });

    socket.addEventListener('message', event => {
        const { ty, data } = JSON.parse(event.data);

        switch (ty) {
            case 'Msg':
                add_client_msg(data);
                break;
            case 'Err':
                alert(data);
                break;
            case 'Info':
                alert(data);
                break;
        }
        console.log('Mango > Message from server ', data);
    });

    socket.addEventListener('error', event => {
        console.error(event);
    });

    return socket;
}

function disable_input(disable) {
    chat_input_box.disable = disable;
    chat_input_send_bt.disable = disable;
}

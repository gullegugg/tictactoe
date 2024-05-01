const socket = new WebSocket("/ws")

const sendForm = document.getElementById("send-command")

function onSubmit(e) {
    e.preventDefault()

    const data = new FormData(sendForm)

    socket.send(data.get("command"))
}

sendForm.addEventListener("submit", onSubmit)

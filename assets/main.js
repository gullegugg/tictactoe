const statusElement = document.getElementById("status")
const sendForm = document.getElementById("send-command")

const socket = new WebSocket("/ws")

socket.addEventListener("message", (event) => {
    statusElement.textContent = event.data
})

socket.addEventListener("open", (event) => {
    console.log("open", event)
})

socket.addEventListener("error", (event) => {
    console.log("error ", event)
})

socket.addEventListener("close", (event) => {
    console.log("Close", event)
})

function onSubmit(e) {
    e.preventDefault()

    const data = new FormData(sendForm)

    socket.send(data.get("command"))
}

sendForm.addEventListener("submit", onSubmit)

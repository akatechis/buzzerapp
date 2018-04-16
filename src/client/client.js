const socket = new WebSocket('ws://127.0.0.1:3000', 'rust-ws')

socket.onmessage = function (event) {
  const received = document.getElementById('received')
  const item = document.createElement('li')
  const text = document.createTextNode(event.data)
  item.appendChild(text)
  received.appendChild(item)
}

const sendMessage = form => e => {
  e.preventDefault()
  const input = form.querySelector('input.text')
  socket.send(input.value)
  input.value = ''
}

const main = () => {
  const form = document.getElementById('socket-form')
  form.addEventListener('submit', sendMessage(form))
}

main()

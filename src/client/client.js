const host = window.location.host || '127.0.0.1'
const socket = new WebSocket('ws://' + host + ':4000')

const submitJoinForm = form => e => {
  e.preventDefault()
  const input = form.querySelector('input.text')
  const msg = JSON.stringify(joinMessage(input.value))
  socket.send(msg)
  input.value = ''
}

const joinMessage = name => {
  return {
    type: 'Join',
    name: name
  }
}

const main = () => {
  const form = document.getElementById('join-form')
  form.addEventListener('submit', submitJoinForm(form))
}

main()

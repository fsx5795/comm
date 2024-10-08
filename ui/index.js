const { invoke } = window.__TAURI__.tauri
window.addEventListener('DOMContentLoaded', () => {
    const select = document.querySelector('select')
    invoke('get_ip').then((msg) => {
	for (let i = 0; i < msg.length; ++i) {
	    const option = document.createElement('option')
	    option.innerText = msg[i]
	    select.appendChild(option)
	}
    })
    document.querySelector('button').onclick = () => {
	invoke('start_udp', { ip: document.querySelector('select').value, port: parseInt(document.querySelector('input').value) })
    }
})

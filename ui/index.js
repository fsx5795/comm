const { invoke } = window.__TAURI__.tauri
window.addEventListener('DOMContentLoaded', () => {
    const select = document.querySelector('select')
    invoke('get_ip').then((msg) => {
	const option = document.createElement('option')
	option.innerText = msg
	select.appendChild(option)
    })
})

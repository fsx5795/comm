function inputValueChanged() {
    const input = document.querySelector('input')
    input.value = input.value.replace(/\D/g, '')
}
const { listen } = window.__TAURI__.event
const unlisten = async() => {
    await listen('error', event => {
        const dialog = document.querySelector('dialog')
        dialog.querySelector('button').addEventListener('click', () => { dialog.close() })
        const ps = dialog.querySelectorAll('p')
        ps[0].innerText = '错误'
        ps[1].innerText = event.payload
        dialog.showModal()
    })
}
document.addEventListener('DOMContentLoaded', () => {
    const form = document.querySelector('form')
    form.addEventListener('submit', event => {
        event.preventDefault()
    })
    const select = form.querySelector('select')
    const input = form.querySelector('input')
    const multinput = document.getElementById('multaddr')
    //提交按钮
    form.querySelector('button').onclick = () => {
        const radios = document.getElementsByName('network')
        for (let i in radios) {
            if (radios[i].checked == true) {
                if (radios[i].value === "udp")
                    invoke('start_udp', { ip: select.options[select.selectedIndex].value, port: parseInt(input.value), multaddr: multinput.value })
                break
            }
        }
    }
    input.addEventListener('input', () => inputValueChanged())
    input.addEventListener('afterpaste', () => inputValueChanged())
    const { invoke } = window.__TAURI__.core
    invoke('get_ip').then((addrs) => {
        addrs.forEach((addr, _) => {
            const option = document.createElement('option')
            option.innerText = addr
            select.appendChild(option)
        })
    })
})
unlisten()
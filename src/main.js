const { invoke } = window.__TAURI__.tauri;
const fetch = async (url) => {return await invoke("fetch", { url: url } )}

window.addEventListener("DOMContentLoaded", () => {
  fetch('gemini://gemini.circumlunar.space/').then((content) => {
    document.body.innerHTML = content;
  })
});
const { invoke } = window.__TAURI__.tauri;

let fnameInputEl;
let lnameInputEl;
let emailInputEl;
let phoneInputEl;
let genderInputEl;
let dateInputEl;
let testiclesInputEl;
let greetMsgEl;

async function gather_info() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("gather_info", {
    fname: fnameInputEl.value,
    lname: lnameInputEl.value,
    mail: emailInputEl.value,
    pnum: phoneInputEl.value,
    gen: genderInputEl.value,
    bday: dateInputEl.value,
    balls: testiclesInputEl.value
  });
}

window.addEventListener("DOMContentLoaded", () => {
  fnameInputEl = document.querySelector("#fname-input");
  lnameInputEl = document.querySelector("#lname-input");
  emailInputEl = document.querySelector("#email-input");
  phoneInputEl = document.querySelector("#phone-input");
  genderInputEl = document.querySelector("#gender-input");
  dateInputEl = document.querySelector("#birthdate-input");
  testiclesInputEl = document.querySelector("#testicles-input");

  greetMsgEl = document.querySelector("#greet-msg");

  document
    .querySelector("#send-button")
    .addEventListener("click", () => gather_info());
});

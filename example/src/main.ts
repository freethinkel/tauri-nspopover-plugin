<<<<<<< HEAD
import { TrayIcon } from "@tauri-apps/api/tray";
import { isOpen, show, hide } from "tauri-plugin-nspopover";

const main = async () => {
  TrayIcon.new({
    id: "main",
    async action(event) {
      console.log(event);
      if (
        event.type === "Click" &&
        event.buttonState === "Up" &&
        event.button === "Left"
      ) {
        const isShown = await isOpen();

        if (isShown) {
          hide();
        } else {
          show();
        }
      }
    },
  });
};

// uncomment and disable rust code to handle popover in javascript
// main();
||||||| af9b6d0
=======
import { invoke } from "@tauri-apps/api/tauri";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
>>>>>>> main

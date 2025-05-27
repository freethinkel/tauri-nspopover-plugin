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

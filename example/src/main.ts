import { TrayIcon } from "@tauri-apps/api/tray";
import { isOpen, show, hide } from "tauri-plugin-nspopover";

const main = () => {
  TrayIcon.new({
    id: "main",
    async action() {
      const isShown = await isOpen();
      if (isShown) {
        hide();
      } else {
        show();
      }
    },
  });
};

// uncomment and disable rust code to handle popover in javascript
// main()

import { invoke } from "@tauri-apps/api/core";

export const isOpen = () => invoke("plugin:nspopover|is_popover_shown");
export const show = () => invoke("plugin:nspopover|show_popover");
export const hide = () => invoke("plugin:nspopover|hide_popover");

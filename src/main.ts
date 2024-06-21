import './app.css'
import App from './App.svelte'

const app = new App({
  target: document.getElementById('app'),
})

export default app

import { appWindow } from "@tauri-apps/api/window";

const minimizeButton = document.getElementById('titlebar-minimize');
const closeButton = document.getElementById('titlebar-close');

if (minimizeButton) {
  minimizeButton.addEventListener('click', () => appWindow.minimize());
}

if (closeButton) {
  closeButton.addEventListener('click', () => appWindow.close());
}
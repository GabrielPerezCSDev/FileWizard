// preload.js
const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electron', {
  sendFolderPath: (path) => ipcRenderer.send('folder-path', path),
});

ipcRenderer.on('folder-path-reply', (event, arg) => {
  console.log('Received reply from main process:', arg);
});

window.addEventListener('DOMContentLoaded', () => {
    console.log('[Preload] Running in Electron environment');
});
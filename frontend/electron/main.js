// main.js
const { app, BrowserWindow, ipcMain, session } = require('electron');
const path = require('path');

let mainWindow;

try {
  require('electron-reloader')(module, { ignore: ['node_modules'] });
} catch (err) {
  console.log('Hot reloading not enabled:', err);
}


function createWindow() {
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      enableRemoteModule: false,
    },
  });

  if (process.env.NODE_ENV === 'development') {
    mainWindow.loadURL('http://localhost:3000'); // For Vite dev server
  } else {
    mainWindow.loadFile(path.join(__dirname, '../react/dist/index.html')); // Main file
  }

  // Prevent Electron from opening files on drop
  mainWindow.webContents.on('will-navigate', (event) => {
    event.preventDefault();
  });

  mainWindow.webContents.openDevTools();
}

app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

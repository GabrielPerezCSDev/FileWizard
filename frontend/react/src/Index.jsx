import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
if (window.electron) {
    console.log('[React] Running in Electron environment');
} else {
    console.log('[React] Running in Browser environment');
}
ReactDOM.createRoot(document.getElementById('root')).render(<App />);

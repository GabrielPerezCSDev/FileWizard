// components/search/SearchContainer.jsx
import React, { useState, useEffect } from 'react';
import './styling/SearchContainer.css';

const SearchContainer = ({ isSearchStopped }) => {
  const [state, setState] = useState('upload');
  const [folderPath, setFolderPath] = useState('');

  useEffect(() => {
    if (isSearchStopped) {
      setState('upload');
      setFolderPath(''); // Reset folder path when search is stopped
    }
  }, [isSearchStopped]);

  const handleDrop = (event) => {
    event.preventDefault();
    const items = event.dataTransfer.items;

    if (items && items.length > 0) {
      const item = items[0].webkitGetAsEntry();
      if (item && item.isDirectory) {
        // Use Electron's API to get the full path
        const fullPath = item.fullPath; // This will give you the full path in Electron
        setFolderPath(fullPath); // Set the full folder path
      }
    }
  };

  return (
    <div className="search-container">
      {state === 'upload' ? (
        <div 
          className="file-upload-area drop-zone"
          onDrop={handleDrop}
          onDragOver={(e) => e.preventDefault()}
        >
          <div className="file-upload-content">
            <i className="fas fa-file-upload"></i>
            <p>Drag and drop a folder here</p>
            <p>or enter a directory</p>
            <button className="file-select-button">Choose a file</button>
            <input 
              type="text" 
              placeholder="Enter directory path" 
              className="directory-input"
            />
            {folderPath && <p>Folder Path: {folderPath}</p>} {/* Display the folder path */}
          </div>
        </div>
      ) : (
        <div className="search-placeholder">
          <p>Search in progress...</p>
        </div>
      )}
    </div>
  );
};

export default SearchContainer;
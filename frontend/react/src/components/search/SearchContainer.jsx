import React, { useState, useEffect } from 'react';
import './styling/SearchContainer.css';
import { startSearch, stopSearch } from '../../controllers/search/searchContainerController';

const SearchContainer = ({ isSearchStopped }) => {
  const [state, setState] = useState('upload'); // Controls the component state
  const [folderPath, setFolderPath] = useState(''); // Holds the entered or dropped directory path

  useEffect(() => {
    console.log('[SearchContainer.jsx] useEffect triggered with isSearchStopped:', isSearchStopped);
    if (isSearchStopped) {
      console.log('[SearchContainer.jsx] Resetting to upload state');
      setState('upload');
      setFolderPath('');
    }
  }, [isSearchStopped]);

  const handleInputChange = (event) => {
    const inputValue = event.target.value;
    console.log('[handleInputChange] Directory input changed:', inputValue);
    setFolderPath(inputValue); // Update folderPath state with user input
  };

  const handleStartClick = () => {
    if (folderPath) {
      console.log('[handleStartClick] Starting search with directory:', folderPath);
      startSearch(folderPath).then((success) => {
        if (success) {
          console.log('[handleStartClick] Switching to search in progress view.');
          setState('searching'); // Change state to "searching" on successful start
        }
      });
    } else {
      console.warn('[handleStartClick] No directory path provided.');
      alert('Please enter or drop a directory path before starting.');
    }
  };

  const handleStopClick = () => {
    console.log('[handleStopClick] Stopping search...');
    stopSearch().then(() => {
      console.log('[handleStopClick] Search stopped successfully.');
      // Optionally, you can reset the state here if needed
      setState('upload'); // Reset to upload state after stopping
    }).catch((error) => {
      console.error('[handleStopClick] Error stopping search:', error);
    });
  };

  return (
    <div className="search-container">
      {state === 'upload' ? (
        <div className="file-upload-area drop-zone">
          <div className="file-upload-content">
            <p>Drag and drop a folder here</p>
            <input
              type="text"
              placeholder="Enter directory path"
              value={folderPath} // Bind the input value to folderPath
              onChange={handleInputChange} // Update folderPath on user input
            />
            <button className="search-button" onClick={handleStartClick}>
              Start
            </button>
          </div>
        </div>
      ) : (
        <div className="search-placeholder">
          {/* Container for D3 Tree Graph */}
        <div id="d3-tree-container" className="d3-tree">
          {/* D3 Graph renders here */}
        </div>
          <button className="stop-button" onClick={handleStopClick}>
            Stop
          </button>
        </div>
      )}
    </div>
  );
};

export default SearchContainer;
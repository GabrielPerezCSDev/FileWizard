//SearchContainer.jsx
import React, { useState, useEffect } from 'react';
import './styling/SearchContainer.css';
import D3Tree from './D3Tree';

const SearchContainer = ({ isSearchStopped, onStartSearch }) => {
  const [folderPath, setFolderPath] = useState('');
  const [treeKey, setTreeKey] = useState(0); // State to force re-render

  console.log('[SearchContainer] Rendering with isSearchStopped:', isSearchStopped);

  // Dummy data for D3 testing
  const dummyData = {
    name: 'Root',
    children: [
      {
        name: 'Folder A',
        metadata: { raw_size: 100 },
        children: [
          { name: 'File A1', metadata: { raw_size: 50 } },
          { name: 'File A2', metadata: { raw_size: 50 } },
        ],
      },
      {
        name: 'Folder B',
        metadata: { raw_size: 200 },
        children: [
          { name: 'File B1', metadata: { raw_size: 150 } },
          { name: 'File B2', metadata: { raw_size: 50 } },
        ],
      },
    ],
  };

  useEffect(() => {
    console.log('[SearchContainer] useEffect triggered with isSearchStopped:', isSearchStopped);
    if (isSearchStopped) {
      setFolderPath('');
      setTreeKey(prevKey => prevKey + 1); // Increment key to force re-render of D3Tree
      console.log('[SearchContainer] Folder path reset');
    }
  }, [isSearchStopped]);

  const handleInputChange = (event) => {
    const inputValue = event.target.value;
    console.log('[SearchContainer] Directory input changed:', inputValue);
    setFolderPath(inputValue);
  };

  const handleStartClick = () => {
    if (folderPath) {
      console.log('[SearchContainer] Starting search with directory:', folderPath);
      onStartSearch(folderPath);
      setTreeKey(prevKey => prevKey + 1); // Increment key to force re-render of D3Tree
    } else {
      console.warn('[SearchContainer] No directory path provided.');
      alert('Please enter or drop a directory path before starting.');
    }
  };
  
  return (
    <div className="search-container">
      {isSearchStopped ? (
        <div className="file-upload-area drop-zone">
          <div className="file-upload-content">
            <p>Drag and drop a folder here</p>
            <input
              type="text"
              placeholder="Enter directory path"
              value={folderPath}
              onChange={handleInputChange}
            />
            <button className="search-button" onClick={handleStartClick}>
              Start
            </button>
          </div>
        </div>
      ) : (
        <div className="search-placeholder">
          <D3Tree key={treeKey} data={dummyData} /> {/* Use treeKey to force re-render */}
        </div>
      )}
    </div>
  );
};

export default SearchContainer;
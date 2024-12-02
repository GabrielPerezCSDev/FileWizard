//SearchContainer.jsx
import React, { useState, useEffect, useRef } from 'react';
import './styling/SearchContainer.css';
import D3Tree from './D3Tree';
import { fetchD3Data } from '../../controllers/search/searchContainerController';

const SearchContainer = ({ isSearchStopped, onStartSearch }) => {
  const [folderPath, setFolderPath] = useState('');
  const [treeKey, setTreeKey] = useState(0);
  const [treeData, setTreeData] = useState(null);
  const pollingIntervalRef = useRef(null);

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
    // Clear any existing interval when the component mounts or isSearchStopped changes
    if (pollingIntervalRef.current) {
      clearInterval(pollingIntervalRef.current);
      pollingIntervalRef.current = null;
    }

    if (!isSearchStopped) {
      console.log('[Polling] Starting polling');
      
      // Create a function to fetch data
      const fetchData = async () => {
        try {
          console.log('[Polling] Fetching data from /search/get_root');
          const rootData = await fetchD3Data();
          
          if (rootData) {
            console.log('[Polling] Updating tree data');
            setTreeData(rootData);
          }
        } catch (error) {
          console.error('Error during polling fetch:', error);
        }
      };

      // Initial fetch
      fetchData();

      // Start a new polling interval
      pollingIntervalRef.current = setInterval(fetchData, 100);
    }

    // Cleanup function to clear interval when component unmounts or search stops
    return () => {
      if (pollingIntervalRef.current) {
        clearInterval(pollingIntervalRef.current);
        pollingIntervalRef.current = null;
        console.log('[Polling] Stopping polling');
      }
    };
  }, [isSearchStopped]); // Re-run effect when isSearchStopped changes

  useEffect(() => {
    if (isSearchStopped) {
      setFolderPath('');
      setTreeData(null);
      setTreeKey(prevKey => prevKey + 1);
      console.log('[SearchContainer] Folder path and tree data reset');
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
      setTreeKey(prevKey => prevKey + 1);
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
          <D3Tree key={treeKey} data={treeData} />
        </div>
      )}
    </div>
  );
};

export default SearchContainer;
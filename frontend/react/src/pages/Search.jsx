import React, { useState } from 'react';
import Taskbar from '../components/search/Taskbar';
import SearchContainer from '../components/search/SearchContainer';
import { startSearch } from '../controllers/search/searchContainerController';
import { stopSearch } from '../controllers/search/taskbarController';
import './styling/Search.css';

function Search() {
  const [isSearchStopped, setIsSearchStopped] = useState(true);

  const handleStopSearch = async () => {
    console.log('[Search.jsx] handleStopSearch triggered');
    const success = await stopSearch(); // Call your API function without folderPath
    if (success) {
        setIsSearchStopped(true); // Set to true when stopped
        console.log('[Search.jsx] Search stopped successfully.');
    } else {
        console.error('[Search.jsx] Failed to stop search.');
    }
};

  const handleStartSearch = async (folderPath) => { // Accept folder path as an argument
    console.log('[Search.jsx] handleStartSearch triggered with path:', folderPath);
    const success = await startSearch(folderPath); // Call your API function with the path
    if (success) {
        setIsSearchStopped(false);
        console.log('[Search.jsx] Search started successfully.');
    } else {
        console.error('[Search.jsx] Failed to start search.');
    }
};


  console.log('[Search.jsx] Rendering SearchContainer with isSearchStopped:', isSearchStopped);

  return (
    <div className="search-page">
      
      <Taskbar 
      isSearchStopped={isSearchStopped} 
      onStop={handleStopSearch} 
      />
      <SearchContainer 
        isSearchStopped={isSearchStopped} 
        onStartSearch={handleStartSearch} 
      />
      
    </div>
  );
}

export default Search;
import React, { useState } from 'react';
import Taskbar from '../components/search/Taskbar';
import SearchContainer from '../components/search/SearchContainer';
import './styling/Search.css';

function Search() {
  const [isSearchStopped, setIsSearchStopped] = useState(true);

  const handleStopSearch = () => {
    setIsSearchStopped(true);
    console.log('[Search.jsx] isSearchStopped state set to:', true);
  };

  const handleStartSearch = () => {
    console.log('[Search.jsx] handleStartSearch triggered');
    setIsSearchStopped(false); // Verify this updates `isSearchStopped`
  };

  console.log('[Search.jsx] Rendering SearchContainer with isSearchStopped:', isSearchStopped);

  return (
    <div className="search-page">
      {/* Pass isSearchStopped to control visibility of the Stop button */}
      <Taskbar isSearchStopped={isSearchStopped} />
      
      <SearchContainer isSearchStopped={isSearchStopped} />
    </div>
  );
}

export default Search;
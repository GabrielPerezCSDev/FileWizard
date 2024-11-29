import React, { useState } from 'react';
import Taskbar from '../components/search/Taskbar';
import SearchContainer from '../components/search/SearchContainer';
import './styling/Search.css';

function Search() {
  const [isSearchStopped, setIsSearchStopped] = useState(true);

  const handleStopSearch = () => {
    setIsSearchStopped(true);
  };

  const handleStartSearch = () => {
    setIsSearchStopped(false);
  };

  return (
    <div className="search-page">
      <Taskbar onStop={handleStopSearch} onStart={handleStartSearch} />
      <SearchContainer isSearchStopped={isSearchStopped} />
    </div>
  );
}

export default Search;

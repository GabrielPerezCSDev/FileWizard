import React from 'react';
import Taskbar from '../components/search/Taskbar';
function Search() {
  console.log("[Search} Component has been activated!");
  return (
    <><Taskbar />
    <div>
          <h2>Search time!</h2>
          <p>Organize your files effortlessly.</p>
      </div></>
  );
}

export default Search;
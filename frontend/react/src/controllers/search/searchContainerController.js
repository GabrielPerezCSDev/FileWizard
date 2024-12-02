
//searchContainerController.js
export const startSearch = async (folderPath) => {
  console.log('[startSearch] Setting directory to:', folderPath);

  try {
    const setDirectoryResponse = await fetch('http://localhost:8080/search/set_directory', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ new_directory: folderPath }),
    });

    if (!setDirectoryResponse.ok) {
      throw new Error('Failed to set directory.');
    }

    console.log('[startSearch] Directory set successfully.');

    const startResponse = await fetch('http://localhost:8080/search/start', { method: 'GET' });
    if (!startResponse.ok) {
      throw new Error('Failed to start search.');
    }

    console.log('[startSearch] Search started successfully.');
    return true; // Indicate success
  } catch (error) {
    console.error('[startSearch] Error during search:', error);
    alert('Failed to start search. Please check your input and try again.');
    return false; // Indicate failure
  }
};

export const stopSearch = () => {
  console.log('Stop clicked');
  return fetch('http://localhost:8080/search/stop', { method: 'GET' }) // Add 'return' here
      .then((response) => {
          if (!response.ok) throw new Error('Failed to stop search');
          return response.text();
      })
      .then((data) => {
          console.log('Search stopped:', data);
      })
      .catch((error) => console.error('Error stopping search:', error));
};

export const fetchD3Data = async () => {
  try {
    //console.log('[fetchD3Data] Attempting to fetch data...');
    const response = await fetch('http://localhost:8080/search/get_root', { 
      method: 'GET',
      headers: {
        'Accept': 'application/json'
      }
    });

    //console.log('[fetchD3Data] Response status:', response.status);
    
    if (!response.ok) {
      const errorText = await response.text();
      console.error('[fetchD3Data] Error response:', errorText);
      throw new Error(`Failed to fetch D3 data. Status: ${response.status}`);
    }

    const data = await response.json();
    //console.log('[fetchD3Data] Data received:', data);
    return data;
  } catch (error) {
    console.error('[fetchD3Data] Detailed error:', error);
    return null;
  }
};
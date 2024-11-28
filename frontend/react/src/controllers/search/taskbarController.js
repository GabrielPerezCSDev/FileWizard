export const startSearch = () => {
    console.log('Start clicked');
    // Call the backend endpoint for starting the search
    fetch('http://localhost:8080/search/start', { method: 'GET' })
        .then((response) => {
            if (!response.ok) throw new Error('Failed to start search');
            return response.text();
        })
        .then((data) => {
            console.log('Search started:', data);
        })
        .catch((error) => console.error('Error starting search:', error));
};

export const pauseSearch = () => {
    console.log('Pause clicked');
    fetch('http://localhost:8080/search/pause', { method: 'GET' })
        .then((response) => {
            if (!response.ok) throw new Error('Failed to pause search');
            return response.text();
        })
        .then((data) => {
            console.log('Search paused:', data);
        })
        .catch((error) => console.error('Error pausing search:', error));
};

export const resumeSearch = () => {
    console.log('Resume clicked');
    fetch('http://localhost:8080/search/resume', { method: 'GET' })
        .then((response) => {
            if (!response.ok) throw new Error('Failed to resume search');
            return response.text();
        })
        .then((data) => {
            console.log('Search resumed:', data);
        })
        .catch((error) => console.error('Error resuming search:', error));
};

export const stopSearch = () => {
    console.log('Stop clicked');
    fetch('http://localhost:8080/search/stop', { method: 'GET' })
        .then((response) => {
            if (!response.ok) throw new Error('Failed to stop search');
            return response.text();
        })
        .then((data) => {
            console.log('Search stopped:', data);
        })
        .catch((error) => console.error('Error stopping search:', error));
};


//taskbarController.js
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

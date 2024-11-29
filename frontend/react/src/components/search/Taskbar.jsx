import React from 'react';
import { startSearch, pauseSearch, resumeSearch, stopSearch } from '../../controllers/search/taskbarController';
import './styling/Taskbar.css'
const Taskbar = ({ onStart, onStop }) => {
    const handleStart = () => {
        startSearch().then(() => {
            onStart();
        }).catch((error) => {
            console.error('Error starting search:', error);
        });
    };

    const handleStop = () => {
        stopSearch().then(() => {
            onStop();
        }).catch((error) => {
            console.error('Error stopping search:', error);
        });
    };

    return (
        <div className="taskbar">
            <button className="taskbar-button start-btn" onClick={handleStart}>
                Start
            </button>
            <button className="taskbar-button pause-btn" onClick={pauseSearch}>
                Pause
            </button>
            <button className="taskbar-button resume-btn" onClick={resumeSearch}>
                Resume
            </button>
            <button className="taskbar-button stop-btn" onClick={handleStop}>
                Stop
            </button>
        </div>
    );
};

export default Taskbar;
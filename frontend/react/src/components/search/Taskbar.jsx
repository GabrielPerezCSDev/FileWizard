// components/search/Taskbar.jsx
import React from 'react';
import { pauseSearch, resumeSearch } from '../../controllers/search/taskbarController';
import { stopSearch } from '../../controllers/search/searchContainerController';
import './styling/Taskbar.css';

const Taskbar = ({ isSearchStopped, onStop }) => {
    return (
        <div className="taskbar">
            <button className="taskbar-button resume-btn" onClick={resumeSearch}>
                Resume
            </button>
            <button className="taskbar-button pause-btn" onClick={pauseSearch}>
                Pause
            </button>
        </div>
    );
};

export default Taskbar;
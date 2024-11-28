import React from 'react';
import { startSearch, pauseSearch, resumeSearch, stopSearch } from '../../controllers/search/taskbarController';

const Taskbar = () => {
    return (
        <div className="taskbar">
            <button className="taskbar-button start-btn" onClick={startSearch}>
                Start
            </button>
            <button className="taskbar-button pause-btn" onClick={pauseSearch}>
                Pause
            </button>
            <button className="taskbar-button resume-btn" onClick={resumeSearch}>
                Resume
            </button>
            <button className="taskbar-button stop-btn" onClick={stopSearch}>
                Stop
            </button>
        </div>
    );
};

export default Taskbar;
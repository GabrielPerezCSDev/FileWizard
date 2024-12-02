// components/search/Taskbar.jsx
import {resumeSearch, pauseSearch, stopSearch} from '../../controllers/search/taskbarController'
import './styling/Taskbar.css'

const Taskbar = ({ isSearchStopped, onStart, onStop }) => {
    const handleStopClick = () => {
      console.log('[Taskbar] Stop button clicked');
      if (typeof onStop === 'function') {
        console.log('[Taskbar] Calling handleStop in parent component');
        onStop();
      } else {
        console.error('[Taskbar] onStop is not a function');
      }
    };
  
    return (
      <div className="taskbar">
          <>
            <button className="taskbar-button pause-btn" onClick={pauseSearch}>
              Pause
            </button>
            <button className="taskbar-button resume-btn" onClick={resumeSearch}>
              Pause
            </button>
            <button className="taskbar-button stop-btn" onClick={handleStopClick}>
              Stop
            </button>
          </>
      </div>
    );
  };
  
  export default Taskbar;
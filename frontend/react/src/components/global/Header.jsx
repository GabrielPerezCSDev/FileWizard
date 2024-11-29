import React from 'react';
import './styling/Header.css';
import { Link } from 'react-router-dom';

function Header() {
  return (
    <header>
      {/* Logo Section */}
      <div className="logo">
        <img 
          src="/file_wizard_logo.png" 
          alt="File Wizard Logo" 
        />
      </div>

      {/* Navigation Section */}
      <nav className="nav-links">
        <ul>
          <li><Link to="/">Home</Link></li>
          <li><Link to="/search">Search</Link></li>
        </ul>
      </nav>
    </header>
  );
}

export default Header;
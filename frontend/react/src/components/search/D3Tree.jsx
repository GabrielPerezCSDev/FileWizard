//D3Tree.jsx
import React, { useEffect } from 'react';
import { renderD3Tree } from '../../utils/d3Tree';

const D3Tree = ({ containerId = 'd3-tree-container', data }) => {
  useEffect(() => {
    if (data) {
      const container = document.getElementById(containerId);
      if (container) {
        renderD3Tree(containerId, data);
      }
    }
  }, [data, containerId]);

  return (
    <div 
      id={containerId} 
      style={{ 
        width: '100%', 
        height: '100%',
        position: 'relative',
        overflow: 'hidden', // Prevent any overflow
        backgroundColor: '#a62424'
      }}
    />
  );
};

export default D3Tree;
import React, { useEffect } from 'react';
import { renderD3Tree } from '../../utils/d3Tree';

const D3Tree = ({ data }) => {
    useEffect(() => {
        console.log('[D3Tree Component] Rendering tree with data:', data);
      
        const containerId = 'd3-tree-container';
        console.log(`[D3Tree Component] Calling renderD3Tree with containerId: ${containerId}`);
        renderD3Tree(containerId, data);
      
        return () => {
            console.log('[D3Tree Component] Cleaning up SVG');
            const container = document.getElementById(containerId);
            if (container) {
              const svg = container.querySelector('svg');
              if (svg) svg.remove(); // Remove only the SVG, not the entire container content
            }
          };
      }, [data]);

  return <div id="d3-tree-container" style={{ width: '100%', height: '500px' }} />;
};

export default D3Tree;
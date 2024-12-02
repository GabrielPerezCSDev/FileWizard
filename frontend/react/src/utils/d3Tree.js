//d3Tree.js
import * as d3 from 'd3';

/**
 * Transform backend JSON to D3-compatible hierarchical format
 * @param {Object} backendData - The JSON data from the backend
 * @returns {Object} D3-compatible hierarchical object
 */
function transformBackendDataForD3(backendData) {
    // If the data doesn't match expected structure, return a default object
    if (!backendData || !backendData.name) {
        return { name: 'Root', children: [] };
    }

    // Helper function to recursively transform data
    const transformNode = (node) => {
        // Base transformation object
        const transformedNode = {
            name: node.name,
            // Use raw_size for sizing, default to 1 if not present
            size: parseInt(node.metadata?.raw_size || 1),
            // Preserve original metadata for potential future use
            originalMetadata: node.metadata
        };

        // Add children if they exist
        if (node.children && node.children.length > 0) {
            transformedNode.children = node.children.map(transformNode);
        }

        return transformedNode;
    };

    // Transform the root data
    return transformNode(backendData);
}

/**
 * Render a D3 treemap graph in the specified container.
 * @param {string} containerId - The ID of the container DOM element.
 * @param {Object} backendData - The hierarchical data from backend.
 */
export function renderD3Tree(containerId, backendData) {
    console.log('[renderD3Tree] Rendering treemap for container:', containerId);
    console.log('[renderD3Tree] Backend data received:', backendData);

    // Transform backend data to D3-compatible format
    const data = transformBackendDataForD3(backendData);
    console.log('[renderD3Tree] Transformed D3 data:', data);

    const container = d3.select(`#${containerId}`);
    const width = container.node().clientWidth;
    const height = container.node().clientHeight;

    console.log('[renderD3Tree] Container dimensions:', { width, height });

    // Clear existing SVG
    container.selectAll('svg').remove();

    const svg = container
        .append('svg')
        .attr('width', width)
        .attr('height', height)
        .style('background-color', '#f0f0f0');

    // Create hierarchy and sum sizes
    const root = d3.hierarchy(data)
        .sum(d => d.size)
        .sort((a, b) => b.value - a.value);

    // Create treemap layout
    const treemapLayout = d3.treemap()
        .size([width, height])
        .padding(1)
        .round(true);

    // Apply treemap layout
    treemapLayout(root);

    // Color scale for differentiation
    const color = d3.scaleOrdinal(d3.schemeCategory10);

    // Create nodes
    const nodes = svg
        .selectAll('g')
        .data(root.leaves())
        .join('g')
        .attr('transform', d => `translate(${d.x0},${d.y0})`);

    // Rectangles for each node
    nodes
        .append('rect')
        .attr('width', d => d.x1 - d.x0)
        .attr('height', d => d.y1 - d.y0)
        .attr('fill', (d, i) => color(i))
        .attr('stroke', '#fff')
        .attr('stroke-width', 1);

    // Labels for each node
    nodes
        .append('text')
        .attr('x', 5)
        .attr('y', 20)
        .text(d => d.data.name)
        .attr('font-size', '10px')
        .attr('fill', 'black')
        .call(wrap, 100); // Optional text wrapping function

    // Optional text wrapping function
    function wrap(text, width) {
        text.each(function() {
            const text = d3.select(this);
            const words = text.text().split(/\s+/).reverse();
            let word;
            let line = [];
            let lineNumber = 0;
            const lineHeight = 1.1; // ems
            const y = text.attr('y');
            const dy = parseFloat(text.attr('dy') || 0);
            let tspan = text.text(null).append('tspan')
                .attr('x', 5)
                .attr('y', y)
                .attr('dy', dy + 'em');

            while (word = words.pop()) {
                line.push(word);
                tspan.text(line.join(' '));
                if (tspan.node().getComputedTextLength() > width) {
                    line.pop();
                    tspan.text(line.join(' '));
                    line = [word];
                    tspan = text.append('tspan')
                        .attr('x', 5)
                        .attr('y', y)
                        .attr('dy', ++lineNumber * lineHeight + dy + 'em')
                        .text(word);
                }
            }
        });
    }
}
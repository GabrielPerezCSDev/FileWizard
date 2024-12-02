import * as d3 from 'd3';

/**
 * Render a D3 treemap graph in the specified container.
 * @param {string} containerId - The ID of the container DOM element.
 * @param {Object} data - The hierarchical data for the treemap.
 */
export function renderD3Tree(containerId, data) {
    console.log('[renderD3Tree] Rendering treemap for container:', containerId);
    console.log('[renderD3Tree] Data received:', data);

    const container = d3.select(`#${containerId}`);
    const width = container.node().getBoundingClientRect().width;
    const height = container.node().getBoundingClientRect().height;

    console.log('[renderD3Tree] Container dimensions:', { width, height });

    // Clear existing SVG
    container.selectAll('svg').remove();
    console.log('[renderD3Tree] Cleared previous SVG');

    const svg = container
        .append('svg')
        .attr('width', width)
        .attr('height', height)
        .style('background-color', '#f0f0f0'); // Temporary background for debugging

    const root = d3.hierarchy(data).sum(d => d.metadata?.raw_size || 1);

    const treemapLayout = d3.treemap().size([width, height]).padding(1);
    treemapLayout(root);

    console.log('[renderD3Tree] Nodes:', root.descendants());

    const nodes = svg
        .selectAll('g')
        .data(root.descendants())
        .join('g')
        .attr('transform', d => `translate(${d.x0},${d.y0})`);

    nodes
        .append('rect')
        .attr('width', d => d.x1 - d.x0)
        .attr('height', d => d.y1 - d.y0)
        .attr('fill', d => (d.children ? '#69b3a2' : '#d3e5ff'))
        .attr('stroke', '#000')
        .each((d, i, nodes) => {
            console.log(`[renderD3Tree] Rect #${i} - width: ${d.x1 - d.x0}, height: ${d.y1 - d.y0}`);
        });

    nodes
        .append('text')
        .attr('dx', 5)
        .attr('dy', 20)
        .text(d => d.data.name)
        .style('font-size', '12px');
}

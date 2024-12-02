import * as d3 from 'd3';

export function renderD3Tree(containerId, backendData) {
    const container = d3.select(`#${containerId}`);
    const containerDiv = document.getElementById(containerId);
    const boundingRect = containerDiv.getBoundingClientRect();
    const width = boundingRect.width;
    const height = boundingRect.height;

    // Clear existing SVG
    container.selectAll('svg').remove();

    const svg = container
        .append('svg')
        .attr('width', width)
        .attr('height', height)
        .style('display', 'block');

    // Add a main group for the treemap
    const mainGroup = svg.append('g');

    // Transform data
    const data = transformBackendDataForD3(backendData);
    
    // Create hierarchy and sum sizes
    const root = d3.hierarchy(data)
        .sum(d => d.size)
        .sort((a, b) => b.value - a.value);

    // Create treemap layout
    const treemapLayout = d3.treemap()
        .size([width, height])
        

    treemapLayout(root);

    // Helper function to check if a name represents a folder
    const isFolder = (name) => !name.includes('.');

    // Color scale for differentiation between files and folders
    const color = d3.scaleOrdinal()
        .domain(['file', 'folder'])
        .range(['#64748b', '#0ea5e9']) // Orange for files, Steel Blue for folders

     // Create nodes with interactions
     const nodes = mainGroup
     .selectAll('g')
     .data(root.children || [])
     .join('g')
     .attr('transform', d => `translate(${d.x0},${d.y0})`)
     .style('cursor', d => isFolder(d.data.name) ? 'pointer' : 'default') // Change cursor for folders
     .on('mouseover', (event, d) => {
         // Log metadata on hover
         console.log('Hovering:', {
             name: d.data.name,
             size: formatSize(d.value),
             type: isFolder(d.data.name) ? 'folder' : 'file',
             metadata: d.data.originalMetadata
         });

         // Change fill color on hover
         d3.select(event.currentTarget)
             .select('rect')
             .transition()
             .duration(200)
             .attr('fill', isFolder(d.data.name) ? '#0bc8eb' : '#9d6eff');
     })
     .on('mouseout', (event, d) => {
         // Reset fill color
         d3.select(event.currentTarget)
             .select('rect')
             .transition()
             .duration(200)
             .attr('fill', isFolder(d.data.name) ? '#06b6d4' : '#8b5cf6');
     })
     .on('click', (event, d) => {
        // Only handle clicks for folders
        if (isFolder(d.data.name)) {
            console.log('Would navigate to:', `${window.location.pathname}/${d.data.name}`);
            
            // Here you would typically:
            // 1. Update the URL
            // 2. Trigger a new data fetch
            // 3. Re-render with new data
            console.log('Folder clicked:', {
                name: d.data.name,
                path: `${window.location.pathname}/${d.data.name}`,
                action: 'navigate'
            });
        }
    });

    // Rectangles for each node
    nodes
        .append('rect')
        .attr('width', d => Math.max(d.x1 - d.x0, 0))
        .attr('height', d => Math.max(d.y1 - d.y0, 0))
        .attr('fill', d => color(isFolder(d.data.name) ? 'folder' : 'file'))
        .attr('stroke', '#fff')
        .attr('stroke-width', 1);

    // Function to format size
    const formatSize = (size) => {
        if (size < 1024) return `${size} B`;
        if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
        if (size < 1024 * 1024 * 1024) return `${(size / (1024 * 1024)).toFixed(1)} MB`;
        return `${(size / (1024 * 1024 * 1024)).toFixed(1)} GB`;
    };

    // Add labels
    const labels = nodes
        .append('g')
        .attr('class', 'node-label')
        .attr('pointer-events', 'none');

    // File/folder name
    labels
        .append('text')
        .attr('x', 4)
        .attr('y', 15)
        .text(d => d.data.name)
        .attr('font-size', '12px')
        .attr('fill', 'black')

    // Size text
    labels
        .append('text')
        .attr('x', 4)
        .attr('y', 30)
        .text(d => formatSize(d.value))
        .attr('font-size', '10px')
        .attr('fill', '#666')

    // Hide labels that don't fit
    labels.each(function() {
        const label = d3.select(this);
        const parentNode = d3.select(this.parentNode);
        const rect = parentNode.select('rect');
        const rectWidth = parseFloat(rect.attr('width'));
        const rectHeight = parseFloat(rect.attr('height'));

        if (rectWidth < 60 || rectHeight < 40) {
            label.style('display', 'none');
        }
    });
}

function transformBackendDataForD3(backendData) {
    if (!backendData || !backendData.name) {
        return { name: 'Root', children: [] };
    }

    // Take only immediate children with their sizes
    const children = backendData.children?.map(child => ({
        name: child.name,
        size: parseInt(child.metadata?.raw_size || 1),
    })) || [];

    // Return simplified data structure
    return {
        name: backendData.name,
        children: children
    };
}
import * as d3 from 'd3';
import { switchRoot} from '../controllers/search/d3Controller';
export function renderD3Tree(containerId, backendData) {
    let isNavigating = false;
    // Different size thresholds
    const SIZE_THRESHOLDS = {
        tiny: { width: 40, height: 30 },      // Show just name with smaller font
        small: { width: 80, height: 40 },     // Show name and size
        medium: { width: 120, height: 60 },   // Show name, size, and type
        large: { width: 180, height: 100 }    // Show full metadata
    };

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
        .size([width, height]);

    treemapLayout(root);

    // Helper function to check if a name represents a folder
    const isFolder = (name) => !name.includes('.');

    // Helper function to format date
    const formatDate = (dateString) => {
        return new Date(dateString).toLocaleString().split(',')[0];
    };

    // Color scale for differentiation between files and folders
    const color = d3.scaleOrdinal()
        .domain(['file', 'folder'])
        .range(['#64748b', '#0ea5e9']);

    // Function to format size
    const formatSize = (size) => {
        if (size < 1024) return `${size} B`;
        if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
        if (size < 1024 * 1024 * 1024) return `${(size / (1024 * 1024)).toFixed(1)} MB`;
        return `${(size / (1024 * 1024 * 1024)).toFixed(1)} GB`;
    };

    // Create nodes
    const nodes = mainGroup
        .selectAll('.node')
        .data(root.children || [])
        .join('g')
        .attr('class', 'node')
        .attr('transform', d => `translate(${d.x0},${d.y0})`);

    // Add rectangles
    nodes.append('rect')
        .attr('width', d => Math.max(d.x1 - d.x0, 0))
        .attr('height', d => Math.max(d.y1 - d.y0, 0))
        .attr('fill', d => color(isFolder(d.data.name) ? 'folder' : 'file'))
        .attr('stroke', '#fff')
        .attr('stroke-width', 1);

    // Add text container
    const textContainers = nodes
        .append('g')
        .attr('class', 'text-container')
        .style('pointer-events', 'none');

    // Function to create default text with size-based formatting
    const createDefaultText = (container, d) => {
        container.selectAll('*').remove();
        
        const rect = container.node().parentNode;
        const rectWidth = parseFloat(d3.select(rect).select('rect').attr('width'));
        const rectHeight = parseFloat(d3.select(rect).select('rect').attr('height'));

        if (rectWidth < SIZE_THRESHOLDS.tiny.width || rectHeight < SIZE_THRESHOLDS.tiny.height) {
            // Tiny: Just name in smaller font
            container.append('text')
                .attr('x', 2)
                .attr('y', 12)
                .text(d => d.data.name.length > 8 ? d.data.name.slice(0, 8) + '...' : d.data.name)
                .attr('font-size', '8px')
                .attr('fill', 'black');
        } else if (rectWidth < SIZE_THRESHOLDS.small.width || rectHeight < SIZE_THRESHOLDS.small.height) {
            // Small: Name only
            container.append('text')
                .attr('x', 3)
                .attr('y', 15)
                .text(d.data.name)
                .attr('font-size', '10px')
                .attr('fill', 'black');
        } else {
            // Regular: Name and size
            container.append('text')
                .attr('x', 4)
                .attr('y', 15)
                .text(d.data.name)
                .attr('font-size', '12px')
                .attr('fill', 'black');

            container.append('text')
                .attr('x', 4)
                .attr('y', 30)
                .text(formatSize(d.value))
                .attr('font-size', '10px')
                .attr('fill', '#666');
        }
    };

    // Function to create metadata text with size-based content
    const createMetadataText = (container, d) => {
        container.selectAll('*').remove();
        const metadata = d.data.metadata;
        const isDir = isFolder(d.data.name);
        
        const rect = container.node().parentNode;
        const rectWidth = parseFloat(d3.select(rect).select('rect').attr('width'));
        const rectHeight = parseFloat(d3.select(rect).select('rect').attr('height'));

        if (rectWidth < SIZE_THRESHOLDS.medium.width || rectHeight < SIZE_THRESHOLDS.medium.height) {
            // Simple metadata for smaller rectangles
            container.append('text')
                .attr('x', 4)
                .attr('y', 15)
                .text(d.data.name)
                .attr('font-size', '10px')
                .attr('fill', '#1f2937');

            container.append('text')
                .attr('x', 4)
                .attr('y', 28)
                .text(`${isDir ? 'Folder' : 'File'} - ${formatSize(d.value)}`)
                .attr('font-size', '9px')
                .attr('fill', '#666');
        } else if (rectWidth < SIZE_THRESHOLDS.large.width || rectHeight < SIZE_THRESHOLDS.large.height) {
            // Medium size metadata
            container.append('text')
                .attr('x', 4)
                .attr('y', 15)
                .text(d.data.name)
                .attr('font-size', '11px')
                .attr('fill', '#1f2937');

            const details = [
                `Type: ${isDir ? 'Folder' : 'File'}`,
                `Size: ${formatSize(d.value)}`,
                isDir ? `Items: ${d.data.children?.length || 0}` : `Ext: ${metadata?.file_extension || 'None'}`
            ];

            details.forEach((text, i) => {
                container.append('text')
                    .attr('x', 4)
                    .attr('y', 32 + (i * 15))
                    .text(text)
                    .attr('font-size', '9px')
                    .attr('fill', '#666');
            });
        } else {
            // Full metadata for large rectangles
            container.append('text')
                .attr('x', 4)
                .attr('y', 16)
                .text(d.data.name)
                .attr('font-size', '12px')
                .attr('font-weight', 'bold')
                .attr('fill', '#1f2937');

            const lines = [
                ['Type', isDir ? 'Folder' : 'File'],
                ['Size', formatSize(d.value)],
                isDir ? ['Items', d.data.children?.length || 0] : ['Ext', metadata?.file_extension || 'None'],
                ['Created', metadata?.created ? formatDate(metadata.created) : 'Unknown'],
                ['Modified', metadata?.modified ? formatDate(metadata.modified) : 'Unknown'],
                ['Path', d.data.url || 'Unknown']
            ];

            lines.forEach((item, i) => {
                container.append('text')
                    .attr('x', 8)
                    .attr('y', 40 + (i * 18))
                    .text(`${item[0]}: ${item[1]}`)
                    .attr('font-size', '10px')
                    .attr('fill', '#666');
            });
        }
    };

    // Apply default text
    textContainers.each(function(d) {
        createDefaultText(d3.select(this), d);
    });

    // Add interactions
    nodes
        .style('cursor', d => isFolder(d.data.name) ? 'pointer' : 'default')
        .on('mouseenter', function(event, d) {
            const textContainer = d3.select(this).select('.text-container');
            createMetadataText(textContainer, d);
        })
        .on('mouseleave', function(event, d) {
            const textContainer = d3.select(this).select('.text-container');
            createDefaultText(textContainer, d);
        })
        .on('click', async (event, d) => {
            if (isFolder(d.data.name) && !isNavigating) {
                try {
                    isNavigating = true; // Set flag before starting
                    console.log('Folder being sent to change too:', {
                        name: d.data.name,
                        path: d.data.url || '',
                        action: 'navigate'
                    });
                    await switchRoot(d.data.url);
                } catch (error) {
                    console.error('Navigation error:', error);
                } finally {
                    isNavigating = false; // Reset flag whether successful or not
                }
            }
        });
}

function transformBackendDataForD3(backendData) {
    if (!backendData || !backendData.name) {
        return { name: 'Root', children: [] };
    }

    const children = backendData.children?.map(child => ({
        name: child.name,
        size: parseInt(child.metadata?.raw_size || 1),
        metadata: child.metadata,
        type: child.type,
        children: child.children,
        url: child.url
    })) || [];

    return {
        name: backendData.name,
        children: children
    };
}
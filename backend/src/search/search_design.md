# Real-Time Directory Search Algorithm Documentation

This document outlines the design and architecture for the real-time directory search algorithm used in the GUI application. It provides an overview of the approach, key components, and the interaction between threads and data structures.

---

## Overview

The search algorithm is designed to traverse directories in a multi-threaded manner while providing continuous, real-time updates to the frontend. The design focuses on minimizing contention by allowing each worker thread to update its own local map of discovered files and folders. These local maps are then periodically aggregated into a global snapshot, which is sent to the GUI for display.

---

## Architecture

### Components

- **Thread Orchestrator (Controller):**
  - Spawns a fixed number of worker threads.
  - Manages a queue of directories to be searched.
  - Monitors thread progress and completion.
  - Periodically aggregates local results from each thread.

- **Worker Threads:**
  - Each thread is assigned a directory from the current search root.
  - Each thread maintains its own local map (map pool) of discovered files and folders.
  - Updates to the local map are continuous and do not require global locking.
  
- **Local Map Pools:**
  - Individual collections (e.g., hash maps or vectors) maintained by each worker thread.
  - These local maps store discovered paths (files and folders) along with their metadata.
  
- **Snapshot Aggregation:**
  - The orchestrator provides a method (`get_map_pool`) that copies the current state of each thread's local map.
  - The aggregated snapshot is used to update the frontend display.
  - The snapshot is a “freeze frame” that does not interfere with ongoing thread updates.

---

## Detailed Workflow

1. **Initialization:**
   - The thread orchestrator is created with a fixed number of worker threads.
   - A global work queue is populated with the initial directory (or directories) for the search.

2. **Worker Thread Operation:**
   - Each worker thread picks a directory from the work queue.
   - The thread traverses the directory recursively, discovering files and subdirectories.
   - For each discovered item, the thread updates its own local map pool with the file/folder information and associated metadata.
   - If a thread encounters errors (e.g., inaccessible files), it tags the entry accordingly. These are later aggregated for error reporting.

3. **Local Map Pool Updates:**
   - Updates to local maps are continuous and thread-local, which avoids global contention.
   - No locks are required for local maps because each thread has its own dedicated collection.

4. **Snapshot Aggregation (Periodic):**
   - At regular intervals (or on GUI refresh), the thread orchestrator calls `get_map_pool`.
   - This method copies (or “freezes”) the current state of all local map pools.
   - The aggregated results are merged into a single data structure that is passed to the frontend.
   - Since the snapshot is read-only, no locks are needed during the merge.

5. **GUI Update:**
   - The frontend receives the aggregated snapshot and updates the display in real time.
   - The user sees continuous updates as new files and directories are discovered.

6. **Completion:**
   - The orchestrator monitors the work queue and the status of all worker threads.
   - When every thread has finished processing and no directories remain, the search is complete.
   - Final results, including any error-tagged entries, are available for further processing or reporting.

---

## Advantages of This Design

- **Low Contention:**  
  Each thread works on its own local map, reducing the need for global locking.
  
- **Scalability:**  
  The design scales with the number of threads. A snapshot aggregator can efficiently combine local results.
  
- **Real-Time Updates:**  
  Continuous local updates and periodic snapshots provide near-real-time search progress to the GUI.
  
- **Error Handling:**  
  Errors are captured at the thread level and aggregated, making it easy to display or log problematic files/folders.
  
- **Simplicity:**  
  Local maps simplify concurrency management. The freeze-frame approach for the GUI minimizes complexity in data merging.

---

## Future Extensions

- **Dynamic Thread Pool Sizing:**  
  Tune the number of worker threads based on system performance and workload characteristics.
  
- **Advanced Aggregation:**  
  Consider using concurrent data structures or lock-free queues if the workload grows significantly.
  
- **Platform-Specific Enhancements:**  
  Extend the system with additional metadata handling for other platforms (e.g., Windows, macOS) by leveraging the common metadata interface.
  
- **GUI Feedback Loop:**  
  Integrate progress reporting from the orchestrator to the GUI to provide visual feedback on search completion status.

---

## Conclusion

This search algorithm design balances real-time GUI responsiveness with efficient, low-contention directory traversal. By using a thread orchestrator and per-thread local map pools, the system minimizes global locking while still providing aggregated, up-to-date search results for the frontend.


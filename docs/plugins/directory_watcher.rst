Directory Watcher Plugin
=====================

Overview
--------
The Directory Watcher plugin monitors specified directories for file system changes and generates events when files are created, deleted, modified, or renamed. It uses Windows API for efficient directory monitoring and supports both single directory and recursive subdirectory watching.

Core Components
--------------
Directory Monitor
~~~~~~~~~~~~~~~
.. code-block:: rust

    pub struct DirectoryMonitor {
        path: PathBuf,
        include_subdirs: bool,
        stop_event: Event,
        watcher_thread: Option<JoinHandle<()>>,
        state: Arc<Mutex<MonitorState>>,
    }

    impl DirectoryMonitor {
        pub fn start_monitoring(&mut self) -> Result<(), Error> {
            // Initialize directory handle
            // Set up notification buffer
            // Start monitoring thread
            // Handle startup conditions
        }
    }

Event Generation System
~~~~~~~~~~~~~~~~~~~~~
.. code-block:: rust

    pub enum FileSystemEvent {
        Created(PathBuf),
        Deleted(PathBuf),
        Modified(PathBuf),
        Renamed(PathBuf, PathBuf),  // old_path, new_path
    }

    impl EventHandler for DirectoryMonitor {
        fn handle_event(&mut self, event: FileSystemEvent) -> Result<(), Error> {
            // Process file system event
            // Generate EventGhost event
            // Handle event payload
        }
    }

Key Features
-----------
1. File System Monitoring
   - File creation detection
   - File deletion tracking
   - Modification monitoring
   - Rename operation handling
   - Subdirectory recursion support

2. Event Generation
   - Created events
   - Deleted events
   - Updated events
   - Renamed events with path tracking
   - Full path information in payloads

3. Configuration Options
   - Watch path selection
   - Subdirectory inclusion toggle
   - Event prefix customization
   - Buffer size configuration
   - Thread management

4. Windows Integration
   - Native API usage
   - Efficient change notification
   - Asynchronous operation
   - Resource management
   - Error handling

Migration Considerations
----------------------
1. Windows API Integration
   - Safe API wrappers
   - Error handling
   - Resource cleanup
   - Thread safety

2. Event System
   - Async event generation
   - Thread coordination
   - Buffer management
   - Path handling

Implementation Strategy
---------------------
1. Directory Monitoring
   .. code-block:: rust

    impl DirectoryMonitor {
        pub fn monitor_changes(&mut self) -> Result<(), Error> {
            let (tx, rx) = mpsc::channel();
            
            // Set up Windows directory monitoring
            let handle = unsafe {
                CreateFileW(
                    self.path.as_os_str(),
                    FILE_LIST_DIRECTORY,
                    FILE_SHARE_READ | FILE_SHARE_WRITE,
                    null_mut(),
                    OPEN_EXISTING,
                    FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OVERLAPPED,
                    null_mut(),
                )
            };

            // Start monitoring thread
            self.watcher_thread = Some(thread::spawn(move || {
                self.watch_loop(handle, tx)
            }));

            // Process events
            while let Ok(event) = rx.recv() {
                self.handle_event(event)?;
            }
            
            Ok(())
        }
    }

2. Event Processing
   .. code-block:: rust

    impl EventProcessor {
        pub fn process_change(&mut self, buffer: &[u8]) -> Result<Vec<FileSystemEvent>, Error> {
            let mut events = Vec::new();
            let mut offset = 0;
            
            while offset < buffer.len() {
                let info = unsafe { 
                    &*(buffer.as_ptr().add(offset) as *const FILE_NOTIFY_INFORMATION)
                };
                
                let event = match info.Action {
                    FILE_ACTION_ADDED => FileSystemEvent::Created(self.get_path(info)?),
                    FILE_ACTION_REMOVED => FileSystemEvent::Deleted(self.get_path(info)?),
                    FILE_ACTION_MODIFIED => FileSystemEvent::Modified(self.get_path(info)?),
                    FILE_ACTION_RENAMED_OLD_NAME => {
                        self.old_name = Some(self.get_path(info)?);
                        continue;
                    }
                    FILE_ACTION_RENAMED_NEW_NAME => {
                        if let Some(old_path) = self.old_name.take() {
                            FileSystemEvent::Renamed(old_path, self.get_path(info)?)
                        } else {
                            continue;
                        }
                    }
                    _ => continue,
                };
                
                events.push(event);
                
                if info.NextEntryOffset == 0 {
                    break;
                }
                offset += info.NextEntryOffset as usize;
            }
            
            Ok(events)
        }
    }

Testing Strategy
---------------
1. Unit Tests
   - Event generation
   - Path handling
   - Configuration validation
   - Error conditions

2. Integration Tests
   - File system operations
   - Event triggering
   - Thread management
   - Resource cleanup

3. Performance Tests
   - Large directory handling
   - High-frequency changes
   - Memory usage
   - Thread coordination

Error Handling
-------------
1. File System Errors
   - Access denied
   - Path not found
   - Invalid paths
   - Resource exhaustion

2. Event Processing
   - Buffer overflow
   - Invalid events
   - Thread termination
   - Resource cleanup

3. Configuration Errors
   - Invalid paths
   - Permission issues
   - Thread creation
   - Resource allocation

Platform Considerations
---------------------
1. Windows Integration
   - ReadDirectoryChangesW API
   - Event handling
   - Thread management
   - Resource cleanup

2. Cross-platform Support
   - Platform-specific APIs
   - Event normalization
   - Path handling
   - Resource management 
# EventGhost Plugin Architecture Analysis

## Overview
This document contains detailed architectural analysis of EventGhost plugins, focusing on their internal structure, dependencies, and migration considerations.

## Analysis Wave 2 Plugins

### Global Monitor Plugin
#### Core Components
- [x] System Performance Counters
- [x] Resource Usage Tracking
- [x] Event Generation Pipeline
- [x] Data Collection Services

#### Dependencies
- Windows Performance APIs
- System Metrics Interface
- Event System Integration
- Configuration Storage

#### Migration Considerations
- Cross-platform monitoring alternatives
- Performance impact optimization
- Data sampling strategies
- Event throttling mechanisms

### File Operations Plugin
#### Core Components
- [ ] File System Watchers
- [ ] Operation Interceptors
- [ ] Path Management System
- [ ] Event Triggers

#### Dependencies
- Windows File APIs
- File System Change Notifications
- Security Permissions
- Path Resolution System

#### Migration Considerations
- Cross-platform file operations
- Security model implementation
- Performance optimization
- Error handling strategies

### Directory Watcher Plugin
#### Core Components
- [ ] Directory Monitor Service
- [ ] Change Detection System
- [ ] Filter Management
- [ ] Event Dispatcher

#### Dependencies
- File System Notifications
- Pattern Matching System
- Event Queue Management
- Configuration Storage

#### Migration Considerations
- Scalability for large directories
- Resource usage optimization
- Filter system design
- Cross-platform compatibility

### RadioSure Plugin
#### Core Components
- [x] Window Observation System
- [x] Event Generation Pipeline
- [x] Thread Management
- [x] Windows API Integration

#### Dependencies
- Windows API (User32)
- EventGhost System Events
- Scheduler System
- System Encoding

#### Migration Considerations
- Thread safety improvements
- Window handle management
- Event generation optimization
- Unicode handling

## Common Patterns
### Event Generation
- Standard event format
- Event priority system
- Event filtering mechanisms
- Event routing strategies

### Configuration Management
- Plugin settings storage
- User preferences handling
- Configuration validation
- Default value management

### Resource Management
- Resource allocation tracking
- Cleanup procedures
- Error recovery
- Performance optimization

### Testing Strategy
- Unit test templates
- Integration test patterns
- Performance benchmarks
- Cross-platform validation

## Next Steps
1. Detailed analysis of each plugin's internal architecture
2. Identification of shared components
3. Development of common interfaces
4. Creation of migration templates

## Questions to Address
- How to handle platform-specific features?
- What common interfaces can be extracted?
- How to manage plugin dependencies?
- What testing strategies work best?

### Global Monitor Plugin Analysis

#### Core Components
1. **Performance Counter System**
```rust
pub struct PerformanceCounters {
    counters: HashMap<CounterType, PDH_HCOUNTER>,
    query: PDH_HQUERY,
    sampling_interval: Duration,
}

impl PerformanceCounters {
    fn collect_metrics(&mut self) -> Result<SystemMetrics, Error> {
        // Safe PDH API usage
        // Counter collection
        // Data aggregation
        // Error handling
    }
}
```

2. **Resource Monitoring**
```rust:plugins/globalmonitor/monitoring.rs
pub struct ResourceMonitor {
    cpu_monitor: CpuMonitor,
    memory_monitor: MemoryMonitor,
    disk_monitor: DiskMonitor,
    network_monitor: NetworkMonitor,
    state: Arc<Mutex<MonitorState>>,
}

impl ResourceMonitor {
    async fn monitor_loop(&mut self) {
        while let Ok(()) = tokio::time::sleep(self.config.interval).await {
            self.collect_metrics().await?;
            self.generate_events().await?;
        }
    }
}
```

3. **Event Generation Pipeline**
```rust:plugins/globalmonitor/events.rs
pub enum MonitorEventType {
    CpuUsage(f64),
    MemoryUsage(MemoryMetrics),
    DiskActivity(DiskMetrics),
    NetworkActivity(NetworkMetrics),
    SystemAlert(AlertType),
}

impl EventGenerator for GlobalMonitorPlugin {
    fn generate_metric_event(&self, metric: MonitorEventType) -> Result<(), Error> {
        // Type-safe event generation
        // Threshold checking
        // Alert generation
        // Event routing
    }
}
```

4. **Data Collection Services**
```rust:plugins/globalmonitor/collectors.rs
pub trait MetricCollector {
    type Metric;
    fn collect(&self) -> Result<Self::Metric, Error>;
    fn get_thresholds(&self) -> &MetricThresholds;
    fn check_alerts(&self, metric: &Self::Metric) -> Vec<AlertType>;
}

pub struct CpuCollector {
    pdh_query: PDH_HQUERY,
    counters: Vec<PDH_HCOUNTER>,
    thresholds: CpuThresholds,
}

pub struct MemoryCollector {
    thresholds: MemoryThresholds,
}
```

#### Key Features

1. **Performance Monitoring**
- CPU usage tracking
- Memory utilization
- Disk activity
- Network statistics
- Process metrics
- System resources

2. **Event Generation**
- Threshold-based alerts
- Resource usage events
- System state changes
- Performance warnings
- Trend analysis

3. **Configuration**
- Sampling intervals
- Alert thresholds
- Counter selection
- Event filtering
- Data aggregation

4. **Resource Management**
- Counter cleanup
- Query handling
- Memory management
- Thread coordination
- State persistence

#### Migration Challenges

1. **PDH API Integration**
- Safe counter management
- Query lifecycle
- Error handling
- Resource cleanup

2. **Performance Impact**
- Efficient sampling
- Resource overhead
- Thread scheduling
- Data aggregation

3. **Cross-Platform Support**
- Linux metrics
- MacOS integration
- Common interface
- Platform detection

4. **Implementation Strategy**
```rust:plugins/globalmonitor/lib.rs
pub struct GlobalMonitorPlugin {
    monitors: Arc<Mutex<ResourceMonitor>>,
    config: MonitorConfig,
    event_tx: mpsc::Sender<MonitorEvent>,
}

impl Plugin for GlobalMonitorPlugin {
    fn start(&mut self) -> Result<(), Error> {
        // Initialize monitors
        // Start collection
        // Set up events
    }

    fn stop(&mut self) -> Result<(), Error> {
        // Stop collection
        // Cleanup resources
        // Handle errors
    }
}
```

#### Testing Strategy

1. **Unit Tests**
```rust:plugins/globalmonitor/tests.rs
#[cfg(test)]
mod tests {
    #[test]
    fn test_counter_collection() {
        // Test PDH queries
        // Verify metrics
        // Check cleanup
    }

    #[test]
    fn test_threshold_alerts() {
        // Test alert generation
        // Verify thresholds
        // Check conditions
    }
}
```

2. **Integration Tests**
```rust:plugins/globalmonitor/integration_tests.rs
#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_monitoring_cycle() {
        // Test full monitoring
        // Verify events
        // Check resources
    }
}
```

3. **Performance Tests**
```rust:plugins/globalmonitor/bench.rs
#[cfg(test)]
mod benchmarks {
    #[bench]
    fn bench_metric_collection() {
        // Measure collection time
        // Check memory usage
        // Verify overhead
    }
}
```

#### Error Handling

1. **Counter Errors**
```rust
#[derive(Error, Debug)]
pub enum CounterError {
    #[error("Failed to open counter: {0}")]
    OpenError(String),
    #[error("Failed to collect data: {0}")]
    CollectionError(String),
    #[error("Invalid counter path: {0}")]
    InvalidPath(String),
}
```

2. **Recovery Strategy**
```rust
impl ResourceMonitor {
    fn handle_counter_error(&mut self, error: CounterError) -> Result<(), Error> {
        match error {
            CounterError::OpenError(_) => self.reinitialize_counter(),
            CounterError::CollectionError(_) => self.retry_collection(),
            CounterError::InvalidPath(_) => self.use_fallback_counter(),
        }
    }
}
```

#### Platform Considerations

1. **Windows Implementation**
```rust
#[cfg(target_os = "windows")]
mod windows {
    use windows::Win32::System::Performance::*;
    
    pub struct WindowsCounters {
        query: PDH_HQUERY,
        counters: Vec<PDH_HCOUNTER>,
    }
}
```

2. **Linux Implementation**
```rust
#[cfg(target_os = "linux")]
mod linux {
    pub struct LinuxCounters {
        proc_stat: ProcStat,
        sys_info: SysInfo,
    }
}
```

3. **Common Interface**
```rust
pub trait SystemMetrics {
    fn cpu_usage(&self) -> Result<f64, Error>;
    fn memory_usage(&self) -> Result<MemoryMetrics, Error>;
    fn disk_activity(&self) -> Result<DiskMetrics, Error>;
    fn network_activity(&self) -> Result<NetworkMetrics, Error>;
}
``` 
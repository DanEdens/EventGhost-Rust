## Plugin Configuration and GUI Patterns

### Core Configuration Components

The plugin configuration system in EventGhost is built around several key components:

1. **ConfigPanel**: The base class for all plugin configuration dialogs, providing:
   - Standard layout management
   - Automatic dirty state tracking
   - Event binding for common controls
   - Helper methods for control creation

2. **ConfigDialog**: Manages the configuration window lifecycle:
   - Tab management for settings/help
   - Button row (OK/Cancel/Apply)
   - Window sizing and positioning

3. **Plugin Configure Method**: Each plugin implements a Configure method that:
   - Takes default parameters for fresh configuration
   - Returns configuration values when affirmed
   - Handles reconfiguration with existing values

### Rust Implementation Strategy

For the Rust implementation, we'll maintain similar patterns while leveraging Rust's type system:

```rust
pub trait PluginConfig {
    // Core configuration trait
    fn configure(&mut self) -> Option<ConfigResult>;
    fn create_config_panel(&self) -> ConfigPanel;
    fn validate_config(&self) -> Result<(), ConfigError>;
}

pub struct ConfigPanel {
    // Main configuration panel
    pub sizer: Box<dyn Sizer>,
    pub controls: Vec<Box<dyn Control>>,
    pub is_dirty: bool,
}

impl ConfigPanel {
    pub fn new() -> Self {
        // Initialize with standard layout
    }
    
    pub fn add_control(&mut self, control: Box<dyn Control>) {
        // Add control with standard spacing
    }
    
    pub fn bind_events(&mut self) {
        // Bind standard events (checkbox, button, etc)
    }
}
```

### Key Implementation Patterns

1. **Control Creation**:
   ```rust
   // Helper methods for common controls
   impl ConfigPanel {
       pub fn add_text_ctrl(&mut self, label: &str, value: &str) -> TextCtrl;
       pub fn add_checkbox(&mut self, label: &str, value: bool) -> Checkbox;
       pub fn add_choice(&mut self, label: &str, choices: &[String]) -> Choice;
   }
   ```

2. **Event Handling**:
   ```rust
   pub trait ConfigEventHandler {
       fn on_change(&mut self);
       fn on_validate(&self) -> bool;
       fn on_apply(&mut self) -> Result<(), ConfigError>;
   }
   ```

3. **Value Management**:
   ```rust
   pub trait ConfigValue {
       fn get_value(&self) -> Value;
       fn set_value(&mut self, value: Value) -> Result<(), ConfigError>;
       fn is_modified(&self) -> bool;
   }
   ```

### Migration Considerations

1. **GUI Framework Integration**:
   - Use egui for core UI components
   - Maintain native Windows controls where needed
   - Support custom rendering for complex controls

2. **State Management**:
   - Use Rust's type system for configuration validation
   - Implement proper error handling for configuration changes
   - Support undo/redo for configuration changes

3. **Plugin Interface**:
   - Keep configuration interface simple and familiar
   - Support both simple and complex configuration scenarios
   - Maintain backward compatibility with existing plugins

### Testing Strategy

1. **Unit Tests**:
   - Test configuration value validation
   - Verify control creation and layout
   - Check event handling and state management

2. **Integration Tests**:
   - Test complete configuration workflows
   - Verify plugin reconfiguration
   - Test error handling and recovery

3. **UI Tests**:
   - Verify layout and control rendering
   - Test user interaction patterns
   - Check accessibility features

### Platform Considerations

1. **Windows Integration**:
   - Support native Windows dialogs when needed
   - Handle Windows-specific UI patterns
   - Maintain Windows look and feel

2. **Cross-Platform Support**:
   - Abstract platform-specific UI code
   - Use portable UI patterns where possible
   - Support fallback rendering options

### Lessons Learned

1. **Keep It Simple**:
   - Maintain familiar configuration patterns
   - Avoid unnecessary complexity in UI layout
   - Use standard controls when possible

2. **Type Safety**:
   - Leverage Rust's type system for configuration
   - Implement proper error handling
   - Validate configuration values early

3. **Resource Management**:
   - Clean up resources properly
   - Handle window lifecycle correctly
   - Manage memory efficiently 

### Specific Plugin Configuration Examples

1. **Simple Text Configuration**:
```rust
impl Plugin for SimplePlugin {
    fn configure(&mut self) -> Option<ConfigResult> {
        let mut panel = ConfigPanel::new();
        
        // Add a simple text control
        let text_ctrl = panel.add_text_ctrl("Message:", &self.message);
        
        // Handle configuration result
        if panel.show_modal() {
            Some(ConfigResult::new()
                .with_value("message", text_ctrl.get_value()))
        } else {
            None
        }
    }
}
```

2. **Complex Grid Layout**:
```rust
impl Plugin for NetworkPlugin {
    fn configure(&mut self) -> Option<ConfigResult> {
        let mut panel = ConfigPanel::new();
        
        // Create a grid layout
        let grid = panel.create_grid(2, 5);
        
        // Add controls with proper alignment
        grid.add_row(vec![
            Control::Label("Host:"),
            Control::TextCtrl(self.host.clone()),
            Control::Label("Port:"),
            Control::NumberCtrl(self.port),
        ]);
        
        grid.add_row(vec![
            Control::Label("Protocol:"),
            Control::Choice(vec!["TCP", "UDP"], self.protocol_index),
        ]);
        
        // Add validation
        panel.add_validator(|values| {
            let port = values.get_number("port")?;
            if !(1..=65535).contains(&port) {
                return Err("Port must be between 1 and 65535".into());
            }
            Ok(())
        });
        
        if panel.show_modal() {
            Some(panel.get_results())
        } else {
            None
        }
    }
}
```

3. **Dynamic Controls**:
```rust
impl Plugin for DevicePlugin {
    fn configure(&mut self) -> Option<ConfigResult> {
        let mut panel = ConfigPanel::new();
        
        // Add device selection
        let devices = self.scan_devices();
        let device_choice = panel.add_choice("Device:", &devices);
        
        // Dynamic options based on device
        let options_panel = panel.add_collapsible_pane("Options");
        device_choice.on_change(move |choice| {
            options_panel.clear();
            if let Some(device) = devices.get(choice.get_selection()) {
                for option in device.get_options() {
                    options_panel.add_option_control(option);
                }
            }
        });
        
        if panel.show_modal() {
            Some(panel.get_results())
        } else {
            None
        }
    }
}
```

### Enhanced Testing Strategy

1. **Unit Testing Configuration Logic**:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_config_validation() {
        let mut plugin = NetworkPlugin::default();
        let config = ConfigResult::new()
            .with_value("port", 80)
            .with_value("host", "localhost");
            
        assert!(plugin.validate_config(&config).is_ok());
        
        let invalid_config = ConfigResult::new()
            .with_value("port", 0);
        assert!(plugin.validate_config(&invalid_config).is_err());
    }
    
    #[test]
    fn test_config_persistence() {
        let mut plugin = NetworkPlugin::default();
        let config = ConfigResult::new()
            .with_value("port", 80);
            
        plugin.apply_config(&config).unwrap();
        assert_eq!(plugin.get_config().get_number("port"), Some(80));
    }
}
```

2. **Integration Testing**:
```rust
#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_config_workflow() {
        let mut plugin = NetworkPlugin::default();
        
        // Simulate configuration dialog
        let config = simulate_config_dialog(&mut plugin, |panel| {
            panel.set_text_value("host", "localhost");
            panel.set_number_value("port", 8080);
            panel.click_ok();
        });
        
        assert!(config.is_some());
        let config = config.unwrap();
        
        // Verify plugin state after configuration
        assert_eq!(plugin.host, "localhost");
        assert_eq!(plugin.port, 8080);
        
        // Verify plugin functionality with new config
        assert!(plugin.start().is_ok());
        assert!(plugin.is_connected());
    }
}
```

3. **UI Tests**:
```rust
#[cfg(test)]
mod ui_tests {
    #[test]
    fn test_config_panel_layout() {
        let plugin = NetworkPlugin::default();
        let panel = plugin.create_config_panel();
        
        // Verify control placement
        assert_eq!(panel.get_control_count(), 4);
        assert!(panel.find_control("host").is_some());
        assert!(panel.find_control("port").is_some());
        
        // Test tab order
        let tab_order = panel.get_tab_order();
        assert_eq!(tab_order[0].get_name(), "host");
        assert_eq!(tab_order[1].get_name(), "port");
    }
    
    #[test]
    fn test_config_panel_events() {
        let plugin = NetworkPlugin::default();
        let mut panel = plugin.create_config_panel();
        
        // Test control events
        let host_ctrl = panel.find_control("host").unwrap();
        host_ctrl.set_value("localhost");
        assert!(panel.is_dirty());
        
        // Test validation
        assert!(panel.validate().is_ok());
    }
}
```

### Advanced GUI Framework Integration

1. **Hybrid Window Management**:
```rust
pub struct HybridWindow {
    // Core window state
    native_handle: HWND,
    egui_context: egui::Context,
    
    // Rendering state
    renderer: Renderer,
    surface: Surface,
    
    // Integration state
    native_controls: Vec<NativeControl>,
    egui_controls: Vec<EguiControl>,
    shared_state: Arc<Mutex<SharedState>>,
}

impl HybridWindow {
    pub fn new() -> Result<Self, Error> {
        // Create native window
        let native_handle = create_native_window()?;
        
        // Initialize egui
        let egui_context = egui::Context::new();
        
        // Setup rendering
        let renderer = Renderer::new(native_handle)?;
        let surface = Surface::new(&renderer)?;
        
        Ok(Self {
            native_handle,
            egui_context,
            renderer,
            surface,
            native_controls: Vec::new(),
            egui_controls: Vec::new(),
            shared_state: Arc::new(Mutex::new(SharedState::new())),
        })
    }
    
    pub fn render(&mut self) {
        // Update native controls
        for control in &mut self.native_controls {
            control.update()?;
        }
        
        // Render egui
        self.egui_context.run(|ctx| {
            egui::Window::new("Main")
                .show(ctx, |ui| {
                    for control in &mut self.egui_controls {
                        control.render(ui);
                    }
                });
        });
        
        // Composite final frame
        self.renderer.begin_frame();
        self.renderer.render_native_controls(&self.native_controls);
        self.renderer.render_egui(self.egui_context.output());
        self.renderer.end_frame();
    }
}
```

2. **Control Synchronization**:
```rust
pub struct SharedState {
    values: HashMap<String, Value>,
    dirty_flags: HashSet<String>,
}

impl SharedState {
    pub fn notify_change(&mut self, id: &str, value: Value) {
        self.values.insert(id.to_string(), value);
        self.dirty_flags.insert(id.to_string());
    }
}

// Native control wrapper
pub struct NativeControl {
    hwnd: HWND,
    id: String,
    shared_state: Arc<Mutex<SharedState>>,
}

impl NativeControl {
    pub fn update(&mut self) -> Result<(), Error> {
        let mut state = self.shared_state.lock().unwrap();
        if state.dirty_flags.contains(&self.id) {
            // Update native control from shared state
            if let Some(value) = state.values.get(&self.id) {
                self.set_native_value(value)?;
            }
            state.dirty_flags.remove(&self.id);
        }
        Ok(())
    }
    
    fn set_native_value(&self, value: &Value) -> Result<(), Error> {
        unsafe {
            match value {
                Value::Text(s) => {
                    SetWindowTextW(self.hwnd, wide_string(s));
                }
                Value::Number(n) => {
                    SendMessageW(self.hwnd, WM_SETTEXT, 0, n.to_string().as_ptr() as LPARAM);
                }
                Value::Bool(b) => {
                    SendMessageW(
                        self.hwnd,
                        BM_SETCHECK,
                        if *b { BST_CHECKED } else { BST_UNCHECKED } as WPARAM,
                        0,
                    );
                }
            }
        }
        Ok(())
    }
}

// Egui control wrapper
pub struct EguiControl {
    id: String,
    shared_state: Arc<Mutex<SharedState>>,
}

impl EguiControl {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        let mut state = self.shared_state.lock().unwrap();
        if let Some(value) = state.values.get(&self.id) {
            match value {
                Value::Text(s) => {
                    let mut text = s.clone();
                    if ui.text_edit_singleline(&mut text).changed() {
                        state.notify_change(&self.id, Value::Text(text));
                    }
                }
                Value::Number(n) => {
                    let mut num = *n;
                    if ui.add(egui::DragValue::new(&mut num)).changed() {
                        state.notify_change(&self.id, Value::Number(num));
                    }
                }
                Value::Bool(b) => {
                    let mut checked = *b;
                    if ui.checkbox(&mut checked, "").changed() {
                        state.notify_change(&self.id, Value::Bool(checked));
                    }
                }
            }
        }
    }
}
```

3. **Event Integration**:
```rust
pub struct EventBridge {
    native_window: HWND,
    egui_context: egui::Context,
    shared_state: Arc<Mutex<SharedState>>,
}

impl EventBridge {
    pub fn handle_native_event(&mut self, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> bool {
        match msg {
            WM_COMMAND => {
                let control_id = LOWORD(wparam as u32) as i32;
                if let Some(control) = self.find_native_control(control_id) {
                    self.handle_native_control_event(control, HIWORD(wparam as u32));
                    return true;
                }
            }
            WM_NOTIFY => {
                let nmhdr = unsafe { *(lparam as *const NMHDR) };
                self.handle_native_notification(&nmhdr);
                return true;
            }
            _ => {}
        }
        false
    }
    
    pub fn handle_egui_event(&mut self, event: &egui::Event) {
        match event {
            egui::Event::ValueChanged(id) => {
                if let Some(value) = self.egui_context.memory().data.get_temp(id) {
                    let mut state = self.shared_state.lock().unwrap();
                    state.notify_change(id, value);
                }
            }
            egui::Event::FocusGained(id) => {
                unsafe {
                    // Remove focus from native controls
                    SetFocus(null_mut());
                }
            }
            _ => {}
        }
    }
}
```

4. **Layout Management**:
```rust
pub struct HybridLayout {
    native_areas: Vec<NativeArea>,
    egui_areas: Vec<EguiArea>,
}

impl HybridLayout {
    pub fn layout(&mut self, rect: Rect) {
        // Calculate areas for native and egui controls
        let (native_rects, egui_rects) = self.calculate_layout(rect);
        
        // Position native controls
        for (area, rect) in self.native_areas.iter_mut().zip(native_rects) {
            unsafe {
                SetWindowPos(
                    area.hwnd,
                    null_mut(),
                    rect.x as i32,
                    rect.y as i32,
                    rect.width as i32,
                    rect.height as i32,
                    SWP_NOZORDER,
                );
            }
        }
        
        // Update egui areas
        for (area, rect) in self.egui_areas.iter_mut().zip(egui_rects) {
            area.rect = rect;
        }
    }
    
    fn calculate_layout(&self, rect: Rect) -> (Vec<Rect>, Vec<Rect>) {
        // Implement layout algorithm that divides space between
        // native and egui controls while maintaining proper alignment
        // and respecting minimum sizes
    }
}
```

5. **Style Integration**:
```rust
pub struct HybridStyle {
    native_theme: HTHEME,
    egui_visuals: egui::Visuals,
}

impl HybridStyle {
    pub fn new() -> Result<Self, Error> {
        // Get native Windows theme
        let native_theme = unsafe {
            OpenThemeData(null_mut(), wide_string("WINDOW"))
        };
        
        // Create matching egui visuals
        let mut egui_visuals = egui::Visuals::default();
        self.sync_colors_from_native(&mut egui_visuals, native_theme)?;
        
        Ok(Self {
            native_theme,
            egui_visuals,
        })
    }
    
    fn sync_colors_from_native(
        &self,
        visuals: &mut egui::Visuals,
        theme: HTHEME,
    ) -> Result<(), Error> {
        // Read colors from Windows theme
        let window_bg = self.get_theme_color(theme, WP_WINDOW, 0, TMT_FILLCOLOR)?;
        let text_color = self.get_theme_color(theme, WP_WINDOW, 0, TMT_TEXTCOLOR)?;
        
        // Update egui visuals to match
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(
            window_bg.r,
            window_bg.g,
            window_bg.b,
        );
        visuals.text_color = egui::Color32::from_rgb(
            text_color.r,
            text_color.g,
            text_color.b,
        );
        
        Ok(())
    }
} 

### Specific UI Component Implementations

1. **Log View Implementation**:
```rust
pub struct LogEntry {
    timestamp: DateTime<Local>,
    level: LogLevel,
    source: EventSource,
    text: String,
    details: Option<String>,
    highlight: bool,
    indent_level: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub enum EventSource {
    System,
    Plugin(String),
    Macro(String),
    Action(String),
}

pub struct LogView {
    entries: VecDeque<LogEntry>,
    colors: LogColors,
    filters: LogFilters,
    max_entries: usize,
    auto_scroll: bool,
    selected_entry: Option<usize>,
}

impl LogView {
    pub fn handle_event(&mut self, event: &EventGhostEvent) {
        // Create log entry from event
        let entry = LogEntry {
            timestamp: Local::now(),
            level: event.get_log_level(),
            source: event.source.clone(),
            text: event.to_string(),
            details: event.get_details(),
            highlight: event.should_highlight(),
            indent_level: event.get_indent_level(),
        };

        // Apply filters
        if self.filters.should_show(&entry) {
            // Maintain max entries limit
            if self.entries.len() >= self.max_entries {
                self.entries.pop_front();
            }
            self.entries.push_back(entry);
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        // Toolbar with filter controls
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.auto_scroll, "Auto-scroll");
            if ui.button("Clear").clicked() {
                self.entries.clear();
            }
            self.filters.ui(ui);
        });

        // Log entries with virtual scrolling
        egui::ScrollArea::vertical()
            .stick_to_bottom(self.auto_scroll)
            .show_rows(
                ui,
                ui.text_style_height(&TextStyle::Body),
                self.entries.len(),
                |ui, row_range| {
                    for i in row_range {
                        let entry = &self.entries[i];
                        self.render_entry(ui, entry, i);
                    }
                }
            );
    }

    fn render_entry(&mut self, ui: &mut egui::Ui, entry: &LogEntry, index: usize) {
        let indent = entry.indent_level as f32 * 20.0;
        
        ui.horizontal(|ui| {
            ui.add_space(indent);
            
            // Timestamp with optional highlight
            let timestamp = entry.timestamp.format(&self.timestamp_format);
            if entry.highlight {
                ui.colored_label(self.colors.highlight_bg, timestamp);
            } else {
                ui.label(timestamp);
            }

            // Source indicator
            match &entry.source {
                EventSource::System => {
                    ui.colored_label(self.colors.system_event, "⚙");
                }
                EventSource::Plugin(name) => {
                    ui.colored_label(self.colors.plugin_event, "🔌");
                    ui.label(name);
                }
                EventSource::Macro(name) => {
                    ui.colored_label(self.colors.macro_event, "📜");
                    ui.label(name);
                }
                EventSource::Action(name) => {
                    ui.colored_label(self.colors.action_event, "▶");
                    ui.label(name);
                }
            }

            // Main event text with appropriate color
            let text_color = match entry.level {
                LogLevel::Error => self.colors.error_event,
                LogLevel::Warning => self.colors.warning_event,
                _ => self.colors.normal_text,
            };
            let response = ui.colored_label(text_color, &entry.text)
                .interact(egui::Sense::click());

            // Handle selection
            if response.clicked() {
                self.selected_entry = Some(index);
            }
        });

        // Show details if selected
        if Some(index) == self.selected_entry {
            if let Some(details) = &entry.details {
                ui.indent("details", |ui| {
                    ui.label(details);
                });
            }
        }
    }
}

#[derive(Debug)]
pub struct LogFilters {
    show_debug: bool,
    show_system: bool,
    show_plugins: bool,
    show_macros: bool,
    text_filter: String,
}

impl LogFilters {
    pub fn should_show(&self, entry: &LogEntry) -> bool {
        // Level filtering
        if !self.show_debug && entry.level == LogLevel::Debug {
            return false;
        }

        // Source filtering
        match &entry.source {
            EventSource::System if !self.show_system => return false,
            EventSource::Plugin(_) if !self.show_plugins => return false,
            EventSource::Macro(_) if !self.show_macros => return false,
            _ => {}
        }

        // Text filtering
        if !self.text_filter.is_empty() {
            if !entry.text.to_lowercase().contains(&self.text_filter.to_lowercase()) {
                return false;
            }
        }

        true
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.show_debug, "Debug");
        ui.checkbox(&mut self.show_system, "System");
        ui.checkbox(&mut self.show_plugins, "Plugins");
        ui.checkbox(&mut self.show_macros, "Macros");
        ui.text_edit_singleline(&mut self.text_filter)
            .on_hover_text("Filter log entries");
    }
}

#[derive(Debug, Clone)]
struct LogColors {
    highlight_bg: Color32,
    system_event: Color32,
    plugin_event: Color32,
    macro_event: Color32,
    action_event: Color32,
    error_event: Color32,
    warning_event: Color32,
    normal_text: Color32,
}
```

2. **Tree View with Icons**:
```rust
pub struct ConfigTreeView {
    root: TreeNode,
    icons: IconMap,
    drag_state: Option<DragState>,
}

impl ConfigTreeView {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            self.render_node(ui, &mut self.root, 0);
        });
    }
    
    fn render_node(&mut self, ui: &mut egui::Ui, node: &mut TreeNode, depth: u32) {
        let indent = depth * 20.0;
        ui.horizontal(|ui| {
            ui.add_space(indent);
            
            // Render expand/collapse icon
            if node.has_children() {
                if ui.add(IconButton::new(
                    if node.expanded {
                        self.icons.collapse
                    } else {
                        self.icons.expand
                    }
                )).clicked() {
                    node.expanded = !node.expanded;
                }
            }
            
            // Render node icon and label
            ui.add(IconButton::new(node.get_icon(&self.icons)));
            let label = ui.add(
                egui::Label::new(&node.name)
                    .sense(Sense::click())
            );
            
            // Handle selection
            if label.clicked() {
                self.selected = Some(node.id);
            }
            
            // Handle drag and drop
            if label.dragged() {
                self.drag_state = Some(DragState::new(node.id));
            }
        });
        
        // Render children if expanded
        if node.expanded {
            for child in &mut node.children {
                self.render_node(ui, child, depth + 1);
            }
        }
    }
}
```

3. **Toolbar Implementation**:
```rust
pub struct Toolbar {
    buttons: Vec<ToolbarButton>,
    separator_indices: HashSet<usize>,
}

impl Toolbar {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (idx, button) in self.buttons.iter_mut().enumerate() {
                if self.separator_indices.contains(&idx) {
                    ui.add(egui::Separator::vertical());
                }
                
                let response = ui.add(
                    IconButton::new(button.icon)
                        .enabled(button.enabled)
                        .tooltip(button.tooltip.as_str())
                );
                
                if response.clicked() {
                    button.action.execute();
                }
            }
        });
    }
}

#[derive(Clone)]
struct ToolbarButton {
    icon: Icon,
    enabled: bool,
    tooltip: String,
    action: Action,
}
```

4. **Status Bar with Multiple Sections**:
```rust
pub struct StatusBar {
    sections: Vec<StatusSection>,
    progress: Option<ProgressInfo>,
}

impl StatusBar {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Main status text
            ui.label(&self.sections[0].text);
            
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                // Optional progress bar
                if let Some(progress) = &self.progress {
                    ui.add(
                        egui::ProgressBar::new(progress.value)
                            .show_percentage()
                            .animate(true)
                    );
                }
                
                // Right-aligned sections
                for section in self.sections.iter().skip(1) {
                    ui.add(egui::Separator::vertical());
                    if let Some(icon) = &section.icon {
                        ui.add(IconButton::new(*icon));
                    }
                    ui.label(&section.text);
                }
            });
        });
    }
}
```

5. **Plugin Configuration Dialog**:
```rust
pub struct PluginConfigDialog {
    tabs: Vec<ConfigTab>,
    description: String,
    button_row: ButtonRow,
}

impl PluginConfigDialog {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::bottom("buttons")
            .show(ui, |ui| self.button_row.render(ui));
            
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                // Left side: Settings tabs
                ui.vertical(|ui| {
                    for tab in &mut self.tabs {
                        if ui.selectable_label(
                            tab.selected,
                            &tab.name
                        ).clicked() {
                            self.select_tab(tab.id);
                        }
                    }
                });
                
                ui.separator();
                
                // Right side: Current tab content
                if let Some(tab) = self.get_selected_tab() {
                    tab.render(ui);
                }
            });
            
            // Bottom: Description panel
            if !self.description.is_empty() {
                ui.separator();
                ui.add(
                    egui::TextEdit::multiline(&mut self.description.as_str())
                        .desired_width(f32::INFINITY)
                        .desired_rows(3)
                        .read_only(true)
                );
            }
        });
    }
}
```

These implementations specifically address the UI patterns visible in the EventGhost screenshot, including:
- Yellow highlighting in the log view
- Tree view with proper indentation and icons
- Toolbar with icon buttons and separators
- Multi-section status bar
- Plugin configuration with tabs and description panel

### Event and Macro Drag and Drop

1. **Drag State Management**:
```rust
#[derive(Debug)]
pub struct DragState {
    source_type: DragSourceType,
    source_id: String,
    preview_data: DragPreviewData,
    drop_target: Option<DropTarget>,
}

#[derive(Debug, Clone, Copy)]
pub enum DragSourceType {
    Event,
    Macro,
    Action,
    Plugin,
}

#[derive(Debug)]
struct DragPreviewData {
    text: String,
    icon: Icon,
    color: Color32,
}

#[derive(Debug)]
struct DropTarget {
    target_id: String,
    position: DropPosition,
    valid: bool,
}

#[derive(Debug, Clone, Copy)]
enum DropPosition {
    Before,
    After,
    Inside,
}
```

2. **Tree View Drag and Drop Implementation**:
```rust
impl ConfigTreeView {
    fn handle_drag_and_drop(&mut self, ui: &mut Ui) {
        // Start drag operation
        if let Some(response) = ui.memory().drag_and_drop.active {
            if response.dragged_by(egui::PointerButton::Primary) {
                if self.drag_state.is_none() {
                    if let Some(node) = self.find_node_at(response.hover_pos()) {
                        self.drag_state = Some(DragState {
                            source_type: node.get_type(),
                            source_id: node.id.clone(),
                            preview_data: node.get_preview_data(),
                            drop_target: None,
                        });
                    }
                }
            }
        }

        // Update drag preview
        if let Some(drag_state) = &mut self.drag_state {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            
            // Draw drag preview near cursor
            let preview_rect = ui.cursor().rect.translate(vec2(15.0, 15.0));
            ui.painter().rect_filled(
                preview_rect,
                2.0,
                drag_state.preview_data.color.linear_multiply(0.1),
            );
            ui.painter().image(
                drag_state.preview_data.icon.id(),
                preview_rect,
                Rect::from_min_size(pos2(0.0, 0.0), vec2(1.0, 1.0)),
                Color32::WHITE,
            );
            ui.painter().text(
                preview_rect.right_center(),
                Align2::LEFT_CENTER,
                &drag_state.preview_data.text,
                TextStyle::Body.resolve(ui.style()),
                drag_state.preview_data.color,
            );

            // Find and validate drop target
            if let Some(target) = self.find_drop_target(ui.cursor().hover_pos()) {
                drag_state.drop_target = Some(target);
            }
        }

        // Handle drop
        if ui.memory().drag_and_drop.released {
            if let Some(drag_state) = self.drag_state.take() {
                if let Some(target) = drag_state.drop_target {
                    if target.valid {
                        self.handle_drop(drag_state, target);
                    }
                }
            }
        }
    }

    fn find_drop_target(&self, pos: Pos2) -> Option<DropTarget> {
        if let Some(node) = self.find_node_at(pos) {
            // Determine drop position based on relative Y position
            let node_rect = node.rect;
            let relative_y = (pos.y - node_rect.top()) / node_rect.height();
            
            let position = match relative_y {
                y if y < 0.25 => DropPosition::Before,
                y if y > 0.75 => DropPosition::After,
                _ => DropPosition::Inside,
            };

            // Validate drop target based on source type
            let valid = match self.drag_state.as_ref().unwrap().source_type {
                DragSourceType::Event => {
                    // Events can only be dropped into macros
                    node.node_type == NodeType::Macro && position == DropPosition::Inside
                }
                DragSourceType::Macro => {
                    // Macros can be dropped before/after other macros or inside folders
                    match (node.node_type, position) {
                        (NodeType::Macro, DropPosition::Before | DropPosition::After) => true,
                        (NodeType::Folder, DropPosition::Inside) => true,
                        _ => false,
                    }
                }
                DragSourceType::Action => {
                    // Actions can only be dropped inside macros
                    node.node_type == NodeType::Macro && position == DropPosition::Inside
                }
                DragSourceType::Plugin => {
                    // Plugins can't be dragged
                    false
                }
            };

            Some(DropTarget {
                target_id: node.id.clone(),
                position,
                valid,
            })
        } else {
            None
        }
    }

    fn handle_drop(&mut self, drag_state: DragState, target: DropTarget) {
        match drag_state.source_type {
            DragSourceType::Event => {
                // Create new action from event
                let action = Action::from_event(&drag_state.source_id);
                self.add_action_to_macro(&target.target_id, action);
            }
            DragSourceType::Macro => {
                match target.position {
                    DropPosition::Before | DropPosition::After => {
                        // Reorder macro
                        self.move_macro(&drag_state.source_id, &target.target_id, target.position);
                    }
                    DropPosition::Inside => {
                        // Move macro into folder
                        self.move_macro_to_folder(&drag_state.source_id, &target.target_id);
                    }
                }
            }
            DragSourceType::Action => {
                // Move action between macros
                self.move_action(&drag_state.source_id, &target.target_id);
            }
            _ => {}
        }
    }

    fn render_drop_indicator(&self, ui: &mut Ui) {
        if let Some(drag_state) = &self.drag_state {
            if let Some(target) = &drag_state.drop_target {
                if target.valid {
                    if let Some(node) = self.find_node_by_id(&target.target_id) {
                        let rect = match target.position {
                            DropPosition::Before => {
                                Rect::from_min_max(
                                    pos2(node.rect.left(), node.rect.top() - 2.0),
                                    pos2(node.rect.right(), node.rect.top() + 2.0),
                                )
                            }
                            DropPosition::After => {
                                Rect::from_min_max(
                                    pos2(node.rect.left(), node.rect.bottom() - 2.0),
                                    pos2(node.rect.right(), node.rect.bottom() + 2.0),
                                )
                            }
                            DropPosition::Inside => {
                                node.rect.expand(2.0)
                            }
                        };

                        ui.painter().rect_filled(
                            rect,
                            2.0,
                            Color32::from_rgb(0, 120, 215),
                        );
                    }
                }
            }
        }
    }
}
```

3. **Event to Macro Conversion**:
```rust
impl Action {
    pub fn from_event(event_id: &str) -> Self {
        // Create a new action that triggers when the event occurs
        Self {
            id: Uuid::new_v4().to_string(),
            name: format!("On {}", event_id),
            event_filter: Some(EventFilter {
                event_id: event_id.to_string(),
                conditions: Vec::new(),
            }),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventFilter {
    event_id: String,
    conditions: Vec<FilterCondition>,
}

#[derive(Debug, Clone)]
pub enum FilterCondition {
    Equals(String, String),
    Contains(String, String),
    Regex(String, Regex),
}
```

This implementation provides:
- Drag state management for different types of draggable items
- Visual feedback during drag operations with preview
- Drop target validation based on source and target types
- Drop position detection (before/after/inside)
- Visual indicators for valid drop targets
- Automatic conversion of events to actions when dropped
- Support for macro reordering and folder organization

// ... existing code ... 
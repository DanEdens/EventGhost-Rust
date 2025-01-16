# EventGhost GUI Architecture

## Original Implementation Analysis

EventGhost's GUI is built on wxPython with a sophisticated multi-pane layout:

1. **Main Window Components**
   - Tree Control (macro/event tree)
   - Log Control (event log)
   - Status Bar (status info and options)
   - Tool Bar (common actions)
   - Menu Bar (all available actions)

2. **Core Features**
   - AUI (Advanced User Interface) management
   - Drag and drop support
   - Persistent window layout
   - Custom tree control
   - Configurable logging display

3. **Plugin Integration**
   ```python
   class PluginItem(ActionItem):
       def __init__(self, parent, node):
           # Plugin initialization
           self.info = info = eg.pluginManager.OpenPlugin(
               guid,
               evalName,
               args,
               self,
           )
           self.name = eg.text.General.pluginLabel % info.label
           self.executable = info.instance
   ```

4. **Event System**
   ```python
   class LogCtrl(wx.ListCtrl):
       def __init__(self):
           # Event logging setup
           self.logDates = True
           self.logTimes = True
           self.data = collections.deque()
           eg.log.SetCtrl(self)
           
       def OnGetItemTextWithDT(self, item, dummyColumn):
           # Format log entries with timestamps
           line, _, _, when, indent = self.data[item]
           return strftime(self._datetime_fmt, localtime(when)) + indent + line
   ```

5. **Window Management**
   ```python
   class MainFrame(wx.Frame):
       def __init__(self, document):
           # Core window setup
           self.auiManager = wx.aui.AuiManager()
           self.treeCtrl = self.CreateTreeCtrl()
           self.logCtrl = self.CreateLogCtrl()
           
       def CreateTreeCtrl(self):
           # Tree control initialization
           treeCtrl = TreeCtrl(self, document=self.document)
           self.auiManager.AddPane(
               treeCtrl,
               wx.aui.AuiPaneInfo().
               Name("tree").
               Center()
           )
   ```

## Rust Implementation Strategy

### Framework Selection: egui + native-windows-gui

We'll use a hybrid approach:
1. egui for immediate mode UI elements
2. native-windows-gui for native Windows controls
3. Custom rendering for tree control

### Core Architecture

```rust
pub struct MainWindow {
    // Core state
    document: Document,
    config: Config,
    
    // UI Components
    tree_view: TreeView,
    log_view: LogView,
    toolbar: ToolBar,
    status_bar: StatusBar,
    
    // Layout
    layout: Layout,
    window_state: WindowState,
}

impl MainWindow {
    pub fn new(document: Document) -> Result<Self, Error> {
        // Initialize window and components
    }
    
    pub fn update(&mut self, ctx: &Context) {
        // Update layout and components
        self.layout.update(ctx);
        self.tree_view.update(ctx);
        self.log_view.update(ctx);
        self.toolbar.update(ctx);
        self.status_bar.update(ctx);
        
        // Handle events
        self.handle_events();
    }
}

// Plugin Integration
pub struct PluginView {
    info: PluginInfo,
    instance: Box<dyn Plugin>,
    config_window: Option<ConfigWindow>,
}

impl PluginView {
    pub fn show_config(&mut self, ctx: &Context) {
        if let Some(window) = &mut self.config_window {
            window.show(ctx);
        }
    }
}

// Event System
pub struct LogView {
    entries: VecDeque<LogEntry>,
    config: LogConfig,
    scroll: ScrollArea,
}

impl LogView {
    pub fn add_entry(&mut self, entry: LogEntry) {
        self.entries.push_back(entry);
        if self.entries.len() > self.config.max_entries {
            self.entries.pop_front();
        }
    }
    
    pub fn show(&mut self, ctx: &Context) {
        self.scroll.show(ctx, |ui| {
            for entry in &self.entries {
                entry.show(ui);
            }
        });
    }
}
```

### Key Differences from Original

1. **Layout Management**
   - Custom layout system instead of AUI
   - More efficient immediate mode rendering
   - Better state management

2. **Tree Control**
   - Custom tree implementation
   - Hardware-accelerated rendering
   - More efficient node management

3. **Event System**
   - Strongly typed events
   - Better state synchronization
   - More predictable updates

4. **Resource Management**
   - RAII-based cleanup
   - Better memory efficiency
   - Explicit state ownership

5. **Plugin Integration**
   - Type-safe plugin interface
   - Async-aware plugin loading
   - Sandboxed plugin execution

### Migration Strategy

1. **Phase 1: Core Windows**
   - Basic window management
   - Native title bar and menus
   - Status bar integration

2. **Phase 2: Tree Control**
   - Custom tree implementation
   - Drag and drop support
   - Node editing

3. **Phase 3: Log View**
   - Efficient log rendering
   - Search functionality
   - Filtering system

4. **Phase 4: Integration**
   - Plugin UI integration
   - Configuration system
   - State persistence

### Testing Strategy

1. **Unit Tests**
   - Component behavior
   - Event handling
   - State management

2. **Integration Tests**
   - Window interaction
   - Layout persistence
   - Plugin integration

3. **Performance Tests**
   - Rendering efficiency
   - Memory usage
   - State updates

### Platform Considerations

1. **Windows Integration**
   - Native window frame
   - System theme support
   - HiDPI support

2. **Performance**
   - Hardware acceleration
   - Efficient state updates
   - Memory optimization

### Lessons Learned
1. **Keep Plugin UI Simple**
   - Follow EventGhost's straightforward UI patterns
   - Avoid over-engineering async patterns
   - Use direct event handling where possible

2. **Maintain Familiar Workflow**
   - Preserve EventGhost's intuitive interface
   - Keep plugin configuration dialogs simple
   - Focus on reliability over complexity

3. **Resource Management**
   - Use Rust's ownership model effectively
   - Clean up resources deterministically
   - Avoid complex async resource management 
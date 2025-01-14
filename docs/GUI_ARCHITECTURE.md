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

3. **Key Classes**
   ```python
   class MainFrame(wx.Frame):
       def __init__(self, document):
           # Core components
           self.treeCtrl = self.CreateTreeCtrl()
           self.logCtrl = self.CreateLogCtrl()
           self.toolBar = self.CreateToolBar()
           self.menuBar = self.CreateMenuBar()
           self.statusBar = StatusBar(self)
           
           # Layout management
           self.auiManager = wx.aui.AuiManager()
           
           # Event binding
           self.Bind(wx.EVT_SIZE, self.OnSize)
           self.Bind(wx.EVT_MOVE, self.OnMove)
           self.Bind(wx.aui.EVT_AUI_PANE_CLOSE, self.OnPaneClose)
   ```

4. **Tree Control Implementation**
   ```python
   class TreeCtrl(wx.TreeCtrl):
       def __init__(self, parent, document):
           # Drag and drop support
           self.SetDropTarget(DropTarget(self))
           
           # Custom drawing
           self.SetInsertMark()
           self.ClearInsertMark()
           
           # Node management
           self.CreateTreeItem()
           self.EditNodeLabel()
           self.ExpandAll()
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
        // Update layout
        self.layout.update(ctx);
        
        // Update components
        self.tree_view.update(ctx);
        self.log_view.update(ctx);
        self.toolbar.update(ctx);
        self.status_bar.update(ctx);
        
        // Handle events
        self.handle_events();
    }
    
    fn handle_events(&mut self) {
        while let Some(event) = self.event_queue.pop() {
            match event {
                Event::NodeSelected(node) => self.on_node_selected(node),
                Event::NodeChanged(node) => self.on_node_changed(node),
                Event::ConfigChanged(config) => self.on_config_changed(config),
                // ... other events
            }
        }
    }
}

pub struct TreeView {
    // Tree state
    root: Node,
    selected: Option<NodeId>,
    drag_state: Option<DragState>,
    
    // Visual state
    scroll_offset: Vec2,
    visible_range: Range<usize>,
    
    // Rendering
    node_cache: NodeCache,
    layout_cache: LayoutCache,
}

impl TreeView {
    pub fn update(&mut self, ctx: &Context) {
        // Handle input
        self.handle_input(ctx);
        
        // Update layout
        self.update_layout(ctx);
        
        // Render visible nodes
        self.render_nodes(ctx);
    }
    
    fn handle_input(&mut self, ctx: &Context) {
        // Handle mouse/keyboard input
        if let Some(pos) = ctx.input().pointer.hover_pos() {
            self.handle_hover(pos);
        }
        
        if ctx.input().pointer.any_pressed() {
            self.handle_click(ctx);
        }
    }
    
    fn render_nodes(&self, ctx: &Context) {
        // Efficient node rendering
        for node in self.visible_nodes() {
            self.render_node(ctx, node);
        }
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
use windows::Win32::Foundation::{HWND, HINSTANCE, WPARAM, LPARAM};
use windows::Win32::UI::Controls::{
    TVS_HASLINES, TVS_LINESATROOT, TVS_HASBUTTONS, TVS_SHOWSELALWAYS,
    TVS_EX_DOUBLEBUFFER, TVM_SETEXTENDEDSTYLE,
};
use windows::Win32::UI::WindowsAndMessaging::{WS_CHILD, WS_VISIBLE, ShowWindow, SW_SHOW, SW_HIDE, SendMessageA};
use crate::core::Error;
use crate::win32;
use super::UIComponent;
use gtk::prelude::*;
use gtk::{self, TreeView, TreeStore, TreeViewColumn, CellRendererText};
use glib;

pub struct TreeItem {
    pub id: String,
    pub text: String,
    pub icon_index: i32,
    pub selected_icon_index: i32,
    pub data: Option<Box<dyn std::any::Any + Send + Sync>>,
}

pub struct TreeCtrl {
    pub widget: TreeView,
    store: TreeStore,
    hwnd: HWND,
    parent: HWND,
    instance: HINSTANCE,
    is_visible: bool,
}

impl TreeCtrl {
    pub fn new(parent: HWND, instance: HINSTANCE) -> Result<Self, Error> {
        // Create tree store with columns
        let store = TreeStore::new(&[
            glib::Type::STRING, // Name
            glib::Type::STRING, // Icon
            glib::Type::BOOL,   // Enabled
            glib::Type::STRING, // Type
        ]);

        // Create tree view
        let widget = TreeView::new();
        widget.set_model(Some(&store));
        
        // Add columns
        let renderer = CellRendererText::new();
        let column = TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", 0);
        column.set_title("Name");
        widget.append_column(&column);
        
        // Enable selection
        let selection = widget.selection();
        selection.set_mode(gtk::SelectionMode::Single);
        
        // Enable drag and drop
        widget.set_reorderable(true);
        
        Ok(Self {
            widget,
            store,
            hwnd: HWND::default(),
            parent,
            instance,
            is_visible: false,
        })
    }

    pub fn initialize(&mut self) -> Result<(), Error> {
        // Create the tree control window
        let hwnd = win32::create_window(
            "SysTreeView32\0",
            "",
            WS_CHILD | WS_VISIBLE | TVS_HASLINES | TVS_LINESATROOT | TVS_HASBUTTONS | TVS_SHOWSELALWAYS,
            0,
            0,
            0,
            0,
            Some(self.parent),
            self.instance,
        )?;

        self.hwnd = hwnd;
        self.is_visible = true;

        // Set extended styles
        unsafe {
            SendMessageA(
                self.hwnd,
                TVM_SETEXTENDEDSTYLE,
                WPARAM(TVS_EX_DOUBLEBUFFER as usize),
                LPARAM(TVS_EX_DOUBLEBUFFER as isize),
            );
        }

        Ok(())
    }

    pub fn add_item(&mut self, parent: Option<&TreeItem>, item: TreeItem) -> Result<TreeItem, Error> {
        todo!()
    }

    pub fn remove_item(&mut self, item: &TreeItem) -> Result<(), Error> {
        todo!()
    }

    pub fn get_selected_item(&self) -> Result<Option<TreeItem>, Error> {
        todo!()
    }

    pub fn expand_item(&self, iter: &gtk::TreeIter) {
        if let Some(path) = self.store.path(iter) {
            self.widget.expand_row(&path, false);
        }
    }

    pub fn collapse_item(&self, iter: &gtk::TreeIter) {
        if let Some(path) = self.store.path(iter) {
            self.widget.collapse_row(&path);
        }
    }

    pub fn set_image_list(&mut self, image_list: HWND) -> Result<(), Error> {
        todo!()
    }

    pub fn ensure_visible(&mut self, item: &TreeItem) -> Result<(), Error> {
        todo!()
    }

    pub fn append_item(&self, parent: Option<&gtk::TreeIter>, name: &str, item_type: &str) -> gtk::TreeIter {
        self.store.append(parent)
    }
    
    pub fn set_item_values(&self, iter: &gtk::TreeIter, name: &str, icon: &str, enabled: bool, item_type: &str) {
        self.store.set(iter, &[
            (0, &name),
            (1, &icon),
            (2, &enabled),
            (3, &item_type),
        ]);
    }
    
    pub fn get_selection(&self) -> Option<gtk::TreeIter> {
        let selection = self.widget.selection();
        if let Some((model, iter)) = selection.selected() {
            Some(iter)
        } else {
            None
        }
    }
    
    pub fn expand_all(&self) {
        self.widget.expand_all();
    }
    
    pub fn collapse_all(&self) {
        self.widget.collapse_all();
    }
}

impl UIComponent for TreeCtrl {
    fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    fn show(&mut self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
        self.is_visible = true;
        Ok(())
    }

    fn hide(&mut self) -> Result<(), Error> {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
        self.is_visible = false;
        Ok(())
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }
} 
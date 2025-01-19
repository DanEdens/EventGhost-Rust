use windows::Win32::Foundation::{HWND, HINSTANCE};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::Controls::*;
use crate::core::Error;
use crate::win32;
use super::UIComponent;

pub struct TreeItem {
    pub id: String,
    pub text: String,
    pub icon_index: i32,
    pub selected_icon_index: i32,
    pub data: Option<Box<dyn std::any::Any + Send + Sync>>,
}

pub struct TreeCtrl {
    hwnd: HWND,
    parent: HWND,
    instance: HINSTANCE,
    is_visible: bool,
}

impl TreeCtrl {
    pub fn new(parent: HWND, instance: HINSTANCE) -> Result<Self, Error> {
        Ok(Self {
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

    pub fn expand_item(&mut self, item: &TreeItem) -> Result<(), Error> {
        todo!()
    }

    pub fn collapse_item(&mut self, item: &TreeItem) -> Result<(), Error> {
        todo!()
    }

    pub fn set_image_list(&mut self, image_list: HWND) -> Result<(), Error> {
        todo!()
    }

    pub fn ensure_visible(&mut self, item: &TreeItem) -> Result<(), Error> {
        todo!()
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
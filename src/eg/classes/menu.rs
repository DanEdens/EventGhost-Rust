use gtk::prelude::*;
use gtk::{self, Box, MenuButton, PopoverMenu, gio};
use super::UIComponent;

pub struct Menu {
    pub widget: Box,
    pub menu_button: MenuButton,
    pub menu: gio::Menu,
}

impl Menu {
    pub fn new() -> Self {
        let widget = Box::new(gtk::Orientation::Vertical, 0);
        let menu_button = MenuButton::new();
        let menu = gio::Menu::new();
        
        let popover = PopoverMenu::from_model(Some(&menu));
        menu_button.set_popover(Some(&popover));
        
        widget.append(&menu_button);
        
        Menu {
            widget,
            menu_button,
            menu,
        }
    }
    
    pub fn append_menu(&self, label: &str) -> gio::Menu {
        let submenu = gio::Menu::new();
        let menu_item = gio::MenuItem::new(Some(label), None);
        menu_item.set_submenu(Some(&submenu));
        self.menu.append_item(&menu_item);
        submenu
    }
    
    pub fn append_item(&self, menu: &gio::Menu, label: &str, action: Option<&str>) {
        let item = gio::MenuItem::new(Some(label), action);
        menu.append_item(&item);
    }
}

impl UIComponent for Menu {
    fn get_widget(&self) -> &gtk::Widget {
        self.widget.upcast_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_menu_initialization() {
        gtk::init().expect("Failed to initialize GTK");
        
        let menu = Menu::new();
        assert!(menu.widget.is_visible());
    }
} 
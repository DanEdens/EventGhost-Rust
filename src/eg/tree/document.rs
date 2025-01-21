use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use crate::core::Error;
use super::root::Root;
use super::item::TreeItem;

#[derive(Debug)]
pub struct Document {
    root: Arc<RwLock<Root>>,
    file_path: Option<PathBuf>,
    is_modified: bool,
}

impl Document {
    pub fn new() -> Self {
        Self {
            root: Arc::new(RwLock::new(Root::new())),
            file_path: None,
            is_modified: false,
        }
    }

    pub fn get_root(&self) -> Arc<RwLock<Root>> {
        Arc::clone(&self.root)
    }

    pub fn set_file_path(&mut self, path: Option<PathBuf>) {
        self.file_path = path;
    }

    pub fn get_file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    pub fn set_modified(&mut self, modified: bool) {
        self.is_modified = modified;
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(path) = &self.file_path {
            // TODO: Implement save logic
            println!("Saving document to {}", path.display());
            self.is_modified = false;
            Ok(())
        } else {

            Err(Error::Tree("No file path set".into()))
        }
    }

    pub fn save_as(&mut self, path: PathBuf) -> Result<(), Error> {
        self.file_path = Some(path);
        self.save()
    }

    pub fn load(&mut self, path: PathBuf) -> Result<(), Error> {
        // TODO: Implement load logic
        self.file_path = Some(path);
        self.is_modified = false;
        Ok(())
    }

    pub fn find_item(&self, id: uuid::Uuid) -> Option<Arc<RwLock<dyn TreeItem>>> {
        if let Ok(root) = self.root.read() {
            root.find_item(id)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        if let Ok(mut root) = self.root.write() {
            *root = Root::new();
        }
        self.file_path = None;
        self.is_modified = false;
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
} 
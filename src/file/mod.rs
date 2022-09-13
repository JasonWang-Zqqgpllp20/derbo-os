use alloc::boxed::Box;
use alloc::vec::Vec;

pub mod file_system;
pub mod initial;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FileType {
    Document,
    Folder,
}

#[derive(Debug, Clone)]
pub struct FileNode {
    file_name: Vec<char>,
    file_type: FileType,
    access: bool,
    children: Vec<Box<FileNode>>,
    content: Vec<Vec<char>>,
}

impl FileNode {
    pub fn new(name: Vec<char>, file_type: FileType) -> FileNode {
        FileNode {
            file_name: name,
            file_type: file_type,
            access: true,
            children: Vec::new(),
            content: Vec::new(),
        }
    }
    pub fn update_name(&mut self, name: Vec<char>) {
        self.file_name = name;
    }
    pub fn add_child(&mut self, child: FileNode) -> Result<(), ()> {
        match &self.file_type {
            FileType::Folder => {
                self.children.push(Box::new(child));
                return Ok(());
            },
            FileType::Document => {
                return Err(())
            }
        }
    }
    pub fn change_access(&mut self) {
        if self.access == true {
            self.access = false;
        } else {
            self.access = true;
        }
    }
    pub fn edit_content(&mut self, content: Vec<Vec<char>>) {
        self.content = content;
    }
    pub fn get_access(&mut self) -> bool {
        self.access
    }
    pub fn get_content(&mut self) -> Vec<Vec<char>> {
        self.content.clone()
    }
}
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use crate::{print, println};
use crate::api::{str2char, char_vec_cmp};
use super::{FileNode, FileType};
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct FileSystem {
    path: Vec<Vec<char>>,
    root: FileNode,
}

impl FileSystem {
    pub fn new() -> FileSystem {           // create a new root
        let root: FileNode = FileNode::new(str2char("root"), FileType::Folder);
        FileSystem {
            path: vec![str2char("root")],
            root: root,
        }
    }

    pub fn init(&mut self) {
        use super::initial;
        initial::init(self);
    }

    pub fn add_file(&mut self, file_input: FileNode) -> Result<(), ()> {
        let name_input = file_input.file_name.clone();
        let mut found = false;

        match self.retrieve(name_input) {
            Ok((file, _)) => {
                if file.file_type != file_input.file_type {
                    found = true;
                }
            },
            Err(_) => {},
        }

        if found == true {      // there is an file named 'xxx' exists
            return Err(());
        }

        let index_folder = self.index_folder();
        let mut current = self.root.clone();
        let mut vec_filenode: Vec<FileNode> = vec![self.root.clone()];  // the whole stucture of every single subtree

        for index in &index_folder {
            current = Box::into_inner(current.children[*index].clone());
            vec_filenode.push(current.clone());
        }

        let len = vec_filenode.len();
        vec_filenode[len-1].children.push(Box::new(file_input.clone()));
        
        for i in 0..len-1 {                                             // backpropagation to update the root
            let node = Box::new(vec_filenode[len-1-i].clone());
            vec_filenode[len-1-i-1].children[index_folder[len-i-2]] = node;
        }

        self.root = vec_filenode[0].clone();

        return Ok(());
    }

    pub fn remove_file(&mut self, file_input: FileNode) -> Result<(), ()> {
        let name_input = file_input.file_name.clone();
        let mut found = false;

        match self.retrieve(name_input.clone()) {
            Ok((file, _)) => {
                if file.file_type == file_input.file_type {
                    found = true;
                }
            },
            Err(_) => {},
        }
        
        if found == false {      // there is no file named 'xxx'
            return Err(());
        }

        let index_folder = self.index_folder();
        let mut current = self.root.clone();
        let mut vec_filenode: Vec<FileNode> = vec![self.root.clone()];  // the whole stucture of every single subtree

        for index in &index_folder {
            current = Box::into_inner(current.children[*index].clone());
            vec_filenode.push(current.clone());
        }

        let len = vec_filenode.len();
        let len_children = vec_filenode[len-1].children.len();
        let mut index = 0;
        
        for f in &vec_filenode[len-1].children {
            if f.file_name == name_input {
                break;
            }
            index += 1;
        }
        for i in index..len_children-1 {                                        // shift left in the vec
            vec_filenode[len-1].children[i] = vec_filenode[len-1].children[i+1].clone();
        }
        vec_filenode[len-1].children.pop();                                     // remove the last one after shifting
        
        for i in 0..len-1 {
            let node = Box::new(vec_filenode[len-1-i].clone());                 // backpropagation to update the root
            vec_filenode[len-1-i-1].children[index_folder[len-i-2]] = node;
        }

        self.root = vec_filenode[0].clone();

        return Ok(());
    }

    pub fn edit_file(&mut self, name_input: Vec<char>, content: Vec<Vec<char>>) -> Result<(), ()> {
        // let name_input = file_input.file_name.clone();
        let mut found = false;
        let mut file_input: FileNode = FileNode::new(Vec::new(), FileType::Document);

        match self.retrieve(name_input.clone()) {
            Ok((file, _)) => {
                if file.file_type == FileType::Document {
                    file_input = file.clone();
                    found = true;
                } else {
                    println!("The folder cannnot be edited");
                    return Err(())
                }
            },
            Err(_) => {},
        }
        
        if found == false {      // there is no file named 'xxx'
            return Err(());
        }

        let index_folder = self.index_folder();
        let mut current = self.root.clone();
        let mut vec_filenode: Vec<FileNode> = vec![self.root.clone()];  // the whole stucture of every single subtree

        for index in &index_folder {
            current = Box::into_inner(current.children[*index].clone());
            vec_filenode.push(current.clone());
        }

        let len = vec_filenode.len();
        let mut index = 0;
        
        for f in &vec_filenode[len-1].children {                        // calculate in index of the input file
            if f.file_name == name_input {
                break;
            }
            index += 1;
        }

        file_input.edit_content(content);                               // edit the file
        vec_filenode[len-1].children[index] = Box::new(file_input);     // modify the edited file in the file system
        
        for i in 0..len-1 {
            let node = Box::new(vec_filenode[len-1-i].clone());                 // backpropagation to update the root
            vec_filenode[len-1-i-1].children[index_folder[len-i-2]] = node;
        }

        self.root = vec_filenode[0].clone();

        return Ok(());
    }

    pub fn index_folder(&self) -> Vec<usize> {        // path中每个文件夹在它目录中的索引
        let mut current = self.root.clone();
        let mut vec_index = vec![];

        for dic in self.path.iter() {               // 当前文件夹path
            if char_vec_cmp(&*dic, &str2char("root")) {
                continue;
            }

            let mut f: FileNode = FileNode::new(str2char(""), FileType::Document);
            for (i, file) in current.children.iter().enumerate() {   // 当前文件夹下的各个文件
                if *dic == Box::into_inner(file.clone()).file_name {
                    f = Box::into_inner(file.clone());
                    vec_index.push(i);
                    break;
                }
            }
            current = f;
        }

        return vec_index;
    }

    pub fn retrieve(&self, name_input: Vec<char>) -> Result<(FileNode, usize), ()> {      // retrieve the file in current path
        let mut current = self.root.clone();
        for dic in self.path.iter() {                   // current path
            if char_vec_cmp(&*dic, &str2char("root")) {
                continue;
            }

            let mut f: FileNode = FileNode::new(str2char(""), FileType::Document);
            for file in current.children.iter() {       // files in current path
                if *dic == Box::into_inner(file.clone()).file_name {
                    f = Box::into_inner(file.clone());
                    break;
                }
            }
            current = f;
        }
        
        let mut found = false;
        let mut file_output = FileNode::new(str2char(""), FileType::Document);
        let mut file_index = 0;
        for (i, file) in current.children.iter().enumerate() {
            if char_vec_cmp(&file.file_name, &name_input) {
                found = true;
                file_index = i;
                file_output = Box::into_inner(file.clone());
                break;
            }
        }
        if found == true {
            return Ok((file_output, file_index));
        } else {
            return Err(());
        }
    }

    pub fn read_file(&mut self, name_input: Vec<char>, if_print: bool) -> Result<Vec<Vec<char>>, ()> {
        match self.retrieve(name_input.clone()) {
            Ok((mut file, _)) => {
                if file.file_type == FileType::Document {
                    let content = file.get_content().clone();
                    if if_print {
                        for line in &content {
                            for c in line {
                                print!("{}", c);
                            }
                            println!("");
                        }
                    }
                    return Ok(file.get_content());
                } else {
                    println!("This is a folder");
                    return Err(());
                }
            },
            Err(_) => {
                print!("No file called ");       // todo
                for c in name_input.clone() {
                    print!("{}", c);
                }
                
                println!("");
                return Err(());
            },
        }
    }

    pub fn list(&self) {      // retrieve all the files in current folder
        let mut current = self.root.clone();
        for dic in self.path.iter() {                   // current path
            if char_vec_cmp(&*dic, &str2char("root")) {
                continue;
            }

            let mut f: FileNode = FileNode::new(str2char(""), FileType::Document);
            for file in current.children.iter() {       // files in current path
                if *dic == Box::into_inner(file.clone()).file_name {
                    f = Box::into_inner(file.clone());
                    break;
                }
            }
            current = f;
        }
        
        let len = current.children.len();
        for (i, file) in current.children.iter().enumerate() {
            for c in &file.file_name {
                print!("{}", c);
            }
            if i < len - 1 {
                print!("  ");
            } else {
                println!("");
            }
        }
    }

    pub fn into_folder(&mut self, name_input: Vec<char>) {
        match self.retrieve(name_input.clone()) {
            Ok((file, _)) => {
                if file.file_type == FileType::Folder {
                    self.path.push(name_input.clone());
                } else {
                    println!("This is not a folder");
                }
            },
            Err(_) => {
                print!("No file called ");       // todo
                for c in name_input.clone() {
                    print!("{}", c);
                }
                
                println!("");
            },
        }
    }

    pub fn outof_forlder(&mut self) {
        if self.path.len() == 1 {
            println!("This is root!");
        } else {
            self.path.pop();
        }        
    }

    pub fn get_path(&self) -> String {
        let len = self.path.len();
        let mut i = 0;
        let mut v: Vec<u8> = Vec::new();

        for p in &self.path {
            for c in p {
                // print!("{}", c);
                v.push(*c as u8);
            }            
            if i < len - 1 {
                // print!("\\");
                v.push('\\' as u8);
            } else {
                // print!(">");
                v.push('>' as u8);
            }
            i += 1;
        }

        String::from_utf8(v).unwrap()
    }

    pub fn print_path(&self) {
        let len = self.path.len();
        let mut i = 0;
        for p in &self.path {
            for c in p {
                print!("{}", c);
            }            
            if i < len - 1 {
                print!("\\");
            } else {
                print!(">");
            }
            i += 1;
        }
    }
}
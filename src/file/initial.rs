use crate::api::str2char;
use super::{FileNode, FileType};
use alloc::vec::Vec;
use super::file_system::FileSystem;

pub fn init(file_system: &mut FileSystem) {
    /* root */
    let mut doc = FileNode::new(str2char("doc"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("This is line1."));
    content.push(str2char("This is line2."));
    content.push(str2char("This is line3."));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    /* Text */
    let folder = FileNode::new(str2char("Text"), FileType::Folder);
    file_system.add_file(folder.clone()).unwrap();
    file_system.into_folder(folder.file_name);
    let doc = FileNode::new(str2char("Document1"), FileType::Document);
    file_system.add_file(doc.clone()).unwrap();
    let doc = FileNode::new(str2char("Document2"), FileType::Document);
    file_system.add_file(doc.clone()).unwrap();
    file_system.outof_forlder();

    /* Compiler */
    let folder = FileNode::new(str2char("Compiler"), FileType::Folder);
    file_system.add_file(folder.clone()).unwrap();
    file_system.into_folder(folder.file_name);

    let mut doc = FileNode::new(str2char("type"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("var a = 10"));
    content.push(str2char("print(string(a))"));
    content.push(str2char("var b = true"));
    content.push(str2char("print(string(b))"));
    content.push(str2char("var c = \"Ahau\""));
    content.push(str2char("print(c)"));
    (doc).edit_content(content);
    file_system.add_file(doc.clone()).unwrap();

    let mut doc = FileNode::new(str2char("calculate"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("var a = 1 + 2 * 3 + 1 / 1"));
    content.push(str2char("print(string(a))"));
    content.push(str2char("var b = 1 == 0 || false"));
    content.push(str2char("print(string(b))"));
    content.push(str2char("var c = \"Ahau\" + \" CS wyb\""));
    content.push(str2char("print(c)"));
    (doc).edit_content(content);
    file_system.add_file(doc.clone()).unwrap();

    let mut doc = FileNode::new(str2char("if"), FileType::Document);
    let mut content = Vec::new();
    for _ in 0..6 {
        content.push(str2char("var a = 0"));
    }
    content.push(str2char("var a = 3"));
    content.push(str2char("if a < 5\n{\n    a = 0\n}\nelse\n{\n    a = 10\n}"));
    content.push(str2char("print(string(a))"));
    (doc).edit_content(content);
    file_system.add_file(doc.clone()).unwrap();

    let mut doc = FileNode::new(str2char("for"), FileType::Document);
    let mut content = Vec::new();
    for _ in 0..2 {
        content.push(str2char("var a = 0"));
    }
    content.push(str2char("var s = 0"));
    content.push(str2char("for i = 1 to 10\n{\n    s = s + i\n}"));
    content.push(str2char("print(string(s))"));
    (doc).edit_content(content);
    file_system.add_file(doc.clone()).unwrap();

    let mut doc = FileNode::new(str2char("while"), FileType::Document);
    let mut content = Vec::new();
    for _ in 0..3 {
        content.push(str2char("var a = 0"));
    }
    content.push(str2char("var s = 0"));
    content.push(str2char("var i = 1"));
    content.push(str2char("while i <= 10\n{\n    s = s + i\n    i = i + 1\n}"));
    content.push(str2char("print(string(s))"));
    (doc).edit_content(content);
    file_system.add_file(doc.clone()).unwrap();

    let mut doc = FileNode::new(str2char("sleep"), FileType::Document);
    let mut content = Vec::new();
    for _ in 0..3 {
        content.push(str2char("var a = 10"));
    }
    content.push(str2char("for i = 1 to 5\n{\n    sleep(1)\n    print(string(i))\n}"));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    let mut doc = FileNode::new(str2char("breakpoint"), FileType::Document);
    let mut content = Vec::new();
    // content.push(str2char("var a = 1 * 2"));
    // content.push(str2char("breakpoint()"));
    // content.push(str2char("for i = 1 to 5\n    breakpoint()"));
    // content.push(str2char("var b = 3 * 4"));
    // content.push(str2char("breakpoint()"));
    content.push(str2char("var a = 1 * 2"));
    content.push(str2char("breakpoint()"));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    let mut doc = FileNode::new(str2char("random"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("var a = rand(10)"));
    content.push(str2char("print(string(a))"));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    let mut doc = FileNode::new(str2char("error1"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("var a = 10 + true"));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    let mut doc = FileNode::new(str2char("error2"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("var a = \"Ahau"));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    let mut doc = FileNode::new(str2char("error3"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("print(a)"));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    let mut doc = FileNode::new(str2char("error4"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("var a = int(true)"));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    let mut doc = FileNode::new(str2char("error5"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("printf(\"ahau\")"));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    let mut doc = FileNode::new(str2char("error6"), FileType::Document);
    let mut content = Vec::new();
    content.push(str2char("print(\"1\", \"2\")"));
    (doc).edit_content(content);
    file_system.add_file(doc).unwrap();

    file_system.outof_forlder();
}


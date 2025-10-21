use rspression::Chunk;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};

pub struct TestHelper;

impl TestHelper {
    pub fn get_path(directory: &str, file_name: &str) -> PathBuf {
        let output_dir = env::current_dir().unwrap();
        output_dir.join(".temp").join(directory).join(file_name)
    }

    pub fn write_string(content: &str, file_path: &Path) {
        Self::create_parent_if_not_exist(file_path);
        if let Ok(mut writer) = File::create(file_path) {
            let _ = writer.write_all(content.as_bytes());
        }
    }

    pub fn write_all_lines(lines: &[String], file_path: &Path) {
        Self::create_parent_if_not_exist(file_path);
        if let Ok(mut writer) = File::create(file_path) {
            let mut bf_writer = BufWriter::new(writer);
            for line in lines {
                let _ = writeln!(bf_writer, "{}", line);
            }
        }
    }

    pub fn create_parent_if_not_exist(file_path: &Path) {
        if let Some(parent) = file_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
    }

    pub fn write_chunk_file(chunk: &Chunk, file_path: &Path) {
        Self::create_parent_if_not_exist(file_path);
        let bytes = chunk.to_bytes();
        let _ = fs::write(file_path, bytes);
    }

    pub fn read_chunk_file(file_path: &Path) -> Option<Chunk> {
        if let Ok(bytes) = fs::read(file_path) {
            Some(Chunk::from_bytes(&bytes))
        } else {
            None
        }
    }
}

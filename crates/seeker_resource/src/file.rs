use bevy::prelude::*;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

#[derive(Component, Clone)]
pub struct File {
    pub path: PathBuf,
    pub filename: String,
    pub is_dir: bool,
    pub root_entity: Entity,
    pub level: Level,
}

#[derive(Resource, Default)]
pub struct CurrentFile {
    pub file: Option<File>,
}

#[derive(Component, Clone, Default)]
pub struct Level {
    pub level: usize,
}

impl Level {
    pub fn new(level: usize) -> Self {
        Self { level }
    }
}

pub fn get_files(path: PathBuf, entity: Entity, dialog_level: Level) -> Vec<File> {
    let Ok(entries) = fs::read_dir(path) else {
        return vec![];
    };
    let mut files = Vec::with_capacity(100);
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(".") {
                continue;
            }
            files.push(File {
                is_dir: path.is_dir(),
                path,
                filename: entry.file_name().to_string_lossy().to_string(),
                root_entity: entity,
                level: dialog_level.clone(),
            })
        }
    }
    files
}

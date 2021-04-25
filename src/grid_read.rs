use crate::Grid;
use std::fs;
use std::path::PathBuf;

pub fn file_read(filepath: PathBuf) -> String {
    let content = fs::read_to_string(filepath).expect("Problème lors de la lecture de la grille");
    content
}

pub fn size(content: &str) -> &str {
    let bytes = content.as_bytes();

    for (i, &element) in bytes.iter().enumerate() {
        if element == b'\n' {
            return &content[0..i];
        }
    }
    panic!("Fichier incorrect : size");
}

pub fn fill_grid_from_file(grid: &mut Grid, content: &str) {
    let bytes = content.as_bytes();
    let mut x = 0;
    let mut y = 0;
    let mut s = false;

    for &element in bytes.iter() {
        if element == b'\n' {
            if s == false {
                s = true;
            }
        } else if s {
            if element == b'1' {
                grid.set(x, y, true);
            } else if element == b'0' {
                grid.set(x, y, false);
            }
            x += 1;
            if x == grid.size {
                y += 1;
                x = 0;
            }
        }
    }
}
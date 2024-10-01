use std::{
    collections::HashMap,
    io::Error,
    path::{Path, PathBuf},
};

use crate::models;

pub struct TreeComponent {
    // Database: tables
    pub abs_paths: Vec<PathBuf>,
    pub databases: HashMap<String, Vec<String>>,
}
impl TreeComponent {
    pub fn new() -> Self {
        Self {
            abs_paths: vec![],
            databases: HashMap::new(),
        }
    }
}
impl super::Component for TreeComponent {
    // observer
    fn draw(
        &self,
        frame: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        app: &crate::app::App,
    ) {
        todo!()
    }

    fn setup(&mut self, args: &models::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        for path in &args.paths {
            let path = Path::new(path);
            if path.ends_with(".db") {
                self.abs_paths.push(path.canonicalize()?);
                // println!("{:?}", path);
            } else {
                return Err(Box::new(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Path has to be a database!",
                )));
            }
        }
        Ok(())
        // retrieve .db files
    }

    fn event(&mut self, key: Option<crossterm::event::KeyEvent>) -> super::KeyState {
        todo!()
    }

    fn hide(&mut self) {
        todo!()
    }

    fn show(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::super::Component;
    use std::{collections::HashMap, path::Path};

    #[test]
    fn path_exists() {
        // let mut tree = super::TreeComponent {
        //     databases: HashMap::new(),
        // };

        let path = Path::new("test.db");
        if path.ends_with(".db") {
            //assert_eq!(path.canonicalize()?)
        }
        assert!(path.canonicalize().unwrap().try_exists().unwrap());
        // println!("{:?}", path.canonicalize().unwrap()); // -- --nocapture
    }
}

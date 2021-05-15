use std::path::PathBuf;

#[derive(Debug)]
pub struct ImageSet {
    images: Vec<PathBuf>,
    curr_image_index: usize,
    len: usize,
}

impl ImageSet {
    pub fn new(mut images: Vec<PathBuf>, curr_image_path: &PathBuf) -> Self {
        images.sort();
        let curr_image_index = Self::get_image_index(&images, curr_image_path);
        let len = images.len();
        if curr_image_index == 0 {
            Self {images: vec![], curr_image_index: 0, len: 0}
        } else {
            Self {
                images,
                curr_image_index,
                len,
            }
        }
    }
    fn get_image_index(images: &Vec<PathBuf>, curr_image_path: &PathBuf) -> usize {
        images.iter().position(|r| r == curr_image_path).unwrap_or_default()
    }

    pub fn next(&mut self) -> Option<PathBuf> {
        if self.len == 0 { return None; }
        self.curr_image_index = (self.curr_image_index + 1) % self.len;
        Some(PathBuf::from(
            self.images.get(self.curr_image_index).unwrap(),
        ))
    }

    pub fn prev(&mut self) -> Option<PathBuf> {
        if self.len == 0 { return None; }
        self.curr_image_index = if self.curr_image_index > 0 {
            ((self.curr_image_index as i32 - 1) % self.len as i32).abs() as usize
        } else {
            self.len - 1
        };
        Some(PathBuf::from(
            self.images.get(self.curr_image_index).unwrap(),
        ))
    }
}

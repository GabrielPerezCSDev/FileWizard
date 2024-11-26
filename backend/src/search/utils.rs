

#[derive(Debug)]
pub struct Search {
    curr_directory: Option<String>,
    //map of files
    //map of folders

}

impl Search {
    pub fn new() -> Self {
        println!("[Search] created");
        Self { curr_directory: None }
    }
    pub fn execute_search(&self) {
        println!(
            "[Search] Executing search in {:?}",
            self.curr_directory.clone().unwrap_or_else(|| "No directory set".to_string())
        );
        // Add actual search logic here
    }

    /// Overwrite the current directory
    pub fn set_curr_directory(&mut self, curr_directory: String) {
        self.curr_directory = Some(curr_directory);
    }

    /// Get the current directory (immutable)
    pub fn get_curr_directory(&self) -> Option<String> {
        self.curr_directory.clone() // Clone to avoid ownership issues
    }
}
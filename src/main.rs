use eframe::egui::{self, CentralPanel, Context};
use eframe::App;
use std::collections::HashSet;

struct ImageViewerApp {
    // Image data could go here
}

impl App for ImageViewerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Rust ImageViewer");
            // Placeholder for image display functionality
        });
    }
}

fn get_supported_file_extensions() -> HashSet<&'static str> {
    let mut supported_formats = HashSet::new();
    
    // Common image formats supported by the `image` crate
    supported_formats.insert("jpeg");
    supported_formats.insert("jpg");
    supported_formats.insert("png");
    supported_formats.insert("gif");  // For animated images
    supported_formats.insert("bmp");
    supported_formats.insert("tiff");
    supported_formats.insert("ico");
    
    // Optionally add video formats (could use different crate for playback)
    supported_formats.insert("mp4");
    supported_formats.insert("avi");
    supported_formats.insert("mov");
    
    supported_formats
}

fn get_image_files_in_directory(path: &str, supported_formats: &HashSet<&str>) -> Vec<String> {
    let mut image_files = Vec::new();
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let file_path = entry.path();
        if let Some(extension) = file_path.extension() {
            if let Some(ext) = extension.to_str() {
                if supported_formats.contains(&ext.to_lowercase()[..]) {
                    image_files.push(file_path.display().to_string());
                }
            }
        }
    }
    
    image_files
}

fn main() {

    let supported_formats = get_supported_file_extensions();
    println!("Supported file extensions: {:?}", supported_formats);
    let image_files = get_image_files_in_directory("./", $supported_formats);
    println!("Found image files: {:?}", image_files);


    let app = ImageViewerApp {};
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Rust Image Viewer", native_options, Box::new(|_cc| Ok(Box::new(app))));
}

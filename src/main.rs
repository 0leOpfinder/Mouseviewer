use eframe::egui::{CentralPanel, Context, Key};
use eframe::{App, Frame, egui};
use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::env;

use eframe::egui::{Ui, TextureHandle, TextureOptions};
use image::io::Reader as ImageReader;
use image::DynamicImage;

struct SharedState {
    image_files: Vec<String>, // List of image files in the folder
    current_index: usize,     // Index of the currently displayed image
}

struct ImageViewerApp {
    state: Arc<Mutex<SharedState>>,
}

impl ImageViewerApp {
    fn new(image_files: Vec<String>) -> Self {
        Self {
            state: Arc::new(Mutex::new(SharedState {
                image_files,
                current_index: 0,
            })),
        }
    }

    fn render_image(&self, ui: &mut Ui, ctx: &eframe::egui::Context, image_path: &str) {
        // Placeholder for image rendering logic.
        // In real use, you'd load the image as a texture and display it.

        ui.label(format!("Rendering image: {}", image_path));
           // Load the image from the file
           let image = ImageReader::open(image_path)
           .expect("Failed to open image")
           .decode()
           .expect("Failed to decode image");

       // Convert the image into an egui texture
       let texture = Self::load_image_as_texture(ctx, image);

       // Display the texture
       ui.image(&texture);
   }

   fn load_image_as_texture(ctx: &eframe::egui::Context, image: DynamicImage) -> TextureHandle {
       let rgba_image = image.into_rgba8(); // Convert to RGBA format
       let size = [rgba_image.width() as usize, rgba_image.height() as usize];

       // Convert the raw image data into a texture
       ctx.load_texture(
           "image_texture", // A unique ID for the texture
           eframe::egui::ColorImage::from_rgba_unmultiplied(size, &rgba_image),
           TextureOptions::default(), // Default texture options
       )
   }
}

impl App for ImageViewerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {

        

        if let Ok(mut state) = self.state.lock() {

            if ctx.input(|i| i.key_pressed(Key::PageUp)) {
                if state.current_index > 0 {
                    state.current_index -= 1; // Move to the previous image
                }
            } else if ctx.input(|i| i.key_pressed(Key::PageDown)) {
                if state.current_index + 1 < state.image_files.len() {
                    state.current_index += 1; // Move to the next image
                }
            }


            let current_image = state
                .image_files
                .get(state.current_index)
                .unwrap_or(&"No Image Found".to_string())
                .clone();

            CentralPanel::default().show(ctx, |ui| {
                self.render_image(ui, ctx, &current_image);
            });
        }
    }
}

fn main() {









    let mut url;
    let mut folder_url = "./".to_string();
    let mut current_image = "".to_string();

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        url = args[1].clone();
        println!("The first argument is {}", args[1]);

        println!("is valid path: {}", Path::new(&url).exists());
        if !Path::new(&url).exists() {
            url = "./".to_string();
        } else {

            if Path::new(&url).is_dir() { 
                // the url is a folder so we dont need its parent. we do however now need an image from that folder
                folder_url=url;
                let images_in_folder = get_image_files_in_directory(&folder_url, &get_supported_file_extensions());

                // TODO: get first image in this folder
            } else {
                // The url is not a folder, and since its valid it must be a file. therefor we need its parent (the folder its in)
                current_image = url.clone();
                if let Some(parent) = Path::new(&url).parent() {
                    println!("The folder containing the file is: {}", parent.display().to_string());
                    folder_url=parent.display().to_string();
                } else {
                    // this should not be possible
                    println!("The given path has no parent (likely a root path).");
                }
            }
        }
    } else {
        url = "./".to_string();
    }







    //let folder_url = "./some/folder/with/images"; // Replace with your folder path
    let supported_formats = get_supported_file_extensions();

    let image_files = get_image_files_in_directory(&folder_url, &supported_formats);

    let app = ImageViewerApp::new(image_files);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Image Viewer", native_options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}

// Helper function to filter supported file extensions
fn get_supported_file_extensions() -> HashSet<&'static str> {
    ["jpeg", "jpg", "png", "gif", "bmp", "tiff", "ico", "webp"]
        .iter()
        .cloned()
        .collect()
}

// Helper function to get image files in a directory
fn get_image_files_in_directory(path: &str, supported_formats: &HashSet<&str>) -> Vec<String> {
    let mut image_files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(extension) = entry.path().extension() {
                        if let Some(ext_str) = extension.to_str() {
                            if supported_formats.contains(ext_str) {
                                image_files.push(entry.path().to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    image_files
}

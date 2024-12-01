# Mouseviewer

    /* 
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
    */





        //std::process::exit(0);


println!("Found image files: {:?}", image_files);
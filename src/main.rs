use reqwest::{
    blocking::Client,
    // header::{HeaderMap, HeaderName, HeaderValue}
};

use std::{
    env,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use crate::har::Har;

mod har;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        println!("No file supplied");
        return Ok(());
    }

    // Open the file and parse the JSON (file format of HAR)
    let mut file = File::open(&args[0])?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let har: Har = serde_json::from_str(buf.as_str()).unwrap();

    // Directory where the files will be downloaded
    let dir = Path::new(&har.log.pages[0].title);

    // To ensure that data is not being overriden, confirm that directory is empty
    if !dir.exists() {
        // Create the directory
        fs::create_dir(dir)?;
    } else if dir.to_path_buf().read_dir()?.next().is_some() {
        println!("Directory not empty");
        return Ok(());
    }


    // Download files
    let client = Client::new();
    // let mut headers = HeaderMap::new();

    for entry in har.log.entries {
        // Use the headers defined in the HAR
        // for header in entry.request.headers {
        //     headers.insert(
        //         HeaderName::from_bytes(header.name.as_bytes()).unwrap(),
        //         HeaderValue::from_bytes(header.name.as_bytes()).unwrap()
        //     );
        // }

        // Fetch the file
        let response = client
            .get(&entry.request.url)
            // .headers(headers.clone())
            .send()?;
        
        // Validate that the request succeeded
        if let Err(err) = response.error_for_status_ref() {
            println!("{}", err);
            // headers.clear();
            continue;
        }

        // Translate URL to file path
        let url = response.url();
        let target_file = dir.clone().join(extract_path(url.path()));
        
        // Create parent directories if necessary
        if !target_file.exists() {
            fs::create_dir_all(target_file.parent().unwrap()).unwrap();
        }
        
        // Write content
        fs::write(target_file, response.bytes()?)?;

        // headers.clear();
    }

    Ok(())
}

fn extract_path(path: &str) -> PathBuf {
    let mut buf = PathBuf::new();

    if path == "/" {
        buf.push("index.html");
        return buf;
    }

    let mut p = path.chars();
    p.next();
    
    buf.push(p.as_str());
    buf
}

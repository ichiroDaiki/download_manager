use hyper::Client;
use hyper_tls::HttpsConnector;
use std::fs;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use std::thread;
use std::time::Duration;
use hyper::body::HttpBody as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut downloaded = 0;
    let mut res = client.get("https://images.wallpapersden.com/image/download/sunset-8k_a2trZmyUmZqaraWkpJRtZWVlrWlqZWU.jpg".parse()?).await?;
    let len = res.headers().get("content-length").unwrap();
    let len = len.to_str().unwrap().parse::<u64>().unwrap();
    let mut _download_size = 0.0;
    let mut _size_format = String::new();

    println!("LEN: {:?}", len);

    if res.status() == 200{
        let pb = ProgressBar::new(len as u64);
        pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));
        
        let mut file = fs::File::create("prueba5".clone()).expect("Error occured while creating file");

        while let Some(item) = res.body_mut().data().await {
            let chunk = item.or(Err(format!("Error while downloading file")))?;
            file.write_all(&chunk)
                .or(Err(format!("Error while writing to file")))?;
            let new = min(downloaded + (chunk.len() as u64), len);
            downloaded = new;
            pb.set_position(new);
        }

        pb.finish_with_message("downloaded");  


        println!("DIV: {}", (len as f64 / 1000.0));
        
        if (len as f64 / 1000.0) < 1024.0{
            _download_size = len as f64 / 1000.0;
            _size_format = "KB".to_string();
        }

        println!("Data: {}.{}", _download_size, _size_format);
    
    
    
    }else{
        println!("No se puede descargar");
    }


    Ok(())
}


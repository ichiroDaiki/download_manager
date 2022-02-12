use hyper::Client;
use hyper_tls::HttpsConnector;
use std::fs;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use hyper::body::HttpBody as _;
use std::env;
use terminal_emoji::Emoji;
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() < 2{
        println!("# No olvides dar un nombre a tu archivo #");
    }else{
        let time_sleep = time::Duration::from_millis(1000);
        
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let getinfo = Emoji::new("👀", "looking");
        println!("{} Reading link [1/5]", getinfo);
        thread::sleep(time_sleep);

        //get url to download
        let mut res = client.get(args[0].parse()?).await?;
        let len = res.headers().get("content-length").unwrap();

        //get type of the content 
        let mut response_analize:Vec<char> = Vec::new();
        let mut resv2 = String::new(); 
        let mut other_type = String::new();
        let mut contador2 = 0;
            
        resv2 = args[0].clone();
        contador2 = resv2.len()-1;
        response_analize = resv2.chars().collect();

        loop{

            if response_analize[contador2] != '.'{
                other_type.push_str(&response_analize[contador2].to_string());
            }else{
                break;
            }

            contador2 -= 1;
        }

        let reversed = reverse_string(&other_type);
  
        let get_type = Emoji::new("🔖", "Get type");
        let type_icon = Emoji::new("📦", "Type");
        println!("{} Getting type of the file [2/5]", get_type);
        println!("{} Content-Type: {}", type_icon, reversed);
        thread::sleep(time_sleep);

        let len = len.to_str().unwrap().parse::<u64>().unwrap();
        let mut _download_size = 0.0;
        let mut _size_format = String::new();
        let getinfo = Emoji::new("🔥", "f");
        println!("{} Set Filename [3/5]", getinfo);        
        let name_file = String::from(&args[1]);
        thread::sleep(time_sleep);

        //check if the connection was successful
        if res.status() == 200{

            let pb = ProgressBar::new(len as u64);
            pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"));
            
            //create the file name
            let mut file_cronstruction = String::new();
            file_cronstruction.push_str(&name_file.to_string());
            file_cronstruction.push_str(&".".to_string());
            file_cronstruction.push_str(&reversed.to_string());
            let mut file = fs::File::create(file_cronstruction.clone()).expect("# Error al crear el archivo #");
            let mut downloaded = 0;

            let starting = Emoji::new("🤖", "downloading");
            println!("{} Starting Download [4/5]", starting);

            //write the file
            while let Some(item) = res.body_mut().data().await {
                let chunk = item.or(Err(format!("Error while downloading file")))?;
                file.write_all(&chunk)
                    .or(Err(format!("Error while writing to file")))?;
                let new = min(downloaded + (chunk.len() as u64), len);
                downloaded = new;
                pb.set_position(new);
            }

            pb.finish_with_message("downloaded");  
            let finish = Emoji::new("✅", "i");
            println!("{} Downloaded [5/5]", finish);
     
        
        }else{
            println!("No se puede descargar");
        }
    }


    Ok(())
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
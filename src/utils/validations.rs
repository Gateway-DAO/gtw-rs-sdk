use std::error::Error;

pub fn validate_file_name(file_name: &str) -> Result<String, Box<dyn Error>> {
    match file_name.split('.').last() {
        Some("png") => Ok("image/png".to_string()),
        Some("jpg") | Some("jpeg") => Ok("image/jpeg".to_string()),
        Some("pdf") => Ok("application/pdf".to_string()),
        _ => Err("Unsupported file extension".into()),
    }
}

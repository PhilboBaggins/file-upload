#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::fs::FileName;
use rocket::fs::TempFile;
use rocket::response::content::RawHtml;
use rocket::serde::Deserialize;
use rocket::State;
use rocket::tokio;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use time::format_description;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    app_upload_dir: String,
    app_allowed_extensions: Vec<String>,
}

fn get_file_extension<'a>(
    file_name: Option<&FileName>,
    allowed_extensions: &'a Vec<String>,
) -> Option<&'a str> {
    let file_name = file_name?;

    let dangerous_unsafe_unsanitized_raw = file_name
        .dangerous_unsafe_unsanitized_raw()
        .as_str()
        .to_ascii_lowercase();

    for ext in allowed_extensions {
        if dangerous_unsafe_unsanitized_raw.ends_with(&format!(".{}", ext)) {
            return Some(ext);
        }
    }

    None
}

fn get_current_timestamp() -> String {
    // TODO: Move this somewhere else so we don't have to parse the format string every time we generate a timestamp
    let timestamp_format =
        format_description::parse("[year]-[month]-[day]_[hour]-[minute]-[second]-[Subsecond]")
            .unwrap();

    // Try to get timstamp in local timezone but if that doesn't work, fall back to UTC
    if let Ok(timestamp) = OffsetDateTime::now_local() {
        timestamp.format(&timestamp_format).unwrap()
    } else {
        OffsetDateTime::now_utc().format(&timestamp_format).unwrap()
    }
}

#[post("/upload", format = "multipart/form-data", data = "<file>")]
async fn upload(
    mut file: Form<TempFile<'_>>,
    app_config: &State<AppConfig>,
) -> std::io::Result<String> {
    if let Some(ext) = get_file_extension(file.raw_name(), &app_config.app_allowed_extensions) {
        // Create directory
        let timestamp = get_current_timestamp();
        let dest_dir = PathBuf::from(&app_config.app_upload_dir).join(timestamp);
        tokio::fs::create_dir_all(&dest_dir).await?;

        // Persist file
        let filename = file.name().unwrap_or("FILENAME_UNKNOWN");
        let mut file_path = PathBuf::from(&dest_dir).join(filename);
        file_path.set_extension(ext);
        file.persist_to(&file_path).await?;

        // Create metadata file
        let metadata_file_path = PathBuf::from(&dest_dir).join("Original filename.txt");
        let mut output = File::create(metadata_file_path)?;
        let original_filename = file
            .raw_name()
            .unwrap()
            .dangerous_unsafe_unsanitized_raw()
            .as_str();
        write!(output, "{}", original_filename)?;

        // Return message to user
        Ok(format!(
            "{} bytes successfully uploaded to {}.{}",
            file.len(),
            file.name().unwrap(),
            ext
        ))
    } else {
        Ok("File rejected due to file type restrictions".to_string())
    }
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("index.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload])
        .attach(AdHoc::config::<AppConfig>())
}

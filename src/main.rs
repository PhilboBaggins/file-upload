#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::fs::FileName;
use rocket::fs::TempFile;
use rocket::serde::Deserialize;
use rocket::State;
use rocket::tokio;
use std::path::{Path, PathBuf};
use time::format_description;
use time::OffsetDateTime;
use rocket_dyn_templates::{Template, tera::Tera, context};

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

async fn try_persist_or_move_copy<P>(file: &mut Form<TempFile<'_>>, path: P) -> std::io::Result<()>
        where P: AsRef<Path>
{
    match file.persist_to(&path).await {
        Ok(x) => Ok(x),
        Err(ref _e) /*if e.kind() == std::io::ErrorKind::CrossesDevices*/ => {
            // I'd prefer this to only happen when `e.kind() == std::io::ErrorKind::CrossesDevices`
            // but `CrossesDevices` is a nightly only feature at this point and I don't want to
            // force the use of nightly right now
            // TODO: Revisit this later

            // If persisting fails, try to move the file instead
            file.move_copy_to(&path).await
        },
    }
}

#[cfg(unix)]
async fn set_permissions(file_path: &PathBuf) -> std::io::Result<()> {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;
    let perms = 436; // 0664 in octal
    let perms = Permissions::from_mode(perms);
    tokio::fs::set_permissions(file_path, perms).await?;

    Ok(())
}

#[cfg(not(unix))]
async fn set_permissions(_file_path: &PathBuf) -> std::io::Result<()> {

    // TODO: ???????????

    Ok(())
}

#[post("/upload", format = "multipart/form-data", data = "<file>")]
async fn upload(
    mut file: Form<TempFile<'_>>,
    app_config: &State<AppConfig>,
) -> std::io::Result<Template> {
    if let Some(ext) = get_file_extension(file.raw_name(), &app_config.app_allowed_extensions) {
        // Create directory
        let timestamp = get_current_timestamp();
        let dest_dir = PathBuf::from(&app_config.app_upload_dir).join(timestamp);
        tokio::fs::create_dir_all(&dest_dir).await?;

        // Persist file
        let filename = file.name().unwrap_or("FILENAME_UNKNOWN");
        let mut file_path = PathBuf::from(&dest_dir).join(filename);
        file_path.set_extension(ext);
        try_persist_or_move_copy(&mut file, &file_path).await?;
        set_permissions(&file_path).await?;

        // Create metadata file (if necessary)
        let original_filename = file
            .raw_name()
            .unwrap()
            .dangerous_unsafe_unsanitized_raw()
            .as_str();
        if format!("{}.{}", file.name().unwrap(), ext) != original_filename {
            let metadata_file_path = PathBuf::from(&dest_dir).join("Original filename.txt");
            tokio::fs::write(&metadata_file_path, original_filename).await?;
        }

        // Return message to user
        Ok(Template::render("message", context! {
            message: format!(
                "{} bytes successfully uploaded to {}.{}",
                file.len(),
                file.name().unwrap(),
                ext
            )
        }))
    } else {
        Ok(Template::render("message", context! {
            message: "File rejected due to file type restrictions"
        }))
    }
}

#[get("/")]
fn index(app_config: &State<AppConfig>) -> Template {
    Template::render("index", context! {
        allowed_extensions: &app_config.app_allowed_extensions,
    })
}

pub fn customize(_tera: &mut Tera) {

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload])
        .attach(AdHoc::config::<AppConfig>())
        .attach(Template::custom(|engines| {
            customize(&mut engines.tera);
        }))
}

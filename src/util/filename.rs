use std::fs::create_dir_all;

pub fn generate_filename<'a>(name: &'a str, extension: &'a str) -> String {
    create_dir_all(format!("./paintings/{}", name.to_lowercase()))
        .expect("Could not create output directory");

    let version = std::fs::read_dir(format!("./paintings/{}", name.to_lowercase()))
        .expect("Could not read output directory")
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|entry| entry.file_name().into_string().ok())
        })
        .count()
        + 1;

    format!(
        "./paintings/{}/{name}-{version}.{extension}",
        name.to_lowercase()
    )
}

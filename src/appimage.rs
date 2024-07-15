use crate::error;

pub fn print_image(url: &str, width: u16, height: u16, offset: i16) {
    let response = reqwest::blocking::get(url)
        .unwrap_or_else(|_| error::error_and_quit(format!("failed to fetch image").as_ref()));

    let image = image::load_from_memory(
        &response
            .bytes()
            .unwrap_or_else(|_| error::error_and_quit("failed to load image bytes")),
    )
    .unwrap_or_else(|_| error::error_and_quit(format!("failed to create image").as_ref()));

    let conf = viuer::Config {
        x: 1,
        y: offset,
        absolute_offset: false,
        width: Some(width.into()),
        height: Some(height.into()),
        ..Default::default()
    };

    viuer::print(&image, &conf).unwrap_or_else(|_| error::error_and_quit("failed to print image"));
}

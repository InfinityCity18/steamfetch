use crate::error;

pub fn print_image(url: &str) {
    let response = reqwest::blocking::get(url)
        .unwrap_or_else(|_| error::error_and_quit(format!("failed to fetch image").as_ref()));

    let image = image::load_from_memory(
        &response
            .bytes()
            .unwrap_or_else(|_| error::error_and_quit("failed to load image bytes")),
    )
    .unwrap_or_else(|_| error::error_and_quit(format!("failed to create image").as_ref()));

    let (term_width, term_height) = viuer::terminal_size();
    let width = term_width / 2;
    let height = (width * 10) / 46;

    let conf = viuer::Config {
        x: 2,
        y: 1,
        absolute_offset: false,
        width: Some(width.into()),
        height: Some(height.into()),
        ..Default::default()
    };

    viuer::print(&image, &conf).unwrap_or_else(|_| error::error_and_quit("failed to print image"));
}

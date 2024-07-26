use crate::error::{ExitResult, IntoResultExitError};

pub async fn print_image(
    url: &str,
    width: u16,
    height: u16,
    x_offset: u16,
    y_offset: i16,
) -> ExitResult<'static, ()> {
    let response = reqwest::get(url)
        .await
        .into_exit_error("failed to fetch image")?;

    let image = image::load_from_memory(
        &response
            .bytes()
            .await
            .into_exit_error("failed to get image bytes")?,
    )
    .into_exit_error("failed to load image into memory")?;

    let conf = viuer::Config {
        x: x_offset,
        y: y_offset,
        absolute_offset: false,
        width: Some(width.into()),
        height: Some(height.into()),
        restore_cursor: true,
        ..Default::default()
    };

    viuer::print(&image, &conf).into_exit_error("failed to print image")?;
    Ok(())
}

use std::process::{Command, Stdio};

use common::{
    failure::Error,
    image::{jpeg::JPEGEncoder, ColorType, RgbaImage},
};

use util::Image;

pub fn filter(image: &Image, command: String) -> Result<(), Error> {
    let (shell, arg0) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let mut child = Command::new(shell)
        .args(&[arg0, &command])
        .stdin(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let image: RgbaImage = image.clone().into();
    JPEGEncoder::new(&mut stdin).encode(
        &image,
        image.width(),
        image.height(),
        ColorType::RGBA(8),
    )?;
    drop(stdin);

    if child.wait()?.success() {
        Ok(())
    } else {
        Err(format_err!("Failed to execute `{}'", command))
    }
}

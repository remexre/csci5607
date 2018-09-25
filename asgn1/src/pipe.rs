use std::process::{Command, Stdio};

use common::{
    failure::Error,
    image::{jpeg::JPEGEncoder, ColorType, RgbaImage},
};

pub fn filter(image: &RgbaImage, command: String) -> Result<(), Error> {
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
    JPEGEncoder::new(&mut stdin).encode(
        image,
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

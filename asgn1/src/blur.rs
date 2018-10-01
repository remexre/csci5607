use convolve;
use util::Image;

pub fn filter(image: &Image) -> Image {
    convolve::filter(
        image,
        [
            [1.0 / 16.0, 1.0 / 8.0, 1.0 / 16.0],
            [1.0 / 8.0, 1.0 / 4.0, 1.0 / 8.0],
            [1.0 / 16.0, 1.0 / 8.0, 1.0 / 16.0],
        ],
    )
}

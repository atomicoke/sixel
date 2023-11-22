use std::any::Any;
use std::fs;
use std::io::Write;
use std::path::Path;

use color_quant::NeuQuant;
use image::{AnimationDecoder, DynamicImage, ImageResult};
use image::imageops::FilterType;

pub fn show(path: &Path, size: (u32, u32)) -> anyhow::Result<Vec<u8>> {
    let img = crop(path, size)?;

    // todo gif
    // let decoder = GifDecoder::new( File::open(path)?)?;
    // let frames = decoder.into_frames();
    // let frames = frames.collect_frames()?;


    let b = encode(img)?;

    Ok(b)
}

fn encode(img: DynamicImage) -> anyhow::Result<Vec<u8>> {
    let alpha = img.color().has_alpha();

    let img = img.into_rgba8();
    let nq = NeuQuant::new(10, 256 - alpha as usize, &img);

    let mut buf: Vec<u8> = Vec::with_capacity(1 << 16);
    write!(
        buf,
        "{}P0;1;8q\"1;1;{};{}",
        "\x1b",
        img.width(),
        img.height()
    )?;

    // Palette
    for (i, c) in nq.color_map_rgba().chunks(4).enumerate() {
        write!(
            buf,
            "#{};2;{};{};{}",
            i + alpha as usize,
            c[0] as u16 * 100 / 255,
            c[1] as u16 * 100 / 255,
            c[2] as u16 * 100 / 255
        )?;
    }

    for y in 0..img.height() {
        let c = (b'?' + (1 << (y % 6))) as char;

        let mut last = 0;
        let mut repeat = 0usize;
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y).0;
            let idx = if pixel[3] == 0 {
                0
            } else {
                nq.index_of(&pixel) as u8 + alpha as u8
            };

            if idx == last || repeat == 0 {
                (last, repeat) = (idx, repeat + 1);
                continue;
            }

            if repeat > 1 {
                write!(buf, "#{last}!{repeat}{c}")?;
            } else {
                write!(buf, "#{last}{c}")?;
            }

            (last, repeat) = (idx, 1);
        }

        if repeat > 1 {
            write!(buf, "#{last}!{repeat}{c}")?;
        } else {
            write!(buf, "#{last}{c}")?;
        }

        write!(buf, "$")?;
        if y % 6 == 5 {
            write!(buf, "-")?;
        }
    }

    write!(buf, "{}\\{}", "\x1b", "")?;

    Ok(buf)
}

fn crop(path: &Path, size: (u32, u32)) -> ImageResult<DynamicImage> {
    let img = fs::read(path)?;

    let (w, h) = size;
    let img = image::load_from_memory(&img)?;

    Ok(if img.width() > w || img.height() > h {
        img.resize(w, h, FilterType::Triangle)
    } else {
        img
    })
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::sixel::show;

    #[test]
    fn qwe() {
        let path = "/home/like/project/fzdwx.github.io/public/images/2023-11-20-22-00-44.gif";
        show(Path::new(path), (300, 300)).unwrap();
    }
}
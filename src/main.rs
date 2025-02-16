use std::{fs::read, path::PathBuf};

use anyhow::{Result, anyhow};
use bevy_asset::RenderAssetUsages;
use bevy_color::{color_difference::EuclideanDistance, prelude::*};
use bevy_image::{CompressedImageFormats, ImageSampler, ImageType, prelude::*};
use bevy_math::FloatOrd;
use clap::Parser;
use itertools::Itertools;
use wgpu_types::{Extent3d, TextureDimension, TextureFormat};

#[derive(Parser)]
struct Cli {
    palette_path: PathBuf,
    #[arg(
        help = "Index of the color to which the output will fade. To find the index, count the colors in your palette from left to right and top to bottom (as if reading English), starting from 0, until you reach the target color."
    )]
    to: u8,
    #[arg(help = "number of frames")]
    frames: u8,
    out_path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let palette = Image::from_buffer(
        &read(cli.palette_path)?,
        ImageType::Format(ImageFormat::Png),
        CompressedImageFormats::NONE,
        true,
        ImageSampler::Default,
        RenderAssetUsages::empty(),
    )?;

    let palette_width = palette.width();
    let palette_height = palette.height();

    let palette_cols = (0..palette_height)
        .cartesian_product(0..palette_width)
        .filter_map(|(y, x)| {
            palette
                .get_color_at(x, y)
                .map(|col| (col.alpha() >= 0.5).then(|| (x, y, col, Oklaba::from(col))))
                .transpose()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let &(_, _, _, to) = palette_cols
        .get(cli.to as usize)
        .ok_or_else(|| anyhow!("color index out of bounds"))?;

    let mut fade = Image::new_fill(
        Extent3d {
            width: palette_width,
            height: palette_height * cli.frames as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::empty(),
    );

    for frame in 0..cli.frames {
        let t_denom = cli.frames - 1;
        let t = if t_denom == 0 {
            0.
        } else {
            frame as f32 / t_denom as f32
        };
        let fade_offset = frame as u32 * palette_height;

        for &(palette_x, palette_y, _, palette_oklab) in &palette_cols {
            let mix = palette_oklab.mix(&to, t);

            let &(_, _, fade_col, _) = palette_cols
                .iter()
                .min_by_key(|&&(_, _, _, col)| FloatOrd(mix.distance_squared(&col)))
                .ok_or_else(|| anyhow!("input image contains no colors"))?;

            fade.set_color_at(palette_x, palette_y + fade_offset, fade_col)?;
        }
    }

    fade.try_into_dynamic()?.save(cli.out_path)?;

    Ok(())
}

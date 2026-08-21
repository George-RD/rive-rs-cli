use crate::render::RenderError;
use png::{BitDepth, ColorType, Decoder, Encoder};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub distinct_colors: usize,
    pub blank: bool,
    pub rgba: Vec<u8>,
}

const PREVIEW_COLUMNS: u32 = 32;
const LOW_COVERAGE_THRESHOLD: f64 = 0.15;
const MEDIUM_COVERAGE_THRESHOLD: f64 = 0.50;

#[derive(Debug, Serialize, Deserialize)]
pub struct PixelBounds {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoveragePreview {
    pub dominant_color: String,
    pub dominant_percentage: f64,
    pub non_background_bounds: Option<PixelBounds>,
    pub grid: Vec<String>,
    pub text: String,
}

pub fn coverage_preview(image: &ImageInfo) -> CoveragePreview {
    let pixel_count = image.rgba.len() / 4;
    let (dominant, dominant_count) = dominant_color(&image.rgba);
    let dominant_percentage = percentage(dominant_count, pixel_count);
    let non_background_bounds = non_background_bounds(image, dominant);
    let rows = preview_rows(image.width, image.height);
    let grid = preview_grid(image, dominant, rows);
    let bounds = non_background_bounds.as_ref().map_or_else(
        || "none".to_string(),
        |bounds| {
            format!(
                "({}, {})..({}, {})",
                bounds.left, bounds.top, bounds.right, bounds.bottom
            )
        },
    );
    let mut text = format!(
        "legend: ' ' entirely transparent | '.' <15% coverage or opaque background | '+' <50% coverage | '#' >=50% coverage\n\
dominant: {} ({dominant_percentage:.2}%) | non-background bounds: {bounds}\n",
        rgba_hex(dominant)
    );
    for line in &grid {
        text.push_str(line);
        text.push('\n');
    }
    CoveragePreview {
        dominant_color: rgba_hex(dominant),
        dominant_percentage,
        non_background_bounds,
        grid,
        text,
    }
}

fn dominant_color(rgba: &[u8]) -> ([u8; 4], usize) {
    let mut counts = HashMap::new();
    for pixel in rgba.as_chunks::<4>().0 {
        let color = [pixel[0], pixel[1], pixel[2], pixel[3]];
        *counts.entry(color).or_insert(0usize) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|(color, count)| (*count, *color))
        .unwrap_or(([0, 0, 0, 0], 0))
}

fn percentage(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        return 0.0;
    }
    numerator as f64 * 100.0 / denominator as f64
}

fn rgba_hex(color: [u8; 4]) -> String {
    format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        color[0], color[1], color[2], color[3]
    )
}

fn non_background_bounds(image: &ImageInfo, background: [u8; 4]) -> Option<PixelBounds> {
    let mut bounds = None;
    for (index, pixel) in image.rgba.as_chunks::<4>().0.iter().enumerate() {
        if pixel == background {
            continue;
        }
        let x = index as u32 % image.width;
        let y = index as u32 / image.width;
        bounds = Some(match bounds {
            Some(PixelBounds {
                left,
                top,
                right,
                bottom,
            }) => PixelBounds {
                left: left.min(x),
                top: top.min(y),
                right: right.max(x),
                bottom: bottom.max(y),
            },
            None => PixelBounds {
                left: x,
                top: y,
                right: x,
                bottom: y,
            },
        });
    }
    bounds
}

fn preview_rows(width: u32, height: u32) -> u32 {
    if width == 0 || height == 0 {
        return 0;
    }
    let numerator = u64::from(height) * u64::from(PREVIEW_COLUMNS);
    let denominator = u64::from(width);
    numerator.div_ceil(denominator) as u32
}

fn preview_grid(image: &ImageInfo, background: [u8; 4], rows: u32) -> Vec<String> {
    let mut grid = Vec::with_capacity(rows as usize);
    for row in 0..rows {
        let mut line = String::with_capacity(PREVIEW_COLUMNS as usize);
        for column in 0..PREVIEW_COLUMNS {
            line.push(cell_marker(image, background, column, row, rows));
        }
        grid.push(line);
    }
    grid
}

fn cell_marker(image: &ImageInfo, background: [u8; 4], column: u32, row: u32, rows: u32) -> char {
    let x_start = (column * image.width / PREVIEW_COLUMNS).min(image.width.saturating_sub(1));
    let x_end = ((column + 1) * image.width / PREVIEW_COLUMNS)
        .max(x_start + 1)
        .min(image.width);
    let y_start = (row * image.height / rows).min(image.height.saturating_sub(1));
    let y_end = ((row + 1) * image.height / rows)
        .max(y_start + 1)
        .min(image.height);
    let mut samples = 0usize;
    let mut covered = 0usize;
    let mut transparent = 0usize;
    for y in y_start..y_end.min(image.height) {
        for x in x_start..x_end.min(image.width) {
            let index = ((y * image.width + x) * 4) as usize;
            let pixel = &image.rgba[index..index + 4];
            samples += 1;
            if pixel[3] == 0 {
                transparent += 1;
            }
            if pixel != background {
                covered += 1;
            }
        }
    }
    if samples > 0 && transparent == samples {
        return ' ';
    }
    let coverage = covered as f64 / samples as f64;
    if coverage < LOW_COVERAGE_THRESHOLD {
        '.'
    } else if coverage < MEDIUM_COVERAGE_THRESHOLD {
        '+'
    } else {
        '#'
    }
}
pub fn analyze(path: &Path) -> Result<ImageInfo, RenderError> {
    let file = File::open(path)?;
    let mut reader = Decoder::new(file)
        .read_info()
        .map_err(|e| RenderError::Message(e.to_string()))?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buf)
        .map_err(|e| RenderError::Message(e.to_string()))?;
    let data = &buf[..info.buffer_size()];
    let mut rgba = Vec::with_capacity((info.width * info.height * 4) as usize);
    match info.color_type {
        ColorType::Rgba => rgba.extend_from_slice(data),
        ColorType::Rgb => {
            for p in data.as_chunks::<3>().0 {
                rgba.extend_from_slice(&[p[0], p[1], p[2], 255])
            }
        }
        ColorType::Grayscale => {
            for &v in data {
                rgba.extend_from_slice(&[v, v, v, 255])
            }
        }
        ColorType::GrayscaleAlpha => {
            for p in data.as_chunks::<2>().0 {
                rgba.extend_from_slice(&[p[0], p[0], p[0], p[1]])
            }
        }
        ColorType::Indexed => return Err(RenderError::Message("indexed PNG unsupported".into())),
    };
    let colors = rgba.as_chunks::<4>().0.iter().collect::<HashSet<_>>().len();
    Ok(ImageInfo {
        width: info.width,
        height: info.height,
        distinct_colors: colors,
        blank: colors <= 1,
        rgba,
    })
}

pub fn pixel_difference(a: &ImageInfo, b: &ImageInfo) -> Result<f64, RenderError> {
    if a.width != b.width || a.height != b.height {
        return Err(RenderError::Message(format!(
            "cannot compare {}x{} against {}x{}",
            a.width, a.height, b.width, b.height
        )));
    }
    let pixels = a.rgba.len() / 4;
    if pixels == 0 {
        return Ok(0.0);
    }
    let different = a
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .zip(b.rgba.as_chunks::<4>().0.iter())
        .filter(|(left, right)| left != right)
        .count();
    Ok(different as f64 / pixels as f64 * 100.0)
}
pub fn contact_sheet(paths: &[PathBuf], out: &Path) -> Result<(), RenderError> {
    let imgs = paths
        .iter()
        .map(|p| analyze(p))
        .collect::<Result<Vec<_>, _>>()?;
    if imgs.is_empty() {
        return Ok(());
    }
    let w = imgs.iter().map(|i| i.width).sum();
    let Some(h) = imgs.iter().map(|i| i.height).max() else {
        return Ok(());
    };
    let mut pixels = vec![0u8; (w * h * 4) as usize];
    let mut x = 0;
    for i in imgs {
        for row in 0..i.height {
            let dst = ((row * w + x) * 4) as usize;
            let src = ((row * i.width) * 4) as usize;
            pixels[dst..dst + (i.width * 4) as usize]
                .copy_from_slice(&i.rgba[src..src + (i.width * 4) as usize]);
        }
        x += i.width;
    }
    let f = File::create(out)?;
    let mut e = Encoder::new(BufWriter::new(f), w, h);
    e.set_color(ColorType::Rgba);
    e.set_depth(BitDepth::Eight);
    e.write_header()
        .map_err(|e| RenderError::Message(e.to_string()))?
        .write_image_data(&pixels)
        .map_err(|e| RenderError::Message(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ImageInfo, analyze, contact_sheet, coverage_preview};
    use png::{BitDepth, ColorType, Encoder};
    use std::{
        collections::HashSet,
        fs::File,
        io::BufWriter,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    static IMAGE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    fn unique_path(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{prefix}-{}-{}.png",
            std::process::id(),
            IMAGE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ))
    }

    fn image(pixels: &[u8], width: u32, height: u32) -> PathBuf {
        let path = unique_path("rive-image-test");
        let file = File::create(&path).unwrap();
        let mut encoder = Encoder::new(BufWriter::new(file), width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        encoder
            .write_header()
            .unwrap()
            .write_image_data(pixels)
            .unwrap();
        path
    }

    #[test]
    fn analyze_marks_single_color_blank_and_counts_rgba_colors() {
        let one = image(&[1, 2, 3, 4, 1, 2, 3, 4], 2, 1);
        let two = image(&[1, 2, 3, 4, 5, 6, 7, 8], 2, 1);
        let a = analyze(&one).unwrap();
        let b = analyze(&two).unwrap();
        assert_eq!((a.distinct_colors, a.blank), (1, true));
        assert_eq!((b.distinct_colors, b.blank), (2, false));
        let _ = std::fs::remove_file(one);
        let _ = std::fs::remove_file(two);
    }

    fn info(pixels: &[u8], width: u32, height: u32) -> ImageInfo {
        let distinct_colors = pixels
            .as_chunks::<4>()
            .0
            .iter()
            .collect::<HashSet<_>>()
            .len();
        ImageInfo {
            width,
            height,
            distinct_colors,
            blank: distinct_colors <= 1,
            rgba: pixels.to_vec(),
        }
    }

    #[test]
    fn coverage_preview_reports_dominant_color_percentage_and_bounds() {
        let preview = coverage_preview(&info(
            &[0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 0, 0, 0],
            2,
            2,
        ));
        assert_eq!(preview.dominant_color, "#00000000");
        assert_eq!(preview.dominant_percentage, 75.0);
        assert_eq!(
            preview.non_background_bounds.as_ref().map(|bounds| (
                bounds.left,
                bounds.top,
                bounds.right,
                bounds.bottom
            )),
            Some((0, 1, 0, 1))
        );
    }

    #[test]
    fn coverage_preview_distinguishes_transparent_and_opaque_background_cells() {
        let transparent = coverage_preview(&info(&[0, 0, 0, 0], 1, 1));
        let opaque = coverage_preview(&info(&[10, 20, 30, 255], 1, 1));
        assert_eq!(transparent.grid[0], " ".repeat(32));
        assert_eq!(opaque.grid[0], ".".repeat(32));
    }
    #[test]
    fn coverage_preview_uses_low_and_medium_thresholds() {
        let mut pixels = vec![0u8; 128 * 16 * 4];
        for pixel in pixels.as_chunks_mut::<4>().0 {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }
        pixels[0..4].copy_from_slice(&[255, 0, 0, 255]);
        let preview = coverage_preview(&info(&pixels, 128, 16));
        assert_eq!(preview.grid[0].chars().next(), Some('.'));

        let mut pixels = vec![0u8; 96 * 3 * 4];
        for pixel in pixels.as_chunks_mut::<4>().0 {
            pixel.copy_from_slice(&[0, 0, 0, 255]);
        }
        pixels[0..8].copy_from_slice(&[255, 0, 0, 255, 255, 0, 0, 255]);
        let preview = coverage_preview(&info(&pixels, 96, 3));
        assert_eq!(preview.grid[0].chars().next(), Some('+'));
        for row in 1..3 {
            let index = row * 96 * 4;
            pixels[index..index + 4].copy_from_slice(&[255, 0, 0, 255]);
            if row == 1 {
                let index = row * 96 * 4 + 4;
                pixels[index..index + 4].copy_from_slice(&[255, 0, 0, 255]);
            }
        }
        let preview = coverage_preview(&info(&pixels, 96, 3));
        assert_eq!(preview.grid[0].chars().next(), Some('#'));
    }

    #[test]
    fn contact_sheet_places_different_sized_frames_left_to_right() {
        let left = image(&[255, 0, 0, 255], 1, 1);
        let right = image(&[0, 255, 0, 255, 0, 0, 255, 255], 2, 1);
        let out = unique_path("rive-image-sheet-test");
        contact_sheet(&[left.clone(), right.clone()], &out).unwrap();
        let sheet = analyze(&out).unwrap();
        assert_eq!((sheet.width, sheet.height), (3, 1));
        assert_eq!(
            &sheet.rgba,
            &[255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255]
        );
        contact_sheet(std::slice::from_ref(&left), &out).unwrap();
        assert_eq!(
            (analyze(&out).unwrap().width, analyze(&out).unwrap().height),
            (1, 1)
        );
        for path in [left, right, out] {
            let _ = std::fs::remove_file(path);
        }
    }
}

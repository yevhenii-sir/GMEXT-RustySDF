//! Generating various types of distance fields.

use image::{GenericImage, Luma, Pixel, Primitive, Rgb, Rgba};
use na::Affine2;
use rayon::prelude::*;

use crate::bezier::{
    prepared::{Aabb, PreparedColoredShape, PreparedComponent},
    scanline::FillRule,
    Point,
};

pub(crate) fn pixel_from_f64<P: Primitive>(x: f64) -> P {
    let min = P::DEFAULT_MIN_VALUE.to_f64().unwrap();
    let max = P::DEFAULT_MAX_VALUE.to_f64().unwrap();
    P::from(x * (max - min) + min).unwrap()
}

pub(crate) fn signed_distance_to_pixel_value<P: Primitive>(sd: f64, range: f64) -> P {
    pixel_from_f64((sd / range + 0.5).clamp(0.0, 1.0))
}

pub(crate) fn pixel_value_to_signed_distance<P: Primitive>(pix: P, range: f64) -> f64 {
    let min = P::DEFAULT_MIN_VALUE.to_f64().unwrap();
    let max = P::DEFAULT_MAX_VALUE.to_f64().unwrap();
    let pix = pix.to_f64().unwrap();
    ((pix - min) / (max - min) - 0.5) * range
}

pub(crate) fn generate<Px: Pixel, I: GenericImage<Pixel = Px>, F: Fn(Point) -> Px>(
    sampler: F,
    dest: &mut I,
) {
    for y in 0..dest.height() {
        for x in 0..dest.width() {
            let p = Point::new((x as f64) + 0.5, (y as f64) + 0.5);
            dest.put_pixel(x, y, sampler(p))
        }
    }
}

pub(crate) fn render<Px: Pixel, I: GenericImage<Pixel = Px>, F: Fn(Point) -> Px>(
    transformation: &Affine2<f64>,
    sampler: F,
    dest: &mut I,
) {
    for y in 0..dest.height() {
        for x in 0..dest.width() {
            let p = Point::new((x as f64) + 0.5, (y as f64) + 0.5);
            let tp = transformation.transform_point(&p);
            dest.put_pixel(x, y, sampler(tp))
        }
    }
}

fn sampler_sdf<P: Primitive>(shape: &PreparedComponent, range: f64, point: Point) -> Luma<P> {
    let d_min = shape.signed_distance_only(point);
    Luma([signed_distance_to_pixel_value(d_min.distance(), range)])
}

/// Generates a single-channel signed distance field.
///
/// * `shape` is the shape for which the distance field should be generated.
/// * `transformation` is a transformation from the units used by `shape` to
///   pixel units.
/// * `range` is the range of the distances represented, so that the resulting
///   distance field represents values in the range `[-range / 2.0, range / 2.0]`.
/// * `dest` is an instance of [`GenericImage`] to which the resulting distance
///   field should be written.
pub fn generate_sdf<P: Primitive, I: GenericImage<Pixel = Luma<P>>>(
    shape: &PreparedComponent,
    range: f64,
    dest: &mut I,
) {
    generate(|point| sampler_sdf(shape, range, point), dest)
}

#[inline]
fn banded_signed_distance(
    shape: &PreparedComponent,
    point: Point,
    half_range: f64,
    aabbs: &[Aabb],
    coarse: Option<Aabb>,
) -> f64 {
    match coarse {
        Some(bounds) if bounds.contains(point) => {
            match shape.signed_distance_near(point, aabbs) {
                Some(d) => d.distance(),
                None => -half_range,
            }
        }
        _ => -half_range,
    }
}

#[inline]
fn apply_fill_sign_unit(mut unit: f64, fill: bool) -> f64 {
    if (unit > 0.5) != fill {
        unit = 1.0 - unit;
    }
    unit
}

/// Generates a single-channel SDF using a narrow band around the outline.
///
/// Pixels farther than `range / 2` from every segment (via conservative
/// control-point AABBs) are written as outside (`0`) and should be followed by
/// [`crate::render::correct_sign_sdf`], which flips interior far pixels.
///
/// This preserves analytic distances near edges while skipping the expensive
/// per-segment distance solve for most of the glyph tile.
///
/// For a parallel, zero-copy atlas path see [`generate_sdf_banded_rgba8`].
pub fn generate_sdf_banded<P: Primitive, I: GenericImage<Pixel = Luma<P>>>(
    shape: &PreparedComponent,
    range: f64,
    dest: &mut I,
) {
    let half_range = range * 0.5;
    // Extra half-pixel slack avoids missing edge samples at AABB boundaries (script joins).
    let aabbs = shape.segment_aabbs_expanded(half_range);
    let coarse = aabbs.iter().copied().reduce(Aabb::merge);

    for y in 0..dest.height() {
        for x in 0..dest.width() {
            let point = Point::new((x as f64) + 0.5, (y as f64) + 0.5);
            let sd = banded_signed_distance(shape, point, half_range, &aabbs, coarse);
            dest.put_pixel(x, y, Luma([signed_distance_to_pixel_value(sd, range)]));
        }
    }
}

/// Bake a narrow-band analytic SDF straight into an RGBA8 atlas tile.
///
/// Combines distance sampling, sign correction ([`FillRule::Nonzero`]), optional
/// Y-flip (font Y-up → top-down textures), and RGBA packing (`R=distance`,
/// `G=B=0`, `A=255`) in one parallel pass — no intermediate `f32` bitmap.
///
/// * `rgba` — destination bytes; must cover at least `stride_px * height * 4`
/// * `stride_px` — row stride in pixels (may be `>= width`)
/// * `width` / `height` — glyph tile size written at column 0 of each row
/// * `flip_y` — when true, destination row 0 samples source `y = height - 1`
pub fn generate_sdf_banded_rgba8(
    shape: &PreparedComponent,
    range: f64,
    rgba: &mut [u8],
    stride_px: u32,
    width: u32,
    height: u32,
    flip_y: bool,
) {
    let stride = stride_px as usize;
    let w = width as usize;
    let h = height as usize;
    assert!(
        w <= stride,
        "width ({width}) must be <= stride_px ({stride_px})"
    );
    assert!(
        rgba.len() >= stride.saturating_mul(h).saturating_mul(4),
        "rgba buffer too small for stride*height*4"
    );
    if w == 0 || h == 0 {
        return;
    }

    let half_range = range * 0.5;
    // Extra half-pixel slack avoids missing edge samples at AABB boundaries (script joins).
    let aabbs = shape.segment_aabbs_expanded(half_range);
    let coarse = aabbs.iter().copied().reduce(Aabb::merge);

    rgba.par_chunks_mut(stride * 4)
        .take(h)
        .enumerate()
        .for_each(|(y_dst, row)| {
            let y_src = if flip_y { h - 1 - y_dst } else { y_dst };
            let scanline = shape.scanline(y_src as f64 + 0.5);
            let mut cursor = scanline.cursor();
            for x in 0..w {
                let fill = cursor.filled(x as f64 + 0.5, FillRule::Nonzero);
                let point = Point::new(x as f64 + 0.5, y_src as f64 + 0.5);
                let sd = banded_signed_distance(shape, point, half_range, &aabbs, coarse);
                let unit = apply_fill_sign_unit((sd / range + 0.5).clamp(0.0, 1.0), fill);
                let v = (unit * 255.0).round() as u8;
                let i = x * 4;
                row[i] = v;
                row[i + 1] = 0;
                row[i + 2] = 0;
                row[i + 3] = 255;
            }
        });
}

fn sampler_msdf<P: Primitive>(shape: &PreparedColoredShape, range: f64, point: Point) -> Rgb<P> {
    let [d_red, d_green, d_blue] = shape.distance3(point);
    let d_red = d_red.signed_pseudo_distance(point);
    let d_green = d_green.signed_pseudo_distance(point);
    let d_blue = d_blue.signed_pseudo_distance(point);
    Rgb([
        signed_distance_to_pixel_value(d_red, range),
        signed_distance_to_pixel_value(d_green, range),
        signed_distance_to_pixel_value(d_blue, range),
    ])
}

fn sampler_mtsdf<P: Primitive>(shape: &PreparedColoredShape, range: f64, point: Point) -> Rgba<P> {
    let [d_red, d_green, d_blue, d_min] = shape.distance4(point);
    let d_red = d_red.signed_pseudo_distance(point);
    let d_green = d_green.signed_pseudo_distance(point);
    let d_blue = d_blue.signed_pseudo_distance(point);
    Rgba([
        signed_distance_to_pixel_value(d_red, range),
        signed_distance_to_pixel_value(d_green, range),
        signed_distance_to_pixel_value(d_blue, range),
        signed_distance_to_pixel_value(d_min.value.distance(), range),
    ])
}

/// Generates a multi-channel signed distance field.
pub fn generate_msdf<P: Primitive, I: GenericImage<Pixel = Rgb<P>>>(
    shape: &PreparedColoredShape,
    range: f64,
    dest: &mut I,
) where
    Rgb<P>: Pixel,
{
    generate(|point| sampler_msdf(shape, range, point), dest)
}

/// Generates a multi-channel signed distance field along with a single-channel
/// (‘true’) SDF.
pub fn generate_mtsdf<P: Primitive, I: GenericImage<Pixel = Rgba<P>>>(
    shape: &PreparedColoredShape,
    range: f64,
    dest: &mut I,
) where
    Rgba<P>: Pixel,
{
    generate(|point| sampler_mtsdf(shape, range, point), dest)
}

#[cfg(test)]
mod tests {
    use image::GrayImage;

    use crate::{
        bezier::{scanline::FillRule, Point, Segment},
        render::correct_sign_sdf,
        shape::{Contour, Shape},
    };

    use super::{generate_sdf, generate_sdf_banded, generate_sdf_banded_rgba8};

    fn unit_square_prepared() -> crate::bezier::prepared::PreparedComponent {
        let contour = Contour {
            segments: vec![
                Segment::line(Point::new(2.0, 2.0), Point::new(14.0, 2.0)),
                Segment::line(Point::new(14.0, 2.0), Point::new(14.0, 14.0)),
                Segment::line(Point::new(14.0, 14.0), Point::new(2.0, 14.0)),
                Segment::line(Point::new(2.0, 14.0), Point::new(2.0, 2.0)),
            ],
        };
        Shape {
            contours: vec![contour],
        }
        .prepare()
    }

    #[test]
    fn banded_sdf_matches_full_after_sign_correction() {
        let prepared = unit_square_prepared();
        let range = 4.0;
        let mut full = GrayImage::new(16, 16);
        let mut banded = GrayImage::new(16, 16);

        generate_sdf(&prepared, range, &mut full);
        correct_sign_sdf(&mut full, &prepared, FillRule::Nonzero);

        generate_sdf_banded(&prepared, range, &mut banded);
        correct_sign_sdf(&mut banded, &prepared, FillRule::Nonzero);

        for y in 0..16 {
            for x in 0..16 {
                let a = full.get_pixel(x, y).0[0];
                let b = banded.get_pixel(x, y).0[0];
                let diff = (a as i16 - b as i16).unsigned_abs();
                assert!(
                    diff <= 1,
                    "pixel ({x},{y}) full={a} banded={b} diff={diff}"
                );
            }
        }
    }

    #[test]
    fn rgba8_flip_matches_banded_with_sign() {
        let prepared = unit_square_prepared();
        let range = 4.0;
        let mut banded = GrayImage::new(16, 16);
        generate_sdf_banded(&prepared, range, &mut banded);
        correct_sign_sdf(&mut banded, &prepared, FillRule::Nonzero);

        let mut rgba = vec![0u8; 16 * 16 * 4];
        generate_sdf_banded_rgba8(&prepared, range, &mut rgba, 16, 16, 16, true);

        for y in 0..16u32 {
            for x in 0..16u32 {
                let src_y = 15 - y;
                let expected = banded.get_pixel(x, src_y).0[0];
                let got = rgba[((y * 16 + x) * 4) as usize];
                let diff = (expected as i16 - got as i16).unsigned_abs();
                assert!(
                    diff <= 1,
                    "pixel ({x},{y}) expected={expected} got={got}"
                );
            }
        }
    }

    #[test]
    fn signed_distance_only_matches_tracked_distance_magnitude() {
        let prepared = unit_square_prepared();
        let p = Point::new(8.5, 8.5);
        let tracked = prepared.distance(p);
        let only = prepared.signed_distance_only(p);
        assert!((tracked.value.distance() - only.distance()).abs() < 1e-9);
    }
}

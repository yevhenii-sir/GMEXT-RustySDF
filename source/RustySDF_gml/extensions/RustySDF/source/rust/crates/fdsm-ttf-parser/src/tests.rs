use std::fs::{self, File};

use fdsm::{generate::generate_msdf, render::render_msdf, shape::Shape};
use image::{EncodableLayout, GrayImage, ImageBuffer, Pixel, PixelWithColorType, RgbImage};
use na::{Affine2, Similarity2, Vector2};

use ttf_parser::Face;

use crate::load_shape_from_face;

// Adapted from the tests in msdfgen-rs:
// <https://github.com/katyo/msdfgen-rs/blob/master/src/lib.rs>

type Previewer<P> = fn(&ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>, &mut GrayImage, f64);

fn save_bitmap_and_preview<P>(
    prefix: &str,
    name: &str,
    suffix: &str,
    image: &ImageBuffer<P, Vec<P::Subpixel>>,
    px_range: f64,
    previewer: Option<Previewer<P>>,
) where
    [P::Subpixel]: EncodableLayout,
    P: Pixel + PixelWithColorType,
{
    fs::create_dir_all("output").unwrap();

    image
        .save(format!("output/{prefix}-{name}-{suffix}.png"))
        .unwrap();

    if let Some(previewer) = previewer {
        let mut preview = GrayImage::new(image.width() * 10, image.height() * 10);

        previewer(image, &mut preview, px_range);

        preview
            .save(format!("output/{prefix}-{name}-{suffix}-preview.png"))
            .unwrap();
    }
}

fn test_font_glyph(prefix: &str, face: &Face, ch: char, expected_error: f64) {
    use image::{buffer::ConvertBuffer, Rgb32FImage, RgbaImage};

    use fdsm::{
        bezier::scanline::FillRule,
        correct_error::{correct_error_msdf, ErrorCorrectionConfig},
        generate::{generate_mtsdf, generate_sdf},
        render::{correct_sign_msdf, correct_sign_mtsdf, render_sdf},
        transform::Transform,
    };

    let glyph_id = face.glyph_index(ch).unwrap();
    let name = face
        .glyph_name(glyph_id)
        .map(String::from)
        .unwrap_or_else(|| ch.to_string());
    let bbox = face.glyph_bounding_box(glyph_id).unwrap();
    let mut shape = load_shape_from_face(face, glyph_id).expect("no shape?");

    const RANGE: f64 = 4.0;
    const SHRINKAGE: f64 = 16.0;
    let transformation = na::convert::<_, Affine2<f64>>(Similarity2::new(
        Vector2::new(
            RANGE - bbox.x_min as f64 / SHRINKAGE,
            RANGE - bbox.y_min as f64 / SHRINKAGE,
        ),
        0.0,
        1.0 / SHRINKAGE,
    ));
    let width = ((bbox.x_max as f64 - bbox.x_min as f64) / SHRINKAGE + 2.0 * RANGE).ceil() as u32;
    let height = ((bbox.y_max as f64 - bbox.y_min as f64) / SHRINKAGE + 2.0 * RANGE).ceil() as u32;
    #[cfg(feature = "visualize")]
    let orig_shape = shape.clone();
    shape.transform(&transformation);

    let colored_shape = Shape::edge_coloring_simple(shape.clone(), 0.03, 69441337420);
    dbg!(&colored_shape);

    let mut sdf = GrayImage::new(width, height);
    let prepared_shape = shape.prepare();
    generate_sdf(&prepared_shape, RANGE, &mut sdf);
    save_bitmap_and_preview(prefix, &name, "sdf", &sdf, RANGE, Some(render_sdf));

    let mut msdf = Rgb32FImage::new(width, height);
    let prepared_colored_shape = colored_shape.prepare();
    generate_msdf(&prepared_colored_shape, RANGE, &mut msdf);
    correct_error_msdf(
        &mut msdf,
        &colored_shape,
        &prepared_colored_shape,
        RANGE,
        &ErrorCorrectionConfig::default(),
    );
    correct_sign_msdf(&mut msdf, &prepared_colored_shape, FillRule::Nonzero);
    let msdf = msdf.convert();
    save_bitmap_and_preview(prefix, &name, "msdf", &msdf, RANGE, Some(render_msdf));

    let mut mtsdf = RgbaImage::new(width, height);
    let prepared_colored_shape = colored_shape.prepare();
    generate_mtsdf(&prepared_colored_shape, RANGE, &mut mtsdf);
    correct_sign_mtsdf(&mut mtsdf, &prepared_colored_shape, FillRule::Nonzero);
    save_bitmap_and_preview(prefix, &name, "mtsdf", &mtsdf, RANGE, None);

    #[cfg(feature = "visualize")]
    {
        use fdsm::visualize::generate_vis;
        use na::Scale2;
        let colored_shape = Shape::edge_coloring_simple(orig_shape, 0.03, 69441337420);
        let mut vis = RgbImage::new(width * 10, height * 10);
        let transformation =
            na::convert::<_, Affine2<f64>>(Scale2::new(10.0, 10.0)) * transformation;
        generate_vis(&colored_shape, &transformation, &mut vis);
        save_bitmap_and_preview(prefix, &name, "voronoi", &vis, RANGE, None);
    }
}

// Test with reference implementation (msdfgen)
#[cfg(not(miri))]
fn test_reference_font_glyph(prefix: &str, face: &Face, ch: char) {
    use crate::tests::msdf_import_from_ttf_parser::glyph_shape_vendored;

    let glyph_id = face.glyph_index(ch).unwrap();
    let name = face
        .glyph_name(glyph_id)
        .map(String::from)
        .unwrap_or_else(|| ch.to_string());
    let bbox = face.glyph_bounding_box(glyph_id).unwrap();

    let mut shape = glyph_shape_vendored(face, glyph_id).unwrap();
    shape.edge_coloring_simple(0.03, 69441337420);

    const RANGE: f64 = 4.0;
    const SHRINKAGE: f64 = 16.0;
    let framing = msdfgen::Framing {
        projection: msdfgen::Projection {
            scale: msdfgen::Vector2 {
                x: 1.0 / SHRINKAGE,
                y: 1.0 / SHRINKAGE,
            },
            translate: msdfgen::Vector2 {
                x: RANGE * SHRINKAGE - bbox.x_min as f64,
                y: RANGE * SHRINKAGE - bbox.y_min as f64,
            },
        },
        range: RANGE * SHRINKAGE,
    };
    let width = ((bbox.x_max as f64 - bbox.x_min as f64) / SHRINKAGE + 2.0 * RANGE).ceil() as u32;
    let height = ((bbox.y_max as f64 - bbox.y_min as f64) / SHRINKAGE + 2.0 * RANGE).ceil() as u32;

    let mut msdf = msdfgen::Bitmap::new(width, height);
    let config = msdfgen::MsdfGeneratorConfig::default();
    shape.generate_msdf(&mut msdf, framing, config);
    shape.correct_sign(&mut msdf, framing, msdfgen::FillRule::NonZero);
    let mut fh = File::create(format!("output/{prefix}-{name}-msdf_ref.png")).unwrap();
    msdf.write_png(&mut fh).unwrap();

    let mut preview =
        msdfgen::Bitmap::<msdfgen::Gray<f32>>::new(msdf.width() * 10, msdf.height() * 10);
    msdf.render(&mut preview, RANGE, 0.5);
    let mut fh2 = File::create(format!("output/{prefix}-{name}-msdf_ref-preview.png")).unwrap();
    preview.write_png(&mut fh2).unwrap();
}

#[cfg(miri)]
fn test_reference_font_glyph(_prefix: &str, _face: &Face, _ch: char) {
    eprintln!("Skipping reference font tests in Miri");
}

#[test]

fn test_glyphs_noto() {
    let font = Face::parse(notosans::REGULAR_TTF, 0).unwrap();
    for c in 'A'..='Z' {
        test_font_glyph("notosans", &font, c, 0.05);
        test_reference_font_glyph("notosans", &font, c);
    }
}

#[test]

fn test_glyphs_inter() {
    let font = Face::parse(assets::INTER, 0).unwrap();
    for c in 'A'..='Z' {
        test_font_glyph("inter", &font, c, 0.05);
        test_reference_font_glyph("inter", &font, c);
    }
}

#[test]

fn test_glyphs_noto_serif_sinhala() {
    let font = Face::parse(assets::NOTO_SERIF_SINHALA, 0).unwrap();
    for c in ['a', 'b', '+'] {
        test_font_glyph("notoserif-sinhala", &font, c, 0.05);
        test_reference_font_glyph("notoserif-sinhala", &font, c);
    }
}

mod assets {
    pub static INTER: &[u8] = include_bytes!("../../assets/Inter-Regular.otf");
    // Ah, Noto Serif Sinhala. Why this specific font? Because someone reported an issue with Caxton using this font, and I noticed that it had some rendering artifacts. And here I am, using it to test MSDF error correction. I’m not even testing this with Sinhala glyphs at the moment. :concern:
    pub static NOTO_SERIF_SINHALA: &[u8] =
        include_bytes!("../../assets/noto_serif_sinhala_regular.ttf");
}

#[cfg(not(miri))]
include!("./_msdf_import_from_ttf_parser.rs");

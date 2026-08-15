# Changelog for `fdsm-ttf-parser`

## Unreleased

* Update repository link

## 0.2.0 (2025-10-02)

`load_shape_from_face` now returns `Option<Shape<Countour>>` instead of `Shape<Contour>` (`None` if the glyph was not found)

## 0.1.2 (2025-05-24)

Downgrade edition to 2021 to (hopefully) support Rust 1.82

## 0.1.1 (2025-05-24)

Re-export the `ttf-parser` dependency

## 0.1.0 (2025-05-23)

Initial release.

# GMEXT-RustySDF

GameMaker extension for high-quality **SDF / MSDF / MTSDF** text: font load, HarfBuzz shaping + bidi, glyph atlas, async render queue, and rich-text layout → vertex buffers for GML shaders.

Native core is **Rust** (extgen `nativeBackend: rust`), with a GML wrapper in the sample project.

## Demo

[Vimeo walkthrough](https://vimeo.com/1186771421) — **older recording**. The toolchain has moved on: **extgen regeneration is now about 10–30× faster** than what the video shows.

## Features

- Font load from buffers, fallback font chains
- Text shaping (HarfBuzz) and bidirectional text
- SDF / PSDF / MSDF / MTSDF glyph render into GM buffers
- Async glyph request / poll
- CPU atlas pack (GML owns surfaces); dirty meta/pixels polling
- Rich text: parse, layout, images, metrics, per-page vertices

## Platforms

| Platform | Status |
|----------|--------|
| Windows | Supported (`.dll`) |
| macOS | Supported (`.dylib`) |
| Linux | Supported (`.so`) |
| Android | Supported (JNI + `.so` per ABI) |
| iOS | Supported (ObjC + embedded `RustySDF_Rust.xcframework`) |
| tvOS | Supported (same pattern as iOS) |
| HTML5 / consoles | Not in this Rust build |

## Layout (extension)

```
extensions/RustySDF/
  RustySDF.yy
  source/          # extgen root (spec, rust/, scripts/)
  AndroidSource/
  iOSSource/ / tvOSSource/
  iOSSourceFromMac/ / tvOSSourceFromMac/   # built Apple frameworks
```

Build helpers: `source/scripts/build_*.{bat,sh}` (Windows, Android, macOS, Linux, iOS, tvOS).

//! 80-byte rich SDF vertex + 20-byte plain (pos/color/uv) layouts.

use std::mem;

/// Interleaved rich vertex: 20 × f32 = 80 bytes (little-endian).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RichVertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub cr: f32,
    pub cg: f32,
    pub cb: f32,
    pub ca: f32,
    pub out_r: f32,
    pub out_g: f32,
    pub out_b: f32,
    pub out_a: f32,
    pub gr: f32,
    pub gg: f32,
    pub gb: f32,
    pub ga: f32,
    pub boldness: f32,
    pub outline_w: f32,
    pub glow_rad: f32,
    pub is_sdf: f32,
}

pub const RICH_VERTEX_STRIDE: usize = 80;

/// Plain GM format: position + colour + texcoord = 20 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PlainVertex {
    pub x: f32,
    pub y: f32,
    pub color: u32, // RGBA8 little-endian (R,G,B,A bytes)
    pub u: f32,
    pub v: f32,
}

pub const PLAIN_VERTEX_STRIDE: usize = 20;
pub const PLAIN_WHITE: u32 = 0xFFFF_FFFF;

impl RichVertex {
    pub fn new(
        x: f32,
        y: f32,
        u: f32,
        v: f32,
        cr: f32,
        cg: f32,
        cb: f32,
        ca: f32,
        out_r: f32,
        out_g: f32,
        out_b: f32,
        out_a: f32,
        gr: f32,
        gg: f32,
        gb: f32,
        ga: f32,
        boldness: f32,
        outline_w: f32,
        glow_rad: f32,
        is_sdf: f32,
    ) -> Self {
        Self {
            x,
            y,
            u,
            v,
            cr,
            cg,
            cb,
            ca,
            out_r,
            out_g,
            out_b,
            out_a,
            gr,
            gg,
            gb,
            ga,
            boldness,
            outline_w,
            glow_rad,
            is_sdf,
        }
    }

    pub fn as_bytes(&self) -> [u8; RICH_VERTEX_STRIDE] {
        unsafe { mem::transmute(*self) }
    }
}

impl PlainVertex {
    pub fn as_bytes(&self) -> [u8; PLAIN_VERTEX_STRIDE] {
        unsafe { mem::transmute(*self) }
    }
}

/// Push one axis-aligned textured quad as 2 triangles (6 verts) — rich format.
pub fn push_quad(
    out: &mut Vec<u8>,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    cr: f32,
    cg: f32,
    cb: f32,
    ca: f32,
    out_r: f32,
    out_g: f32,
    out_b: f32,
    out_a: f32,
    gr: f32,
    gg: f32,
    gb: f32,
    ga: f32,
    boldness: f32,
    outline_w: f32,
    glow_rad: f32,
    is_sdf: f32,
) {
    let mk = |x: f32, y: f32, u: f32, v: f32| {
        RichVertex::new(
            x, y, u, v, cr, cg, cb, ca, out_r, out_g, out_b, out_a, gr, gg, gb, ga, boldness,
            outline_w, glow_rad, is_sdf,
        )
    };
    for v in [
        mk(x1, y1, u1, v1),
        mk(x2, y1, u2, v1),
        mk(x1, y2, u1, v2),
        mk(x2, y1, u2, v1),
        mk(x2, y2, u2, v2),
        mk(x1, y2, u1, v2),
    ] {
        out.extend_from_slice(&v.as_bytes());
    }
}

/// Push plain pos/color/uv quad (6 verts).
pub fn push_plain_quad(
    out: &mut Vec<u8>,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    color: u32,
) {
    let mk = |x: f32, y: f32, u: f32, v: f32| PlainVertex {
        x,
        y,
        color,
        u,
        v,
    };
    for v in [
        mk(x1, y1, u1, v1),
        mk(x2, y1, u2, v1),
        mk(x1, y2, u1, v2),
        mk(x2, y1, u2, v1),
        mk(x2, y2, u2, v2),
        mk(x1, y2, u1, v2),
    ] {
        out.extend_from_slice(&v.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strides() {
        assert_eq!(mem::size_of::<RichVertex>(), RICH_VERTEX_STRIDE);
        assert_eq!(mem::size_of::<PlainVertex>(), PLAIN_VERTEX_STRIDE);
    }
}

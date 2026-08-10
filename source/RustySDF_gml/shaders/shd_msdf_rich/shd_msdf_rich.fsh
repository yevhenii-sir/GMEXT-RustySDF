#ifdef GL_ES
precision mediump float;
#endif

varying vec2 v_vTexcoord;
varying vec4 v_vCoreColor;
varying vec4 v_vOutlineColor;
varying vec4 v_vGlowColor;
varying vec4 v_vParams;

uniform float u_global_alpha;
uniform int u_render_pass;

#if __VERSION__ >= 300
#define texture2D texture
#endif

float median(vec3 v) {
    return max(min(v.r, v.g), min(max(v.r, v.g), v.b));
}

void main()
{
    if (v_vParams.w < 0.5) {
        if (u_render_pass == 1) {
            gl_FragColor = vec4(v_vCoreColor.rgb, v_vCoreColor.a * u_global_alpha);
        } else {
            discard;
        }
        return;
    }

    vec4 msdf_sample = texture2D(gm_BaseTexture, v_vTexcoord);
    float dist = median(msdf_sample.rgb);
    
    // ВАЖНО: Для MSDF лучше использовать fwidth, если расширение включено.
    // Если нет, оставляем как есть.
    float spread = 0.08; 
    spread = max(spread * 0.75, 0.001);

    float boldness = v_vParams.x;
    float outline_width = v_vParams.y;
    float glow_radius = v_vParams.z;

    if (u_render_pass == 0) {
        vec4 eff_color = vec4(0.0);
        if (v_vGlowColor.a > 0.0 && glow_radius > 0.0) {
            float glow_alpha = smoothstep(boldness - outline_width - glow_radius, boldness - outline_width, dist) * v_vGlowColor.a;
            eff_color = vec4(v_vGlowColor.rgb, glow_alpha);
        }
        if (v_vOutlineColor.a > 0.0 && outline_width > 0.0) {
            float out_a = smoothstep(boldness - outline_width - spread, boldness - outline_width + spread, dist) * v_vOutlineColor.a;
            eff_color.rgb = mix(eff_color.rgb, v_vOutlineColor.rgb, out_a);
            eff_color.a = max(eff_color.a, out_a);
        }
        eff_color.a *= u_global_alpha;
        if (eff_color.a <= 0.0) discard;
        gl_FragColor = eff_color;
    } else {
        float core_alpha = smoothstep(boldness - spread, boldness + spread, dist) * v_vCoreColor.a;
        vec4 core_color = vec4(v_vCoreColor.rgb, core_alpha * u_global_alpha);
        if (core_color.a <= 0.0) discard;
        gl_FragColor = core_color;
    }
}
//#extension GL_OES_standard_derivatives : enable

#ifdef GL_ES
precision mediump float;
#endif

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

uniform vec4 u_core_color;
uniform float u_boldness;

uniform bool u_enable_outline;
uniform float u_outline_width;
uniform vec4 u_outline_color;

uniform bool u_enable_glow;
uniform float u_glow_radius;
uniform vec4 u_glow_color;

#if __VERSION__ >= 300
#define texture2D texture
#endif

float median(vec3 v)
{
    return max(min(v.r, v.g), min(max(v.r, v.g), v.b));
}

void main()
{
    vec4 msdf_sample = texture2D(gm_BaseTexture, v_vTexcoord);
    float dist = median(msdf_sample.rgb);
    
    float spread = fwidth(dist);
    spread = max(spread * 0.75, 0.001);
   
    float core_alpha = smoothstep(u_boldness - spread, u_boldness + spread, dist) * u_core_color.a;
   
   vec4 final_color = vec4(u_core_color.rgb, 0.0);
    
    if (u_enable_glow) 
    {
        float glow_alpha = smoothstep(u_boldness - u_outline_width - u_glow_radius, 
                                      u_boldness - u_outline_width, dist) * u_glow_color.a;
        final_color = vec4(u_glow_color.rgb, glow_alpha);
    }
    
    if (u_enable_outline) 
    {
        float out_a = smoothstep(u_boldness - u_outline_width - spread, 
                                 u_boldness - u_outline_width + spread, dist) * u_outline_color.a;
                                 
        final_color.rgb = mix(final_color.rgb, u_outline_color.rgb, out_a);
        final_color.a = max(final_color.a, out_a);
    }
    
    final_color.rgb = mix(final_color.rgb, u_core_color.rgb, core_alpha);
    final_color.a = max(final_color.a, core_alpha);
    
    gl_FragColor = final_color * v_vColour;
}
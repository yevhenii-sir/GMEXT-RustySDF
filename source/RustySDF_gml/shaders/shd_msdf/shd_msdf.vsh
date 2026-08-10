attribute vec3 in_Position;
attribute vec4 in_Colour;
attribute vec2 in_TextureCoord;

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

uniform vec4 u_transform; // x: scale_x, y: scale_y, z: x_offset, w: y_offset
uniform vec2 u_rotation;  // x: dcos(rot), y: dsin(rot)

void main()
{
    vec2 pos = in_Position.xy;
    
    pos *= u_transform.xy;
    
    float rx = pos.x * u_rotation.x + pos.y * u_rotation.y;
    float ry = -pos.x * u_rotation.y + pos.y * u_rotation.x;
    
    pos = vec2(rx, ry) + u_transform.zw;
    
    gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * vec4(pos, in_Position.z, 1.0);
    
    v_vColour = in_Colour;
    v_vTexcoord = in_TextureCoord;
}
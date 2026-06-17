shader_type spatial;

uniform sampler2D albedo_texture : source_color;
uniform vec2 scroll_speed = vec2(0.05, 0.0);
uniform float wobble_strength = 0.02;
uniform float wobble_speed = 2.0;
uniform float wobble_scale = 10.0;

void fragment() {
    vec2 uv = UV;
    uv.x += sin(uv.y * wobble_scale + TIME * wobble_speed) * wobble_strength;
    uv.y += cos(uv.x * wobble_scale + TIME * wobble_speed) * wobble_strength;
    uv += scroll_speed * TIME;
    ALBEDO = texture(albedo_texture, uv).rgb;
}
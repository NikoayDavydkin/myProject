precision mediump float;

uniform float u_time;

varying vec4 pos;

void main() {
    float r = 0.0;
    float g = 0.0;
    float b = 0.0;
    float a = (length(pos.xy) <= 1.0 && length(pos.xy) >= 0.51)? 1.0 : 0.0;

    gl_FragColor = vec4(r, g, b, a);
}

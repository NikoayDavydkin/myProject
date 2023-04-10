precision mediump float;

attribute vec4 a_position;
uniform float radius;

varying vec4 pos;

void main() {
    gl_Position = vec4(a_position.xy + a_position.zw * radius, 0.0, 1.0);
    pos = vec4(a_position.zw, 0.0, 1.0);
}

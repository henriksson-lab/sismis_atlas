
precision mediump float;

attribute vec3 a_position;
varying highp vec3 color;


vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}


void main() {

    //Transform from 0..1024 world coordinates to [-1,1] camera coordinates
    vec2 wh = vec2(
        1.0/1023.0 * 2.0,
        1.0/1023.0 * 2.0
    );

    vec2 a_position2 = vec2(a_position.x, a_position.y);

    vec2 scaled = a_position2 * wh;
    vec2 transformed = scaled - vec2(1.0,1.0);
    gl_Position = vec4(transformed.x, -transformed.y, 0.0, 1.0);   // Invert camera y to match 

    //Set size of points
    gl_PointSize = 5.0;


    //Set color via HSV. Then forward color
    vec3 hsv = vec3(a_position.z, 1.0, 1.0);
    color = hsv2rgb(hsv);
}



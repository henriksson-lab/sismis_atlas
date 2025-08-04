
precision mediump float;

attribute vec3 a_position;
varying highp vec3 color;

uniform float u_camera_x;
uniform float u_camera_y;
uniform float u_camera_zoom;


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

    vec2 a_cam_pos = vec2(u_camera_x, u_camera_y);
    vec2 a_position2 = vec2(a_position.x, a_position.y);
    vec2 a_position3 = a_position2 - a_cam_pos;

    vec2 scaled = a_position3 * wh;
    vec2 scaled2 = scaled * u_camera_zoom;


    
    vec2 transformed = scaled2 - vec2(1.0,1.0);
    gl_Position = vec4(transformed.x, -transformed.y, 0.0, 1.0);   // Invert camera y to match 

    //Set size of points
    gl_PointSize = 5.0;


    //Set color via HSV. Then forward color
    vec3 hsv = vec3(a_position.z, 1.0, 1.0);
    color = hsv2rgb(hsv);
}



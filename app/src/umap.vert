
precision mediump float;

attribute vec2 a_position;

void main() {

    //Transform from 0..1024 world coordinates to [-1,1] camera coordinates
    vec2 wh = vec2(
        1.0/1023.0 * 2.0,
        1.0/1023.0 * 2.0
    );
    vec2 scaled = a_position * wh;
    vec2 transformed = scaled - vec2(1.0,1.0);
    gl_Position = vec4(transformed.x, -transformed.y, 0.0, 1.0);   // Invert camera y to match 

    //Set size of points
    gl_PointSize = 5.0;
}






// uniform mat3 u_matrix;



/*
    vec2 base = vec2(-1.0,-1.0);
    vec2 wh = vec2(800.0,600.0);

    vec2 trans = a_position - base;    // vec2(-1.0,-1.0);
    vec2 scaled = trans*wh;
    gl_Position = vec4(scaled, 0.0, 1.0);
*/



/*


  // starting with the view projection matrix
  // compute a matrix for the F
  var matrix = m4.translate(viewProjectionMatrix, x, 0, y);
 
  // Set the matrix.
  gl.uniformMatrix4fv(matrixLocation, false, matrix);

//   gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);

*/


/*
orig

    gl_Position = vec4(a_position, 0.0, 1.0);
    gl_PointSize = 1.0;


*/
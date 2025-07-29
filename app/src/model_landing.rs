use crate::core_model::*;

use yew::prelude::*;


////////////////////////////////////////////////////////////
/// If condition is met, return "selected", otherwise "". For OPTION
pub fn selected_if(cond: bool) -> String {
    if cond {
        "selected".to_string()
    } else {
        "".to_string()
    }
}   // can do true false https://yew.rs/docs/concepts/html 



impl Model {


    ////////////////////////////////////////////////////////////
    /// x
    pub fn view_landing_page(&self, _ctx: &Context<Self>) -> Html {

        html! {

            <div class="landingdiv">

                <p style="color: rgb(0, 150, 255);">
                    {"test"}
                </p>

                /*
                
                
                
                // init colors
const vertexColors = [
  vec4(0.0, 0.0, 0.0, 1.0), // black
  vec4(1.0, 0.0, 0.0, 1.0), // red
  vec4(1.0, 1.0, 0.0, 1.0), // yellow
  vec4(0.0, 1.0, 0.0, 1.0), // green
  vec4(0.0, 0.0, 0.0, 1.0), // black
  vec4(1.0, 0.0, 0.0, 1.0), // red
  vec4(1.0, 1.0, 0.0, 1.0), // yellow
  vec4(0.0, 1.0, 0.0, 1.0), // green
];
const cBuffer = gl.createBuffer();


// continued
// create buffer to store colors and reference it to "vColor" which is in GLSL
gl.bindBuffer(gl.ARRAY_BUFFER, cBuffer);
gl.bufferData(gl.ARRAY_BUFFER, flatten(vertexColors), gl.STATIC_DRAW);

const vColor = gl.getAttribLocation(program, "vColor");
gl.vertexAttribPointer(vColor, 4, gl.FLOAT, false, 0, 0);
gl.enableVertexAttribArray(vColor);


                 */


/*
/// GLSL??

attribute  vec4 vColor;

void main()
{
  fColor = vColor;
}
 */


/*

https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/WebGL_model_view_projection


 */

            </div>
        }
    }



}

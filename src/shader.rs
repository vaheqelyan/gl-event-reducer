pub(crate) const VERTEX_SOURCE: &str = r#"
            #version 430 core
            layout(location = 0) in vec2 position;
            layout(location = 1) in vec2 texCoord;
            layout(location = 2) in float layer;
            layout(location = 3) in vec3 rgb;
            out vec3 color;
            out float layer_get;
            out vec2 uv;
            uniform mat4 MVP;

            void main()
            {
                gl_Position = vec4(position.x, position.y, 0.0, 1.0);
                uv = texCoord;
                layer_get = layer;
                color = rgb;
            }
            "#;

pub(crate) const FRAGMENT_SOURCE: &str = r#"
#version 430 core

precision highp float;

out vec4 frag_color;

in vec2 uv;
in vec3 color; 
in float layer_get;

layout(binding = 0) uniform sampler2DArray textureArray;

void main() {

  if (layer_get == 0.1) {
    frag_color = vec4(color, 1.0);
  } else {

vec4 t1 = texture(textureArray, vec3(uv, layer_get));

float w = 1.0 * length ( vec2 ( dFdx ( t1.r ) , dFdy ( t1.r )) );
float a = smoothstep(0.5 - w, 0.5 + w, t1.r);

frag_color = vec4(0.0, 0.0, 0.0, a * 1.2);
//frag_color = vec4(0.0, 0.0, 0.0, 1.0);

  }

}
"#;

pub const VERTEX_SHADER: &str = r##"#version 300 es
    in vec3 a_position;
    in vec3 a_normal;

    struct camera {
        mat4 model;
        mat4 view;
        mat4 projection;
    };

    uniform camera u_camera;

    out vec3 v_normal;
    out vec3 v_position;

    vec3 transform_normal(mat4 model_view, vec3 normal) {
        return normalize(transpose(inverse(mat3(model_view))) * normal);
    }

    void main() {
        mat4 mvp = u_camera.projection * u_camera.view * u_camera.model;

        v_position = vec3(u_camera.model * vec4(a_position, 1.0));
        v_normal = transform_normal(u_camera.model, a_normal);

        gl_Position = mvp * vec4(a_position, 1.0);
    }
"##;

pub const FRAGMENT_SHADER: &str = r##"#version 300 es
    precision highp float;

    in vec3 v_normal;
    in vec3 v_position;

    out vec4 out_color;

    vec3 light_position = vec3(0, 0, 10);

    vec3 object_color = vec3(0.8, 0.8, 0.8);
    vec3 diffuse_color = vec3(0.8, 0.9, 0.4);
    vec3 ambient_color = vec3(0.4, 0.7, 0.4);

    float ambient_strength = 0.1;

    void main() {
        vec3 light_dir = normalize(light_position - v_position);
        float diffuse_strength = clamp(dot(light_dir, v_normal), 0.1, 1.0);

        vec3 light = ambient_color * ambient_strength + diffuse_color * diffuse_strength;
        out_color = vec4(object_color * light, 1);
    }
"##;

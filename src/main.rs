pub mod map_parser;

use std::ffi::CString;
use std::time::Instant;

use gl::types::*;

use cgmath::{Matrix4, Point3, Vector3, Vector4};

use sdl2::event::Event;
use sdl2::video::SwapInterval;
use sdl2::keyboard::Keycode;

use map_parser::ObjFile;

// Shader sources
static VS_SRC: &'static str = "
#version 150
in vec3 position;
uniform mat4 transform_matrix;

void main() {
    gl_Position = transform_matrix * vec4(position, 1.0);
}";

static FS_SRC: &'static str = "
#version 150
out vec4 out_color;

void main() {
    out_color = vec4(0.0, 0.0, 1.0, 1.0);
}";

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

pub fn main() {
    if false {
    let mut obj = ObjFile::default();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005832.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004633.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0006130.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0006133.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005733.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003734.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004141.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004736.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002732.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003132.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004339.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003035.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004940.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005135.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002837.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004529.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004439.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002731.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002726.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002940.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002627.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004136.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005228.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002240.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004133.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002936.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005132.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002439.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003038.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002843.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002641.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003539.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004640.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005437.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0006029.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002939.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003135.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005126.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003837.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004932.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004231.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002836.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003327.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005227.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005930.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005139.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004032.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002638.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004230.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003233.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005337.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004436.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003935.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005229.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005428.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003638.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003333.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004833.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002535.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003230.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004829.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004028.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002934.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003736.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004828.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002827.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004832.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005326.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002840.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003332.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004629.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005933.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003232.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004233.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005241.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004835.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002631.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003437.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005435.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005339.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002339.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004836.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004228.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004635.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005336.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004038.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004138.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005038.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002440.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005131.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004934.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003737.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004334.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003933.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003234.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002839.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005628.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003337.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003331.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002434.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004636.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005332.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003129.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003835.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005637.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004538.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004240.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002538.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005439.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002531.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005029.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005239.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0006033.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004437.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004433.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003428.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004936.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002930.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002541.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002529.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002826.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004034.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003632.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004938.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002436.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002635.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0006030.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004341.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002628.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003937.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005535.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003336.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002636.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004931.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005334.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005430.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005539.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003739.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005530.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004031.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002728.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002738.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003434.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003329.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005035.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003133.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004229.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003328.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003532.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004429.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003137.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002629.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003929.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004531.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003027.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005133.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003238.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003731.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004735.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003340.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002337.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003536.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005130.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004237.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003330.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004830.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005233.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002634.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002643.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005831.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002338.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004135.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002542.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004533.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004027.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003039.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005238.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003429.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002735.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005027.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002637.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004733.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002941.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003030.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005434.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005529.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005141.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004641.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0006031.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004039.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004238.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004530.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003342.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004434.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004232.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002841.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003431.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004132.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003441.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002539.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003339.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003938.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003034.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005932.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004030.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003830.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002341.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004738.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003633.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002937.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005034.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003241.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002828.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004532.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004431.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004338.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003134.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002633.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003533.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004239.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005427.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003440.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003136.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004740.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004541.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005331.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004330.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004234.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003042.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004430.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004142.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003740.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005329.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003040.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003634.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004236.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004732.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002943.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005833.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004037.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004941.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003436.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002443.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004841.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003840.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002537.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004739.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002942.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005234.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003833.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005037.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005636.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002926.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004137.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004528.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003227.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003540.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002336.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003240.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003242.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003433.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002536.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004929.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002239.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005538.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002543.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003033.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004634.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004441.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005630.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005340.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003128.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002441.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005731.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005138.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003442.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004040.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003228.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004630.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005629.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005536.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003028.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005028.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002727.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003432.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004331.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003738.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004242.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003029.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002737.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005634.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005235.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004731.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004134.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004930.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0006032.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004628.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003341.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005032.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005226.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003831.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004537.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004042.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003427.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005328.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005632.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003439.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003237.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004131.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004728.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005137.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005327.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003940.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005127.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003939.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004438.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002533.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002730.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004639.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004035.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004942.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002644.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003626.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004535.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003032.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003338.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005631.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005033.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003836.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004734.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005730.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005039.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004127.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002642.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004730.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004637.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005330.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003535.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004834.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003127.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002630.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002742.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003435.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003335.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004435.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005931.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002743.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002740.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004631.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005834.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004329.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003229.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004128.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004036.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004935.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004840.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002632.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004235.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005333.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002830.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003026.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005040.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004241.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003036.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003031.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003625.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002639.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003733.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005429.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005236.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005231.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005734.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003531.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003934.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005230.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002834.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002835.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003931.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003932.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005436.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004432.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003140.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003141.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004337.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003534.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005635.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004332.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005335.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003635.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005830.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002640.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004130.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003838.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004539.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003430.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002829.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005030.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002532.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005432.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002931.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004536.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004838.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005735.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004837.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004335.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003832.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004440.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005136.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004033.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002933.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002929.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004729.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004140.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003138.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005338.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005128.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004129.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002340.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005433.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004336.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005431.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002838.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003732.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004041.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003438.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0006131.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003941.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003239.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005528.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003139.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002833.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004927.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005533.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004632.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004933.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004741.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005026.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003142.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003936.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003130.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002544.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003735.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005041.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003834.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003537.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004831.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004340.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002842.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002733.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002734.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002739.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002438.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005532.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002928.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005534.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003131.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003231.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005732.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005633.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005829.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005929.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002540.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005934.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002528.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002530.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004839.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003639.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002435.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005140.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005240.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004737.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005031.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003235.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005237.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003839.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005426.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004928.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003041.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002741.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002442.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005438.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004939.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005134.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002534.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005531.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004534.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002832.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002831.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003236.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003930.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002927.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004029.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003334.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002932.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003637.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005537.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002935.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003538.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005729.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003037.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004638.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005232.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002736.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0003636.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005036.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005527.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004937.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002729.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002938.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004540.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004333.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0004139.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0006132.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0002437.obj").unwrap();
obj.load("/home/pleb/recastnavigation/build/RecastDemo/meshes_all/map0005129.obj").unwrap();
    return;
}

    // Create an SDL context
    let sdl_context = sdl2::init().unwrap();

    // Get access to the video subsystem
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window
    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut vertex_data = Vec::new();

    for _ in 0..1000 {
        vertex_data.push(-2f32);
        vertex_data.push(0f32);
        vertex_data.push(0f32);
        vertex_data.push(0f32);
        vertex_data.push(1f32);
        vertex_data.push(0f32);
        vertex_data.push(1f32);
        vertex_data.push(0f32);
        vertex_data.push(0f32);
    }

    // Create the GL context
    let _gl = window.gl_create_context().unwrap();

    // Turn off vsync
    video_subsystem.gl_set_swap_interval(SwapInterval::Immediate).unwrap();

    // Load the GL procedure addresses
    gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    
    // Get the event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertex_data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0,
                                 CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program,
                                             CString::new("position")
                                             .unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            std::ptr::null(),
        );
    }

    // Name of the camera uniform
    let transform_matrix_name = CString::new("transform_matrix").unwrap();

    let mut origin: Point3<f32> = Point3::new(0., 0., -1.0);
    let mut target: Point3<f32> = Point3::new(0., 0., 0.);

    fn view_frustum(field_of_view: f32, aspect_ratio: f32,
                    z_near: f32, z_far: f32) -> Matrix4<f32> {
        Matrix4::from_cols(
            Vector4::new(1.0 / field_of_view.tan(), 0., 0., 0.),
            Vector4::new(0., aspect_ratio / field_of_view.tan(), 0., 0.),
            Vector4::new(0., 0., (z_far + z_near) / (z_far - z_near), 1.0),
            Vector4::new(0., 0., -2. * z_far * z_near / (z_far - z_near), 0.),
        )
    }
    
    // Find the transform matrix location
    let transform_matrix_loc = unsafe {
        gl::GetUniformLocation(program, transform_matrix_name.as_ptr())
    };

    let update_transforms = |origin: Point3<f32>, target: Point3<f32>| {
        let proj_matrix: Matrix4<f32> =
            view_frustum(45f32.to_degrees(), 1.0, 0.01, 2.0);
        let mut view_matrix: Matrix4<f32> =
            Matrix4::look_at(origin, target, Vector3::new(0., -1., 0.));
        view_matrix[3] =
            Vector4::new(-origin[0], -origin[1], -origin[2], 1.0);
        let transform_matrix = proj_matrix * view_matrix;
            
        // Update the uniform
        unsafe {
            gl::UniformMatrix4fv(transform_matrix_loc, 1,
                gl::FALSE as GLboolean,
                AsRef::<[f32; 16]>::as_ref(&transform_matrix).as_ptr());
        }
    };
    
    // Update initial transform state
    update_transforms(origin, target);

    // Start a timer
    let start = Instant::now();
    'running: for frame in 1u64.. {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, (vertex_data.len() / 3) as i32);
        }

        // Swap the double buffered OpenGL
        window.gl_swap_window();

        if frame & 0xff == 0 {
            let elapsed = start.elapsed().as_secs_f64();
            print!("FPS {:10.2}\n", frame as f64 / elapsed);
        }

        // Check for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    origin[2] += 0.05;
                    update_transforms(origin, target);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    origin[2] -= 0.05;
                    update_transforms(origin, target);
                },
                _ => {}
            }
        }

        // Sleep a bit
        //std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
    }
}

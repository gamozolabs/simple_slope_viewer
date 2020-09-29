use std::io;
use std::fs::File;
use std::io::{Read, BufReader};
use std::ffi::CString;
use std::cell::Cell;
use std::path::Path;
use std::time::Instant;

use gl::types::*;

use cgmath::{Matrix4, Point3, Vector3, Deg, perspective};

use sdl2::mouse::MouseButton;
use sdl2::event::{Event, WindowEvent};
use sdl2::video::SwapInterval;
use sdl2::keyboard::Keycode;

use parse_ealogpos::Positions;

// Vertex shader
static VS_SRC: &'static str = "
#version 150
in vec3 position;
uniform mat4 transform_matrix;
out vec3 vs_pos;

out VS_OUT {
    vec3 orig_position;
} vs_out;

void main() {
    vs_pos = position;
    vs_out.orig_position = position;
    gl_Position = transform_matrix * vec4(position.x, position.y, position.z, 1.0);
}";

// Line vertex shader
static LINE_VS_SRC: &'static str = "
#version 150
in vec4 position;
uniform mat4 transform_matrix;
out vec4 vs_pos;

void main() {
    vs_pos = position;
    gl_Position = transform_matrix * vec4(position.x, position.y, position.z, 1.0);
}";

// Line fragment shader
static LINE_FS_SRC: &'static str = "
#version 150
out vec4 out_color;
in vec4 vs_pos;

void main() {
    out_color = vec4(0.0, vs_pos.w, 0.0, 1.0);
}
";

// Triangle fragment shader
static FS_SRC: &'static str = "
#version 150

in vec4 geom_color;
in vec2 geom_uv;
out vec4 out_color;

void main() {
    // thickness of the vertex, in fraction of triangle size
    float VERTEX_THICC = 0.03;

    out_color = geom_color;

    /*
    vec2 uv = geom_uv;
    float vert_dist = length(uv);
    vert_dist = min(vert_dist, length(uv - vec2(1.0, 0.0)));
    vert_dist = min(vert_dist, length(uv - vec2(0.5, sqrt(3.0*0.25))));
    out_color.y -= step(vert_dist, VERTEX_THICC);*/
}";

// Geometry shader
static GS_SRC: &'static str = "
#version 150

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in VS_OUT {
    vec3 orig_position;
} gs_in[];

out vec4 geom_color;
out vec2 geom_uv;

vec3 GetNormal()
{
   vec3 a = vec3(gs_in[0].orig_position) - vec3(gs_in[1].orig_position);
   vec3 b = vec3(gs_in[2].orig_position) - vec3(gs_in[1].orig_position);
   return normalize(cross(a, b));
}

void main() {
    vec3 normal = GetNormal();

    // Compute the slope of the triangle, in degrees.
    //   0 degrees = Flat surface, eg, flat terrain
    //  90 degrees = Straight vertical
    // 180 degrees = Flat surface, but upside-down, like looking at a ceiling
    float slope = degrees(acos(max(-1.0, min(1.0, -normal.y))));

    // Color is 0.1 0.1 0.1 for steepest slope
    // Color is 0.9 0.9 0.9 for flattest slope
    vec4 color;
    
    if(slope <= 50.) {
        // Climbable triangle
        // flat plane = 0.9, 0.9, 0.9
        // 50 degree  = 0.1, 0.1, 0.1
        color = vec4(0.1, 0.1, 0.1, 1.0) + (50 - slope) / 62.5;
    } else if(slope <= 90.) {
        // Unclimbable triangle
        //  50 degree = 0.9, 0.9, 0.9
        // 180 degree = 0.1, 0.1, 0.1
        color = vec4(0.0, 0.0, 0.3, 1.0) + (40 - (slope - 50)) / 50;
    } else {
        // Inverted triangle
        //  90 degree = 0.9, 0.9, 0.9
        // 180 degree = 0.1, 0.1, 0.1
        color = vec4(0.3, 0.0, 0.0, 1.0) + (90 - (slope - 90)) / 112.5;
    }

    //color *= 1 + (abs(normal.z) / 10.0);

    gl_Position = gl_in[0].gl_Position;
    geom_color = color;
    geom_uv = vec2(0.0, 1.0);
    EmitVertex();

    gl_Position = gl_in[1].gl_Position;
    geom_color = color;
    geom_uv = vec2(1.0, 0.0);
    EmitVertex();

    gl_Position = gl_in[2].gl_Position;
    geom_uv = vec2(0.5, sqrt(3.0 * 0.25));
    geom_color = color;
    EmitVertex();

    EndPrimitive();
}
";

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

fn link_program(vs: GLuint, fs: GLuint, gs: Option<GLuint>) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        if let Some(gs) = gs {
            gl::AttachShader(program, gs);
        }
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

/// Load a falkvbo file containing the unique verticies and a list of triangle
/// indicies
pub fn load_falkvbo<P: AsRef<Path>>(path: P)
        -> io::Result<(Vec<(f32, f32, f32)>, Vec<(u32, u32, u32)>)> {
    // Open the file
    let mut fd = BufReader::new(File::open(path)?);

    // Create the vertex and triangle buffers
    let mut verticies = Vec::new();
    let mut triangles = Vec::new();

    // Get the number of verticies
    let mut num_verticies = [0u8; 8];
    fd.read_exact(&mut num_verticies)?;
    let num_verticies = u64::from_le_bytes(num_verticies);

    // Load the vertex data
    for _ in 0..num_verticies {
        let mut x = [0u8; 4];
        fd.read_exact(&mut x)?;
        let x = f32::from_le_bytes(x);
        
        let mut y = [0u8; 4];
        fd.read_exact(&mut y)?;
        let y = f32::from_le_bytes(y);
        
        let mut z = [0u8; 4];
        fd.read_exact(&mut z)?;
        let z = f32::from_le_bytes(z);

        // Save the vertex data
        verticies.push((x, y, z));
    }
    
    // Get the number of triangles
    let mut num_triangles = [0u8; 8];
    fd.read_exact(&mut num_triangles)?;
    let num_triangles = u64::from_le_bytes(num_triangles);

    // Load the triangle data
    for _ in 0..num_triangles {
        let mut a = [0u8; 4];
        fd.read_exact(&mut a)?;
        let a = u32::from_le_bytes(a);
        
        let mut b = [0u8; 4];
        fd.read_exact(&mut b)?;
        let b = u32::from_le_bytes(b);
        
        let mut c = [0u8; 4];
        fd.read_exact(&mut c)?;
        let c = u32::from_le_bytes(c);

        // Save the triangle data
        triangles.push((a, b, c));
    }

    // Return the loaded data!
    Ok((verticies, triangles))
}

pub fn main() {
    // Get the arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        print!("Usage: {} <falkvbo file>\n", args[0]);
        return;
    }
    
    let mut head_pos: Point3<f32> = Point3::new(0., 1000., 0.);

    print!("Loading LUA data...\n");

    let mut path_data: Vec<(f32, f32, f32, f32)> = Vec::new();
            
    // Tgt triangle
    // 186.73837, 95.57352, 1843.6765
    // 176.75186, 96.93022, 1838.2499
    //let land_target_x = 1838.2499;
    //let land_target_y = 176.75186;
    let land_target_x = (1843.6765 + 1838.2499) / 2.;
    let land_target_y = (186.73837 + 176.75186) / 2.;

    let positions = Positions::from_lua("gg_uc_zep.lua")
        .expect("Failed to load LUA path");
    let mut closest = std::f64::MAX;
    let mut idx = Cell::new(1);
    positions.positions.windows(2)
        .filter(|poss| {
            idx.set(idx.get() + 1);
            let y_delta = poss[1].y - poss[0].y;
            let x_delta = poss[1].x - poss[0].x;
            let delta = ((y_delta * y_delta) + (x_delta * x_delta)).sqrt();
            poss[0].map_id == 0 && poss[0].map_id == poss[1].map_id &&
                delta < 10.
        })
        .for_each(|poss| {
            let x_delta = land_target_x - poss[0].x;
            let y_delta = land_target_y - poss[0].y;
            let delta_target =
                ((y_delta * y_delta) + (x_delta * x_delta)).sqrt();

            let delta_pct = delta_target / 121.75096014;

            let color = 1.0 - ((delta_pct - 1.0) * 500.).min(1.0);
            let color = color as f32;


            if delta_target < closest {
                closest = delta_target;
                //print!("{}\n", closest);
            }

            if delta_target == 121.75096014778174 {
                print!("WOO CLOSEST\n");

                let delta_x = (poss[0].x - land_target_x).abs();
                let delta_y = (poss[0].y - land_target_y).abs();

                let jump_angle = (delta_y / delta_x).atan().to_degrees();
                print!("X {} Y {} jump from\n", poss[0].x, poss[0].y);
                print!("{} absolute angle\n", 180. - jump_angle);
                print!("{} stationary boat angle\n",
                       (180. - (poss[0].angle - 82.691023903837)) - jump_angle);
            
                if true {
                    head_pos.x = land_target_y as f32;
                    head_pos.z = land_target_x as f32;
                } else {
                    head_pos.x = poss[0].y as f32;
                    head_pos.z = poss[0].x as f32;
                }
                
                path_data.push((poss[0].y as f32, 100.,
                                poss[0].x as f32, 1.0));
                path_data.push((land_target_y as f32, 100.,
                                land_target_x as f32, 1.0));
            }

            path_data.push((poss[0].y as f32, 100.,
                            poss[0].x as f32, color));
            path_data.push((poss[1].y as f32, 100.,
                            poss[1].x as f32, color));
        });
        
    // A decrease in our X (WoW's Y) is west
    // An increase in our Z (WoW's X) is north

    let positions = Positions::from_lua("jumpatt2_success.lua")
        .expect("Failed to load LUA path");
    positions.positions.windows(2)
        .filter(|poss| {
            idx.set(idx.get() + 1);
            let y_delta = poss[1].y - poss[0].y;
            let x_delta = poss[1].x - poss[0].x;
            let delta = ((y_delta * y_delta) + (x_delta * x_delta)).sqrt();
            poss[0].map_id == 0 && poss[0].map_id == poss[1].map_id &&
                delta < 10.
        })
        .for_each(|poss| {
            path_data.push((poss[0].y as f32, 100., poss[0].x as f32, 0.1));
            path_data.push((poss[1].y as f32, 100., poss[1].x as f32, 0.1));
        });
    let positions = Positions::from_lua("jumpatt_success2.lua")
        .expect("Failed to load LUA path");
    positions.positions.windows(2)
        .filter(|poss| {
            idx.set(idx.get() + 1);
            let y_delta = poss[1].y - poss[0].y;
            let x_delta = poss[1].x - poss[0].x;
            let delta = ((y_delta * y_delta) + (x_delta * x_delta)).sqrt();
            poss[0].map_id == 0 && poss[0].map_id == poss[1].map_id &&
                delta < 10.
        })
        .for_each(|poss| {
            path_data.push((poss[0].y as f32, 100., poss[0].x as f32, 0.5));
            path_data.push((poss[1].y as f32, 100., poss[1].x as f32, 0.5));
        });
    
    print!("Loading falkvbo data...\n");

    // Get the vertex data and indicies for the data in our object file
    let (vertex_data, triangles) =
        load_falkvbo(&args[1]).expect("Failed to load falkvbo data");

    print!("Falkvbo data loaded!\n");

    // Create an SDL context
    let sdl_context = sdl2::init().unwrap();

    // Get access to the video subsystem
    let video_subsystem = sdl_context.video().unwrap();

    // Width and height of the window
    let win_width  = Cell::new(1440);
    let win_height = Cell::new(900);

    // Create a window
    let window = video_subsystem
        .window("rust-sdl2 demo", win_width.get(), win_height.get())
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    // Create the GL context
    let _gl = window.gl_create_context().unwrap();

    // Turn off vsync
    video_subsystem.gl_set_swap_interval(SwapInterval::Immediate).unwrap();

    // Set relative mouse mode
    sdl_context.mouse().set_relative_mouse_mode(true);

    // Load the GL procedure addresses
    gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    
    // Get the event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let gs = compile_shader(GS_SRC, gl::GEOMETRY_SHADER);
    let program = link_program(vs, fs, Some(gs));
    
    // Create GLSL shaders
    let line_vs = compile_shader(LINE_VS_SRC, gl::VERTEX_SHADER);
    let line_fs = compile_shader(LINE_FS_SRC, gl::FRAGMENT_SHADER);
    let line_program = link_program(line_vs, line_fs, None);

    let mut vao = 0;
    let mut path_vao = 0;
    let mut vbo = 0;
    let mut ele_buffer = 0;
    let mut path_buffer = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            core::mem::size_of_val(&vertex_data[..]) as isize,
            vertex_data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0,
                                 CString::new("out_color").unwrap().as_ptr());
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

        gl::GenBuffers(1, &mut ele_buffer);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ele_buffer);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
            core::mem::size_of_val(&triangles[..]) as isize,
            triangles.as_ptr() as *const _, gl::STATIC_DRAW);
        
        // Set up the path VAO
        gl::GenVertexArrays(1, &mut path_vao);
        gl::BindVertexArray(path_vao);
       
        // Create a buffer for vertex data for the path
        gl::GenBuffers(1, &mut path_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, path_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            core::mem::size_of_val(&path_data[..]) as isize,
            path_data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::UseProgram(line_program);
        gl::BindFragDataLocation(line_program, 0,
                                 CString::new("out_color").unwrap().as_ptr());
        let pos_attr = gl::GetAttribLocation(line_program,
                                             CString::new("position")
                                             .unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            4,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            std::ptr::null(),
        );

        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::FrontFace(gl::CCW);
        gl::CullFace(gl::BACK);
    }

    // Name of the camera uniform
    let transform_matrix_name = CString::new("transform_matrix").unwrap();

    let mut head_horiz_angle: f32 = 0.;
    let mut head_vert_angle:  f32 = (-89.9999999f32).to_radians();
    let mut move_speed:       f32 = 10.;

    // Find the transform matrix location
    let transform_matrix_loc = unsafe {
        gl::GetUniformLocation(program, transform_matrix_name.as_ptr())
    };
    let transform_matrix_loc_line = unsafe {
        gl::GetUniformLocation(line_program, transform_matrix_name.as_ptr())
    };

    let update_transforms = |origin: &mut Point3<f32>, horiz_angle: f32, vert_angle: f32, movement_front: f32, movement_strafe: f32| {
        let direction = Vector3::new(
            vert_angle.cos() * horiz_angle.sin(),
            vert_angle.sin(),
            vert_angle.cos() * horiz_angle.cos());
        
        let horiz_angle = horiz_angle + std::f32::consts::PI / 2.;
        let vert_angle  = 0f32;
        let direction_strafe = Vector3::new(
            vert_angle.cos() * horiz_angle.sin(),
            vert_angle.sin(),
            vert_angle.cos() * horiz_angle.cos());

        *origin += direction * movement_front;
        *origin += direction_strafe * movement_strafe;
        
        let proj_matrix: Matrix4<f32> =
            perspective(Deg(45.),
                win_width.get() as f32 / win_height.get() as f32,
                0.01, 2000000.0);
        let view_matrix: Matrix4<f32> =
            Matrix4::look_at(*origin, *origin + direction, Vector3::new(0., 1., 0.));
        let transform_matrix = proj_matrix * view_matrix;
        
        print!("Character at {:?}\n", origin);

        // Update the uniform
        unsafe {
            gl::UseProgram(program);
            gl::UniformMatrix4fv(transform_matrix_loc, 1,
                gl::FALSE as GLboolean,
                AsRef::<[f32; 16]>::as_ref(&transform_matrix).as_ptr());
            gl::UseProgram(line_program);
            gl::UniformMatrix4fv(transform_matrix_loc_line, 1,
                gl::FALSE as GLboolean,
                AsRef::<[f32; 16]>::as_ref(&transform_matrix).as_ptr());
        }
    };
    
    // Update initial transform state
    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, 0., 0.);

    // Enables movement of the camera angle by the mouse
    let mut mouse_enabled = true;

    // If set, render the next frame
    let mut frame_changed = true;

    // Tracks if the window has focus
    let mut focused = true;

    // Start a timer
    let start = Instant::now();
    let mut last_status = start;
    let mut frames = 0;
    'running: loop {
        if focused && frame_changed {
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      
                gl::UseProgram(program);
                gl::BindVertexArray(vao);
                gl::DrawElements(gl::TRIANGLES, triangles.len() as i32 * 3,
                    gl::UNSIGNED_INT, core::ptr::null_mut());
                
                gl::UseProgram(line_program);
                gl::BindVertexArray(path_vao);
                gl::DrawArrays(gl::LINES, 0, path_data.len() as i32);
            }

            // Set that the frame has not changed
            frame_changed = false;
        }

        // Swap the double buffered OpenGL
        window.gl_swap_window();
        
        // Update frames rendered
        frames += 1;

        if last_status.elapsed().as_secs_f64() >= 1.0 {
            let elapsed = last_status.elapsed().as_secs_f64();
            print!("FPS {:10.2} | triangles {:10} | verticies {:10}\n",
                   frames as f64 / elapsed,
                   triangles.len(),
                   vertex_data.len());

            // Reset frame counter
            frames = 0;
            last_status = Instant::now();
        }

        // Check for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                },
                Event::Window { win_event: WindowEvent::FocusGained, .. } => {
                    focused = true;
                }
                Event::Window { win_event: WindowEvent::FocusLost, .. } => {
                    focused = false;
                }
                Event::Window { win_event: WindowEvent::Resized(x, y), .. } => {
                    win_width.set(x as u32);
                    win_height.set(y as u32);

                    // Update viewport
                    unsafe {
                        gl::Viewport(0, 0, win_width.get() as i32,
                            win_height.get() as i32);
                    }
                    
                    // Update transforms
                    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, 0., 0.);
                    frame_changed = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    sdl_context.mouse().set_relative_mouse_mode(false);
                    mouse_enabled = false;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, move_speed, 0.);
                    frame_changed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, -move_speed, 0.);
                    frame_changed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, 0., move_speed);
                    frame_changed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    update_transforms(&mut head_pos, head_horiz_angle, head_vert_angle, 0., -move_speed);
                    frame_changed = true;
                },
                Event::MouseWheel { y, .. } => {
                    if y > 0 {
                        move_speed *= 1.2;
                    } else {
                        move_speed /= 1.2;
                    }
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    sdl_context.mouse().set_relative_mouse_mode(true);
                    mouse_enabled = true;
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    if mouse_enabled {
                        let xdel = xrel as f32 / 400.;
                        let ydel = yrel as f32 / 400.;
                        head_horiz_angle = head_horiz_angle - xdel;
                        head_vert_angle  = (head_vert_angle  - ydel)
                            .min(std::f32::consts::PI / 2. - 0.0001)
                            .max(-std::f32::consts::PI / 2. + 0.0001);
                        update_transforms(&mut head_pos,
                            head_horiz_angle, head_vert_angle, 0., 0.);
                        frame_changed = true;
                    }
                }
                _ => {}
            }
        }

        // Cap at 300 fps
        std::thread::sleep(
            std::time::Duration::from_nanos(1_000_000_000 / 300));
    }
}

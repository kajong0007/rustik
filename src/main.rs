extern crate kiss3d;

use kiss3d::nalgebra::{Vector3, UnitQuaternion, Translation3};
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use kiss3d::light::Light;

/*
 * Add one of our cube faces to the parent gray cube
 */
fn add_sub_cube(parent : &mut SceneNode,
            off_x : f32, off_y : f32, off_z : f32,
            col_r : f32, col_g : f32, col_b : f32) {

    let cube_size = 0.29;

    let mut new_cube = parent.add_cube(cube_size, cube_size, cube_size);
    new_cube.set_color(col_r, col_g, col_b);
    new_cube.append_translation(&Translation3::new(off_x, off_y, off_z));
}

/*
 * Add a whole face of cubes by iterating over the vector of vectors
 */
fn add_cube_face(c: &mut SceneNode, v: &Vec<Vec<f32>>, count: &mut usize, color_v: &Vec<Vec<f32>>) {
    for i in v.iter() {
        let color = &color_v[*count];
        add_sub_cube(c, i[0], i[1], i[2], color[0], color[1], color[2]);
        *count += 1;
    }
}

/*
 * Swap two columns of a vector of vectors
 */
fn swap_v(v: &mut Vec<Vec<f32>>, col0: usize, col1: usize) {
    for i in v.iter_mut() {
        let tmp = i[col0];
        i[col0] = i[col1];
        i[col1] = tmp;
    }
}

/*
 * Invert the sign of a column in a vector
 */
fn sign_flip_v(v: &mut Vec<Vec<f32>>, col0: usize) {
    for i in v.iter_mut() {
        i[col0] = -i[col0];
    }
}

fn main() {
    let mut window = Window::new("Kiss3d: cube");

    // Make a giant gray cube
    let mut c      = window.add_cube(3.0, 3.0, 3.0);
    c.set_color(0.3, 0.3, 0.3);

    // These distances are used to translate the cubes for the faces
    let emerge_distance = 1.15;
    let offset_distance = 1.0;

    // This is every color of every face on the cubes
    let color_v : Vec<Vec<f32>> = vec![
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0],

        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0],

        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0],

        vec![0.9, 0.9, 0.9],
        vec![0.9, 0.9, 0.9],
        vec![0.9, 0.9, 0.9],
        vec![0.9, 0.9, 0.9],
        vec![0.9, 0.9, 0.9],
        vec![0.9, 0.9, 0.9],
        vec![0.9, 0.9, 0.9],
        vec![0.9, 0.9, 0.9],
        vec![0.9, 0.9, 0.9],

        vec![1.0, 0.5, 0.0],
        vec![1.0, 0.5, 0.0],
        vec![1.0, 0.5, 0.0],
        vec![1.0, 0.5, 0.0],
        vec![1.0, 0.5, 0.0],
        vec![1.0, 0.5, 0.0],
        vec![1.0, 0.5, 0.0],
        vec![1.0, 0.5, 0.0],
        vec![1.0, 0.5, 0.0],

        vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0],
    ];

    // A single face on the cube, coordinate-wise
    let mut v : Vec<Vec<f32>> = vec![
        vec![emerge_distance, offset_distance, offset_distance],
        vec![emerge_distance, offset_distance, 0.0],
        vec![emerge_distance, offset_distance, -offset_distance],
        vec![emerge_distance, 0.0, offset_distance],
        vec![emerge_distance, 0.0, 0.0],
        vec![emerge_distance, 0.0, -offset_distance],
        vec![emerge_distance, -offset_distance, offset_distance],
        vec![emerge_distance, -offset_distance, 0.0],
        vec![emerge_distance, -offset_distance, -offset_distance],
    ];

    // Render all of the faces of the rubik's cube
    let mut count = 0;
    add_cube_face(&mut c, &v, &mut count, &color_v);
    sign_flip_v(&mut v, 0);
    add_cube_face(&mut c, &v, &mut count, &color_v);
    swap_v(&mut v, 0, 1);
    add_cube_face(&mut c, &v, &mut count, &color_v);
    sign_flip_v(&mut v, 1);
    add_cube_face(&mut c, &v, &mut count, &color_v);
    swap_v(&mut v, 1, 2);
    add_cube_face(&mut c, &v, &mut count, &color_v);
    sign_flip_v(&mut v, 2);
    add_cube_face(&mut c, &v, &mut count, &color_v);

    window.set_light(Light::StickToCamera);

    // Slowly rotate the rubik's cube
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.00014);

    while window.render() {
        c.append_rotation(&rot);
    }
}

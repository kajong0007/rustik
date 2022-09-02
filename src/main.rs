extern crate kiss3d;

use kiss3d::nalgebra::{Vector3, UnitQuaternion, Translation3};
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use kiss3d::light::Light;

/*
 * Add one of our cube faces to the parent gray cube
 */
fn add_sub_cube(parent : &mut SceneNode,
            cube_size : f32,
            off_x : f32, off_y : f32, off_z : f32,
            col_r : f32, col_g : f32, col_b : f32) {

    let mut new_cube = parent.add_cube(cube_size, cube_size, cube_size);
    new_cube.set_color(col_r, col_g, col_b);
    new_cube.append_translation(&Translation3::new(off_x, off_y, off_z));
}

/*
 * Add a whole face of cubes by iterating over the vector of vectors
 */
fn add_cube_face(c: &mut SceneNode, v: &Vec<Vec<f32>>, count: &mut usize, color_v: &Vec<Vec<f32>>, cube_size : f32) {
    for i in v.iter() {
        let color = &color_v[*count];
        add_sub_cube(c, cube_size, i[0], i[1], i[2], color[0], color[1], color[2]);
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

fn clobber_one_vec(color_v: &mut Vec<Vec<f32>>, dst_idx: usize, src_idx: usize) {
    color_v[dst_idx][0] = color_v[src_idx][0];
    color_v[dst_idx][1] = color_v[src_idx][1];
    color_v[dst_idx][2] = color_v[src_idx][2];
}

fn clobber_from_other_vec(color_v: &mut Vec<Vec<f32>>, dst_idx: usize, src_vec: &Vec<f32>) {
    color_v[dst_idx][0] = src_vec[0];
    color_v[dst_idx][1] = src_vec[1];
    color_v[dst_idx][2] = src_vec[2];
}

const LEFT  : usize = 9 * 0;
const RIGHT : usize = 9 * 1;
const DOWN  : usize = 9 * 2;
const UP    : usize = 9 * 3;
const BACK  : usize = 9 * 4;
const FRONT : usize = 9 * 5;

fn rotate_four(color_v: &mut Vec<Vec<f32>>, one: usize, two: usize, three: usize, four: usize) {
    let tmp = vec![
        color_v[one][0],
        color_v[one][1],
        color_v[one][2],
    ];
    clobber_one_vec(color_v, one, two);
    clobber_one_vec(color_v, two, three);
    clobber_one_vec(color_v, three, four);
    clobber_from_other_vec(color_v, four, &tmp);
}

fn apply_3_times(color_v: &mut Vec<Vec<f32>>, func: fn(&mut Vec<Vec<f32>>)) {
    func(color_v);
    func(color_v);
    func(color_v);
}

fn front_clockwise(color_v : &mut Vec<Vec<f32>>) {
    rotate_four(color_v, 20, 17, 35, 2);
    rotate_four(color_v, 23, 14, 32, 5);
    rotate_four(color_v, 26, 11, 29, 8);
    rotate_four(color_v, 47, 53, 51, 45);
    rotate_four(color_v, 46, 50, 52, 48);
}

fn left_clockwise(color_v: &mut Vec<Vec<f32>>) {
    rotate_four(color_v, 0, 6, 8, 2);
    rotate_four(color_v, 1, 3, 7, 5);
    rotate_four(color_v, 38, 20, 45, 27);
    rotate_four(color_v, 37, 19, 46, 28);
    rotate_four(color_v, 36, 18, 47, 29);
}

fn right_clockwise(color_v: &mut Vec<Vec<f32>>) {
    rotate_four(color_v, 11, 17, 15, 9);
    rotate_four(color_v, 10, 14, 16, 12);
    rotate_four(color_v, 51, 26, 44, 33);
    rotate_four(color_v, 52, 25, 43, 34);
    rotate_four(color_v, 53, 24, 42, 35);
}

fn up_clockwise(color_v: &mut Vec<Vec<f32>>) {
    rotate_four(color_v, 35, 33, 27, 29);
    rotate_four(color_v, 32, 34, 30, 28);
    rotate_four(color_v, 36, 2, 51, 9);
    rotate_four(color_v, 39, 1, 48, 10);
    rotate_four(color_v, 42, 0, 45, 11);
}

fn down_clockwise(color_v: &mut Vec<Vec<f32>>) {
    rotate_four(color_v, 18, 24, 26, 20);
    rotate_four(color_v, 19, 21, 25, 23);
    rotate_four(color_v, 44, 17, 47, 6);
    rotate_four(color_v, 41, 16, 50, 7);
    rotate_four(color_v, 38, 15, 53, 8);
}

fn back_clockwise(color_v: &mut Vec<Vec<f32>>) {
    rotate_four(color_v, 44, 38, 36, 42);
    rotate_four(color_v, 43, 41, 37, 39);
    rotate_four(color_v, 24, 6, 27, 9);
    rotate_four(color_v, 21, 3, 30, 12);
    rotate_four(color_v, 18, 0, 33, 15);
}

fn front_counter_clockwise(color_v: &mut Vec<Vec<f32>>) {
    apply_3_times(color_v, front_clockwise);
}

fn left_counter_clockwise(color_v: &mut Vec<Vec<f32>>) {
    apply_3_times(color_v, left_clockwise);
}

fn right_counter_clockwise(color_v: &mut Vec<Vec<f32>>) {
    apply_3_times(color_v, right_clockwise);
}
fn up_counter_clockwise(color_v: &mut Vec<Vec<f32>>) {
    apply_3_times(color_v, up_clockwise);
}
fn down_counter_clockwise(color_v: &mut Vec<Vec<f32>>) {
    apply_3_times(color_v, down_clockwise);
}
fn back_counter_clockwise(color_v: &mut Vec<Vec<f32>>) {
    apply_3_times(color_v, back_clockwise);
}

/* Cube orientation
 *
 *                 ____________           
 *              Up | 27 30 33 |           
 *                 | 28 31 34 |           
 *                 | 29 32 35 |      up     
 *      ___________|__________|___________
 * Left | 00 01 02 | 45 48 51 | 11 10  9 | Right
 *      | 03 04 05 | 46 49 52 | 14 13 12 |
 *      | 06 07 08 | 47 50 53 | 17 16 15 |
 *      |          |  Front   |          | back
 *      |__________|__________|__________|
 *                 | 20 23 26 |           
 *            Down | 19 22 25 |           
 *                 | 18 21 24 |           
 *                 |__________|           
 *                 | 38 41 44 |           
 *            Back | 37 40 43 |           
 *                 | 36 39 42 |           
 *                 ------------           
 *
 */


fn main() {
    let mut window = Window::new("Kiss3d: cube");

    let big_cube_size = 3.0;

    // Make a giant gray cube
    let mut c      = window.add_cube(big_cube_size, big_cube_size, big_cube_size);
    c.set_color(0.3, 0.3, 0.3);

    // These distances are used to translate the cubes for the faces
    let emerge_distance = (big_cube_size / 3.0) * 1.15;
    let offset_distance = big_cube_size / 3.0;
    let cube_size = 0.1 * big_cube_size;

    // colors that decrease in intensity to help find out
    // the indices on each face
    let mut color_v : Vec<Vec<f32>> = vec![
        vec![1.0, 0.5, 0.0], vec![0.9, 0.5, 0.0], vec![0.8, 0.5, 0.0], vec![0.7, 0.5, 0.0],
        vec![0.6, 0.5, 0.0], vec![0.5, 0.5, 0.0], vec![0.4, 0.5, 0.0], vec![0.3, 0.5, 0.0], vec![0.2, 0.5, 0.0],

        vec![1.0, 0.0, 0.0], vec![0.9, 0.0, 0.0], vec![0.8, 0.0, 0.0], vec![0.7, 0.0, 0.0],
        vec![0.6, 0.0, 0.0], vec![0.5, 0.0, 0.0], vec![0.4, 0.0, 0.0], vec![0.3, 0.0, 0.0], vec![0.2, 0.0, 0.0],

        vec![1.0, 1.0, 0.0], vec![0.9, 0.9, 0.0], vec![0.8, 0.8, 0.0], vec![0.7, 0.7, 0.0],
        vec![0.6, 0.6, 0.0], vec![0.5, 0.5, 0.0], vec![0.4, 0.4, 0.0], vec![0.3, 0.3, 0.0], vec![0.2, 0.2, 0.0],

        vec![0.9, 0.9, 0.9], vec![0.8, 0.8, 0.8], vec![0.7, 0.7, 0.7], vec![0.6, 0.6, 0.6],
        vec![0.5, 0.5, 0.5], vec![0.4, 0.4, 0.4], vec![0.3, 0.3, 0.3], vec![0.2, 0.2, 0.2], vec![0.1, 0.1, 0.1],

        vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 0.9], vec![0.0, 0.0, 0.8], vec![0.0, 0.0, 0.7],
        vec![0.0, 0.0, 0.6], vec![0.0, 0.0, 0.5], vec![0.0, 0.0, 0.4], vec![0.0, 0.0, 0.3], vec![0.0, 0.0, 0.2],

        vec![0.0, 1.0, 0.0], vec![0.0, 0.9, 0.0], vec![0.0, 0.8, 0.0], vec![0.0, 0.7, 0.0],
        vec![0.0, 0.6, 0.0], vec![0.0, 0.5, 0.0], vec![0.0, 0.4, 0.0], vec![0.0, 0.3, 0.0], vec![0.0, 0.2, 0.0],
    ];

    // Real Colors
    /*
    let mut color_v = vec![
        vec![1.0, 0.5, 0.0], vec![1.0, 0.5, 0.0], vec![1.0, 0.5, 0.0], vec![1.0, 0.5, 0.0],
        vec![1.0, 0.5, 0.0], vec![1.0, 0.5, 0.0], vec![1.0, 0.5, 0.0], vec![1.0, 0.5, 0.0], vec![1.0, 0.5, 0.0],

        vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0], vec![1.0, 0.0, 0.0],

        vec![1.0, 1.0, 0.0], vec![1.0, 1.0, 0.0], vec![1.0, 1.0, 0.0], vec![1.0, 1.0, 0.0],
        vec![1.0, 1.0, 0.0], vec![1.0, 1.0, 0.0], vec![1.0, 1.0, 0.0], vec![1.0, 1.0, 0.0], vec![1.0, 1.0, 0.0],

        vec![0.9, 0.9, 0.9], vec![0.9, 0.9, 0.9], vec![0.9, 0.9, 0.9], vec![0.9, 0.9, 0.9],
        vec![0.9, 0.9, 0.9], vec![0.9, 0.9, 0.9], vec![0.9, 0.9, 0.9], vec![0.9, 0.9, 0.9], vec![0.9, 0.9, 0.9],

        vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 1.0],

        vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 1.0, 0.0],
    ];
    // */

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

    back_clockwise(&mut color_v);

    // Render all of the faces of the rubik's cube
    let mut count = 0;
    add_cube_face(&mut c, &v, &mut count, &color_v, cube_size);
    sign_flip_v(&mut v, 0);
    add_cube_face(&mut c, &v, &mut count, &color_v, cube_size);
    swap_v(&mut v, 0, 1);
    add_cube_face(&mut c, &v, &mut count, &color_v, cube_size);
    sign_flip_v(&mut v, 1);
    add_cube_face(&mut c, &v, &mut count, &color_v, cube_size);
    swap_v(&mut v, 1, 2);
    add_cube_face(&mut c, &v, &mut count, &color_v, cube_size);
    sign_flip_v(&mut v, 2);
    add_cube_face(&mut c, &v, &mut count, &color_v, cube_size);

    window.set_light(Light::StickToCamera);

    // Slowly rotate the rubik's cube
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.00014);

    while window.render() {
        c.append_rotation(&rot);
    }
}

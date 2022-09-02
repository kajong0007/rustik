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

fn clobber_one_vec_backwards(color_v: &mut Vec<Vec<f32>>, dst_idx: usize, src_idx: usize) {
    color_v[dst_idx][0] = color_v[src_idx][2];
    color_v[dst_idx][1] = color_v[src_idx][1];
    color_v[dst_idx][2] = color_v[src_idx][0];
}

fn clobber_from_other_vec(color_v: &mut Vec<Vec<f32>>, dst_idx: usize, src_vec: &Vec<f32>) {
    color_v[dst_idx][0] = src_vec[0];
    color_v[dst_idx][1] = src_vec[1];
    color_v[dst_idx][2] = src_vec[2];
}

fn clobber_three_vecs(color_v: &mut Vec<Vec<f32>>,
               side0: usize, side1: usize,
               coord0: usize, coord1: usize, coord2: usize) {

    clobber_one_vec(color_v, side0 + coord0, side1 + coord0);
    clobber_one_vec(color_v, side0 + coord1, side1 + coord1);
    clobber_one_vec(color_v, side0 + coord2, side1 + coord2);
}

fn clobber_three_vecs_backwards(color_v: &mut Vec<Vec<f32>>,
               side0: usize, side1: usize,
               coord0: usize, coord1: usize, coord2: usize) {
    clobber_one_vec_backwards(color_v, side0 + coord0, side1 + coord0);
    clobber_one_vec_backwards(color_v, side0 + coord1, side1 + coord1);
    clobber_one_vec_backwards(color_v, side0 + coord2, side1 + coord2);
}

fn rotate_something_clockwise(color_v: &mut Vec<Vec<f32>>,
                              coord0: usize, coord1: usize, coord2: usize,
                              face: usize,
                              side0: usize, side1: usize, side2: usize, side3: usize,
                              corner0: usize, corner1: usize, corner2: usize, corner3: usize,
                              cross0: usize, cross1: usize, cross2: usize, cross3: usize,
                              ) {
    let tmp0 = vec![
        color_v[side0 + coord0][2],
        color_v[side0 + coord0][1],
        color_v[side0 + coord0][0],
    ];
    let tmp1 = vec![
        color_v[side0 + coord1][0],
        color_v[side0 + coord1][1],
        color_v[side0 + coord1][2],
    ];
    let tmp2 = vec![
        color_v[side0 + coord2][0],
        color_v[side0 + coord2][1],
        color_v[side0 + coord2][2],
    ];
    clobber_three_vecs(color_v, side0, side1, coord0, coord1, coord2);
    clobber_three_vecs(color_v, side1, side2, coord0, coord1, coord2);
    clobber_three_vecs(color_v, side2, side3, coord0, coord1, coord2);

    clobber_from_other_vec(color_v, side3 + coord0, &tmp0);
    clobber_from_other_vec(color_v, side3 + coord1, &tmp1);
    clobber_from_other_vec(color_v, side3 + coord2, &tmp2);

    let tmp0 = vec![
        color_v[face + corner0][0],
        color_v[face + corner0][1],
        color_v[face + corner0][2],
    ];
    let tmp1 = vec![
        color_v[face + corner1][0],
        color_v[face + corner1][1],
        color_v[face + corner1][2],
    ];

    clobber_one_vec(color_v, face + corner0, face + corner1);
    clobber_one_vec(color_v, face + corner1, face + corner2);
    clobber_one_vec(color_v, face + corner2, face + corner3);
    clobber_from_other_vec(color_v, face + corner3, &tmp0);

    clobber_one_vec(color_v, face + cross0, face + cross1);
    clobber_one_vec(color_v, face + cross1, face + cross2);
    clobber_one_vec(color_v, face + cross2, face + cross3);
    clobber_from_other_vec(color_v, face + cross3, &tmp0);
}

const LEFT  : usize = 9 * 0;
const RIGHT : usize = 9 * 1;
const DOWN  : usize = 9 * 2;
const UP    : usize = 9 * 3;
const BACK  : usize = 9 * 4;
const FRONT : usize = 9 * 5;

fn apply_3_times(color_v: &mut Vec<Vec<f32>>, func: fn(&mut Vec<Vec<f32>>)) {
    func(color_v);
    func(color_v);
    func(color_v);
}

fn front_clockwise(color_v : &mut Vec<Vec<f32>>) {
    // rotating the front is positions 2,5,8
    // of the left, up, right, and down sides
    rotate_something_clockwise(color_v, 2, 5, 8,
                               FRONT,
                               LEFT, DOWN, RIGHT, UP,
                               0, 2, 8, 6,
                               1, 5, 7, 3);
}

fn front_counter_clockwise(color_v: &mut Vec<Vec<f32>>) {
    apply_3_times(color_v, front_clockwise);
}

fn left_clockwise(color_v: &mut Vec<Vec<f32>>) {
    rotate_something_clockwise(color_v, 0, 1, 2,
                               LEFT,
                               BACK, DOWN, FRONT, UP,
                               6, 8, 2, 0,
                               3, 7, 5, 1);
}

fn left_counter_clockwise(color_v: &mut Vec<Vec<f32>>) {
    apply_3_times(color_v, left_clockwise);
}


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

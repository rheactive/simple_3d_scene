use macroquad::prelude::*;
use std::vec::Vec;
//use std::slice;

const PI: f32 = 3.1415926536;
const SQRT2: f32 = 1.4142135623731;
const SPEED: f32 = 1.0;
const PHI0: f32 = 0.25 * PI;
const NV: usize = 9;

// get parameters related to window size
fn get_window_par (height: f32, width: f32) -> [f32; 5] {
    let d = width / 100.0;
    let r = width * 2.0 / PI;
    let theta0 = PHI0 * height / width;
    [d, r, theta0, width, height]
}

// defines the vertices and edges of a tetrahedron
fn create_tetrahedron (x: f32, y: f32, z: f32, a: f32) -> Vec<(f32, f32, f32)> {
    let a1 = 0.5 * a;
    let da = a / (NV as f32);
    let b = a1 / SQRT2;
    let db = da / SQRT2;
    let xn = [x + a1, x - a1, x, x];
    let yn = [y, y, y + a1, y - a1];
    let zn = [z - b, z - b, z + b, z + b];
    let p1 = (xn[0], yn[0], zn[0]);
    let p2 = (xn[1], yn[1], zn[1]);
    let p3 = (xn[2], yn[2], zn[2]);
    let p4 = (xn[3], yn[3], zn[3]);
    let mut coords = vec![p1, p2, p3, p4];
    // edge connecting first and second vertices
    for n in 0..NV {
        let dx = da * (n as f32);
        coords.push((xn[0] - dx, yn[0], zn[0]))
    }
    // edge connecting third and fourth vertices
    for n in 0..NV {
        let dy = da * (n as f32);
        coords.push((xn[2], yn[2] - dy, zn[2]))
    }
    // edge connecting first and third vertices
    for n in 0..NV {
        let dx = 0.5 * da * (n as f32);
        let dz = db * (n as f32);
        coords.push((xn[0] - dx, yn[0] + dx, zn[0] + dz))
    }
    // edge connecting second and fourth vertices
    for n in 0..NV {
        let dx = 0.5 * da * (n as f32);
        let dz = db * (n as f32);
        coords.push((xn[1] + dx, yn[1] - dx, zn[1] + dz))
    }
    // edge connecting first and fourth vertices
    for n in 0..NV {
        let dx = 0.5 * da * (n as f32);
        let dz = db * (n as f32);
        coords.push((xn[0] - dx, yn[0] - dx, zn[0] + dz))
    }
    // edge connecting second and third vertices
    for n in 0..NV {
        let dx = 0.5 * da * (n as f32);
        let dz = db * (n as f32);
        coords.push((xn[1] + dx, yn[1] + dx, zn[1] + dz))
    }
    coords
}

// calculates the projected screen coordinates and size for any point
fn project_coord (par: [f32; 5], delta: f32, gamma: f32, x: f32, y: f32, z: f32) -> (usize, f32, f32, f32) {
    let d = par[0];
    let r = par[1];
    let theta0 = par[2];
    let height = par[4];
    let xpp = x * delta.cos() - y * delta.sin();
    let ypp = x * delta.sin() + y * delta.cos();
    let zpp = z;
    let xp = xpp;
    let yp = r + zpp * gamma.sin() + ypp * gamma.cos();
    let zp = zpp * gamma.cos() - ypp * gamma.sin();
    let rp = xp * xp + yp * yp + zp * zp;
    let u = r * (0.5 * PI + PHI0 - (xp / (xp * xp + yp * yp).sqrt()).acos());
    let v = height - r * (0.5 * PI + theta0 - (zp / (zp * zp + yp * yp).sqrt()).acos());
    let dp = 2.0 * r * (0.5 * d / rp.sqrt()).asin();
    (rp as usize, u, v, dp)
}

// draws points
fn draw_points (par: [f32; 5], delta: f32, gamma: f32, pn: Vec<(f32, f32, f32)>) {
    let width = par[3];
    let height = par[4];
    let mut points_data: Vec<(usize, f32, f32, f32)> = Vec::new();
    for p in pn {
        let x = p.0;
        let y = p.1;
        let z = p.2;
        points_data.push(project_coord(par, delta, gamma, x, y, z));
    }
    points_data.sort_by_key(|k| k.0);
    points_data.reverse();
    let rmax = points_data[0].0;
    // drawing
    for p in points_data {
        let rp = 255 - ((p.0 as f32) / (rmax as f32) * 200 as f32) as u8;
        let u = p.1;
        let v = p.2;
        let d = p.3;
        if u > 0.0 && u < width && v > 0.0 && v < height {
        draw_circle(u, v, d, Color::from_rgba(rp, rp, rp, 255))
        }
    }

}

#[macroquad::main("Simple 3D scene")]
async fn main() {

    clear_background(LIGHTGRAY);

    let mut delta = 0.0f32;
    let mut gamma = 0.0f32;
    let mut ft: f32;
    let mut par: [f32; 5];
    let mut d: f32;
    let mut tetrahedron: Vec<(f32, f32, f32)>;

    loop {
        clear_background(LIGHTGRAY);

        
        ft = get_frame_time() as f32;

        par = get_window_par(screen_height(), screen_width());
        d = par[0];

        draw_text(format!("Use arrow keys to rotate the camera").as_str(), 5.0 * d, 3.0 * d, 2.5 * d, BLACK);

        if is_key_down(KeyCode::Left) {
            delta = delta - SPEED * ft;
        }

        if is_key_down(KeyCode::Right) {
            delta = delta + SPEED * ft;
        }

        if is_key_down(KeyCode::Up) {
            gamma = gamma + SPEED * ft;
        }

        if is_key_down(KeyCode::Down) {
            gamma = gamma - SPEED * ft;
        }

        tetrahedron = create_tetrahedron(0.0, 0.0, 0.0, 30.0 * d);
        
        draw_points(par, delta, gamma, tetrahedron);

        

        next_frame().await;
    }
    
}

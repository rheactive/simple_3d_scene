use macroquad::prelude::*;
use std::vec::Vec;

const PI: f32 = std::f32::consts::PI;
const SQRT2: f32 = std::f32::consts::SQRT_2;
const SPEED: f32 = 1.0;
const SPEEDX: f32 = 500.0;
const PHI0: f32 = 0.25 * PI;
const TOL: f32 = 1e-12;

// get parameters related to window size
fn get_window_par (height: f32, width: f32) -> [f32; 8] {
    let d = width / 100.0;
    let r = width * 2.0 / PI;
    let theta0 = PHI0 * height / width;
    let xl = r / 2.0;
    let yl = r / 2.0;
    let zl = r / 2.5;
    [d, r, theta0, width, height, xl, yl, zl]
}

// defines the vertices of a tetrahedron
fn create_tetrahedron (x: f32, y: f32, z: f32, a: f32) -> Vec<([f32; 3], [f32; 3], [f32; 3])> {
    let a1 = 0.5 * a;
    let b = a1 / SQRT2;
    let xn = [x + a1, x - a1, x, x];
    let yn = [y, y, y + a1, y - a1];
    let zn = [z - b, z - b, z + b, z + b];
    let p1 = [xn[0], yn[0], zn[0]];
    let p2 = [xn[1], yn[1], zn[1]];
    let p3 = [xn[2], yn[2], zn[2]];
    let p4 = [xn[3], yn[3], zn[3]];
    let tr1 = (p1, p2, p3);
    let tr2 = (p1, p2, p4);
    let tr3 = (p1, p3, p4);
    let tr4 = (p2, p3, p4);
    let coords = vec![tr1, tr2, tr3, tr4];
    coords
}

// calculate middle point of 3 points
fn middle_point(pn: ([f32; 3], [f32; 3], [f32; 3])) -> [f32; 3] {
    let p1 = pn.0;
    let p2 = pn.1;
    let p3 = pn.2;
    let x = (p1[0] + p2[0] + p3[0]) / 3.0;
    let y = (p1[1] + p2[1] + p3[1]) / 3.0;
    let z = (p1[2] + p2[2] + p3[2]) / 3.0;
    [x, y, z]
}

// calculates the projected screen coordinates for any point
fn project_coord (par: [f32; 8], delta: f32, gamma: f32, p: [f32; 3]) -> [f32; 2] {
    let r = par[1];
    let theta0 = par[2];
    let height = par[4];
    let x = p[0];
    let y = p[1];
    let z = p[2];
    let xpp = x * delta.cos() - y * delta.sin();
    let ypp = x * delta.sin() + y * delta.cos();
    let zpp = z;
    let xp = xpp;
    let yp = r + zpp * gamma.sin() + ypp * gamma.cos();
    let zp = zpp * gamma.cos() - ypp * gamma.sin();
    let rpx = xp * xp + yp * yp;
    let rpz = zp * zp + yp * yp;
    let mut u = -r;
    let mut v = -r;
    if rpx > TOL || rpz > TOL {
        u = r * (0.5 * PI + PHI0 - (xp / rpx.sqrt()).acos());
        v = height - r * (0.5 * PI + theta0 - (zp / rpz.sqrt()).acos());
    }
    [u, v]
}

fn dist_to_camera (par: [f32; 8], delta: f32, gamma: f32, p: [f32; 3]) -> usize {
    let r = par[1];
    let x = p[0];
    let y = p[1];
    let z = p[2];
    let xpp = x * delta.cos() - y * delta.sin();
    let ypp = x * delta.sin() + y * delta.cos();
    let zpp = z;
    let xp = xpp;
    let yp = r + zpp * gamma.sin() + ypp * gamma.cos();
    let zp = zpp * gamma.cos() - ypp * gamma.sin();
    let rp = xp * xp + yp * yp + zp * zp;
    rp as usize
}

fn light_value (par: [f32; 8], tr: ([f32; 3], [f32; 3], [f32; 3])) -> u8 {
    let p1 = tr.0;
    let p2 = tr.1;
    let p3 = tr.2;
    let x1 = p2[0] - p1[0];
    let y1 = p2[1] - p1[1];
    let z1 = p2[2] - p1[2];
    let x2 = p3[0] - p1[0];
    let y2 = p3[1] - p1[1];
    let z2 = p3[2] - p1[2];
    let xl = par[5] - (p1[0]+ p2[0] + p3[0])/3.0;
    let yl = par[6] - (p1[1]+ p2[1] + p3[1])/3.0;
    let zl = par[7] - (p1[2]+ p2[2] + p3[2])/3.0;
    let ntr = [y1 * z2 - y2 * z1,
                               z1 * x2 - z2 * x1,
                               x1 * y2 - x2 * y1];
    let light = (xl * ntr[0] + yl * ntr[1] + zl * ntr[2]).abs()
    / (xl * xl + yl * yl + zl * zl).sqrt() 
    / (ntr[0] * ntr[0] + ntr[1] * ntr[1] + ntr[2] * ntr[2]).sqrt();
    (light * 255.0) as u8
}

// draws object
fn draw_object (par: [f32; 8], delta: f32, gamma: f32, trn: Vec<([f32; 3], [f32; 3], [f32; 3])>) {
    let width = par[3];
    let height = par[4];
    let mut points_data: Vec<(usize, [[f32; 2]; 3], u8)> = Vec::new();
    for tr in trn {
        let p1 = tr.0;
        let p2 = tr.1;
        let p3 = tr.2;
        let m = middle_point((p1, p2, p3));
        let dt = dist_to_camera(par, delta, gamma, m);
        let light = light_value(par, tr);

        points_data.push((dt, 
            [project_coord(par, delta, gamma, p1),
            project_coord(par, delta, gamma, p2),
            project_coord(par, delta, gamma, p3)],
        light));
    }
    points_data.sort_by_key(|k| k.0);
    points_data.reverse();
    // drawing
    for p in points_data {
        let light = p.2;
        let u1 = p.1[0][0];
        let v1 = p.1[0][1];
        let u2 = p.1[1][0];
        let v2 = p.1[1][1];
        let u3 = p.1[2][0];
        let v3 = p.1[2][1];
        let col = Color::from_rgba(light, light, light, 255);
        if u1 > 0.0 && u1 < width && v1 > 0.0 && v1 < height
        && u2 > 0.0 && u2 < width && v2 > 0.0 && v2 < height
        && u3 > 0.0 && u3 < width && v3 > 0.0 && v3 < height {
            draw_triangle(vec2(u1, v1), vec2(u2, v2), vec2(u3, v3), col)
        }
    }

}

// draw light source
fn _draw_light (par: [f32; 8], delta: f32, gamma: f32) {
    let width = par[3];
    let height = par[4];
    let xl = par[5];
    let yl = par[6];
    let zl = par[7];
    let p = project_coord(par, delta, gamma, [xl, yl, zl]);
    let u = p[0];
    let v = p[1];
    let rp = dist_to_camera(par, delta, gamma, [xl, yl, zl]) as f32;
    let d = par[0];
    let dp = 2.0 * par[1] * (0.5 * d / rp.sqrt()).asin();
    if u > 0.0 && u < width && v > 0.0 && v < height {
        draw_circle(u, v, dp, Color::from_rgba(255, 255, 255, 255))
        }
}

#[macroquad::main("Simple 3D scene")]
async fn main() {

    // camera rotation angles
    let mut delta = 0.0f32;
    let mut gamma = 0.0f32;
    // object center coordinates
    let mut x0 = 0.0f32;
    let mut y0 = 0.0f32;
    let mut z0 = 0.0f32;
    // frame time
    let mut ft: f32;
    // screen parameters
    let mut par: [f32; 8];
    let mut d: f32;
    // object to render
    let mut tetrahedron: Vec<([f32; 3], [f32; 3], [f32; 3])>;

    // start the main loop
    loop {

        clear_background(BLACK);

        
        ft = get_frame_time() as f32;

        par = get_window_par(screen_height(), screen_width());
        d = par[0];

        draw_text(format!("Use arrow keys to rotate the camera").as_str(), 5.0 * d, 3.0 * d, 2.5 * d, WHITE);
        draw_text(format!("Use [Q][W][E][A][S][D] keys to move the object").as_str(), 5.0 * d, 6.0 * d, 2.5 * d, WHITE);
        draw_text(format!("FPS is {}", get_fps()).as_str(), 5.0 * d, 9.0 * d, 2.5 * d, WHITE);

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

        if is_key_down(KeyCode::Q) {
            y0 = y0 + SPEEDX * ft;
        }

        if is_key_down(KeyCode::E) {
            y0 = y0 - SPEEDX * ft;
        }

        if is_key_down(KeyCode::W) {
            z0 = z0 + SPEEDX * ft;
        }

        if is_key_down(KeyCode::S) {
            z0 = z0 - SPEEDX * ft;
        }

        if is_key_down(KeyCode::A) {
            x0 = x0 - SPEEDX * ft;
        }

        if is_key_down(KeyCode::D) {
            x0 = x0 + SPEEDX * ft;
        }

        tetrahedron = create_tetrahedron(x0, y0, z0, 30.0 * d);
        
        draw_object(par, delta, gamma, tetrahedron);

        //draw_light(par, delta, gamma);

        

        next_frame().await;
    }  
    
}

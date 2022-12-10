use std::fs::File;
use std::io::Write;

const TIMESTEP: f64 = 900.0;
const GRAV_CONST: f64 = 0.0000000000667430;

struct Body {
    mass: f64,
    position: [f64; 3],
    last_position: [f64; 3],
    accel: [f64; 3]
}

fn vec_add(vec1: &[f64; 3], vec2: &[f64; 3], subtract: bool) -> [f64; 3] {
    let mut new_vec: [f64; 3] = [0.0; 3];
    if subtract {
        for i in 0..3 {
            new_vec[i] = vec1[i] - vec2[i];
        }
    }
    else {
        for i in 0..3 {
            new_vec[i] = vec1[i] + vec2[i];
        }
    }
    new_vec
}

fn scalar_mult(vec: &[f64; 3], scalar: f64) -> [f64; 3] {
    let mut new_vec: [f64; 3] = [0.0; 3];
    for i in 0..3 {
        new_vec[i] = scalar*vec[i]; 
    }
    new_vec
}

fn norm_dist(pos1: &[f64;3], pos2: &[f64;3], dist_sq: f64) -> [f64; 3] {
    //! returns the normalized distance vector between the given positions (from pos1 to post2)
    let norm_fact = 1.0/f64::sqrt(dist_sq);
    let dist_vec = vec_add(pos2, pos1, true);
    scalar_mult(&dist_vec, norm_fact)
}

fn dist_sq(pos1: &[f64;3], pos2: &[f64;3]) -> f64 {
    //! returns the square of the distance between the given positions
    let mut sum = 0.0;
    for i in 0..3 {
        sum += f64::powi(pos1[i]-pos2[i], 2)
    }
    sum
}

fn body_init(mass: f64, pos: [f64; 3], vel: [f64; 3]) -> Body {
    //! makes a body with the given params
    return Body { mass: mass, position: pos, last_position: vec_add(&pos, &scalar_mult(&vel, TIMESTEP), true), accel: [0.0; 3] }
}

fn update_accel(body1: &mut Body, body2: &Body) {
    //! Updates the acceleration vector of the first body provided by the force applied by the second body
    let dist_sq = dist_sq(&body1.position, &body2.position);
    let accel_total = GRAV_CONST*body2.mass/dist_sq;
    let norm_dist = norm_dist(&body1.position, &body2.position, dist_sq);
    body1.accel = vec_add(&body1.accel, &scalar_mult(&norm_dist, accel_total), false);
}

fn update_pos(body: &mut Body) {
    //! update the position of the passed body, using the TIMESTEP const
    let hold = body.position;
    // x(t+dt) = 2x(t)-x(x-dt)+a(t)*dt^2
    body.position = vec_add(&vec_add(&scalar_mult(&body.position, 2.0), &body.last_position, true), &scalar_mult(&body.accel, f64::powf(TIMESTEP, 2.0)), false);
    body.last_position = hold;
    body.accel = [0.0; 3];
}

fn main() {
    let mut sun = body_init(4.385*f64::powi(10.0, 30), [0.0; 3], [0.0; 3]);
    let mut earth = body_init(5.972*f64::powi(10.0, 24), [1.49*f64::powi(10.0, 11), 0.0, 0.0], [0.0, 45000.0, 0.0,]);
    
    let mut earth_pos_hist: Vec<[f64; 3]> = Vec::new();
    let mut sun_pos_hist: Vec<[f64; 3]> = Vec::new();
    
    for _i in 0..35064 {
        // the actual sim
        update_accel(&mut earth, &sun);
        update_accel(&mut sun, &earth);
        update_pos(&mut earth);
        update_pos(&mut sun);
        earth_pos_hist.push(earth.position);
        sun_pos_hist.push(sun.position);
    }


    //writing the data generated
    let mut earth_data = File::create("earth.csv").expect("Unable to create file");
    write!(earth_data, "t,x,y,z, \n").expect("write fail lol");   
    for i in 0..earth_pos_hist.len(){    
        write!(earth_data, "{},", TIMESTEP*(i as f64)).expect("write fail lol");   
        for j in 0..3 {
            write!(earth_data, "{},", earth_pos_hist[i][j]).expect("write fail lol");   
        }
        write!(earth_data, "\n").expect("write fail lol"); 
    }

    let mut sun_data = File::create("sun.csv").expect("Unable to create file");
    write!(sun_data, "t,x,y,z, \n").expect("write fail lol");   
    for i in 0..sun_pos_hist.len(){
        write!(sun_data, "{},", TIMESTEP*(i as f64)).expect("write fail lol"); 
        for j in 0..3 {
            write!(sun_data, "{},", sun_pos_hist[i][j]).expect("write fail lol");   
        }
        write!(sun_data, "\n").expect("write fail lol"); 
    }     

}
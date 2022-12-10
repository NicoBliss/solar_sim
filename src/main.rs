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

struct SolarSystem {
    mass: Vec<f64>,
    position: Vec<[f64; 3]>,
    last_position: Vec<[f64; 3]>,
    accel: Vec<[f64; 3]>
}

// vector math functions
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

fn dist_sq(pos1: &[f64;3], pos2: &[f64;3]) -> f64 {
    //! returns the square of the distance between the given positions
    let mut sum = 0.0;
    for i in 0..3 {
        sum += f64::powi(pos1[i]-pos2[i], 2)
    }
    sum
}

fn norm_dist(pos1: &[f64;3], pos2: &[f64;3], dist_sq: f64) -> [f64; 3] {
    //! returns the normalized distance vector between the given positions (from pos1 to post2)
    let norm_fact = 1.0/f64::sqrt(dist_sq);
    let dist_vec = vec_add(pos2, pos1, true);
    scalar_mult(&dist_vec, norm_fact)
}

// init functions
fn body_init(mass: f64, pos: [f64; 3], vel: [f64; 3]) -> Body {
    //! makes a body with the given params
    return Body { mass: mass, position: pos, last_position: vec_add(&pos, &scalar_mult(&vel, TIMESTEP), true), accel: [0.0; 3] }
}

fn solar_init(system: Vec<Body>) -> SolarSystem {
    let mut mass:Vec<f64> = Vec::new();
    let mut position:Vec<[f64;3]> = Vec::new();
    let mut last_position:Vec<[f64;3]> = Vec::new();
    let mut accel:Vec<[f64;3]> = Vec::new();

    for i in system {
        mass.push(i.mass);
        position.push(i.position);
        last_position.push(i.last_position);
        accel.push(i.accel);
    }

    return  SolarSystem {
        mass: mass, 
        position: position, 
        last_position: 
        last_position, 
        accel: accel 
    }
}

// functions to update bodies
fn _update_accel(body1: &mut Body, body2: &Body) {
    //! Updates the acceleration vector of the first body provided by the force applied by the second body
    let dist_sq = dist_sq(&body1.position, &body2.position);
    let accel_total = GRAV_CONST*body2.mass/dist_sq;
    let norm_dist = norm_dist(&body1.position, &body2.position, dist_sq);
    body1.accel = vec_add(&body1.accel, &scalar_mult(&norm_dist, accel_total), false);
}

fn _update_pos(body: &mut Body) {
    //! update the position of the passed body, using the TIMESTEP const
    let hold = body.position;
    // x(t+dt) = 2x(t)-x(x-dt)+a(t)*dt^2
    body.position = vec_add(&vec_add(&scalar_mult(&body.position, 2.0), &body.last_position, true), &scalar_mult(&body.accel, f64::powf(TIMESTEP, 2.0)), false);
    body.last_position = hold;
    body.accel = [0.0; 3];
}

// functions to update a solar system
fn accel_factor(pos1: &[f64;3], pos2: &[f64;3], mass2: f64) -> [f64;3] {
    //! returns the acceleration vector of the first body provided by the force applied by the second body
    let dist_sq = dist_sq(&pos1, &pos2);
    let accel_total = GRAV_CONST*mass2/dist_sq;
    let norm_dist = norm_dist(&pos1, &pos2, dist_sq);
    scalar_mult(&norm_dist, accel_total)
}

fn update_planet_pos(solar: &mut SolarSystem, index: usize) {
    //! update the position of the pointed to body in the passed solar sytem, using the TIMESTEP const
    let hold = solar.position[index];
    // x(t+dt) = 2x(t)-x(x-dt)+a(t)*dt^2
    solar.position[index] = vec_add(&vec_add(&scalar_mult(&solar.position[index], 2.0), &solar.last_position[index], true), &scalar_mult(&solar.accel[index], f64::powf(TIMESTEP, 2.0)), false);
    solar.last_position[index] = hold;
    solar.accel[index] = [0.0; 3];
}

fn update_system(solar: &mut SolarSystem) {
    //! updates the accel of every object with every other object, then updates their positions (using TIMESTEP const ofc)
    for i in 0..solar.mass.len() {
        for j in 0..solar.mass.len() {
            if i == j {continue;}
            solar.accel[i] = vec_add(&solar.accel[i], &accel_factor(&solar.position[i], &solar.position[j], solar.mass[j]), false);
        }
    }

    for i in 0..solar.mass.len() {
        update_planet_pos(solar, i)
    }
}

fn head_data_files(solar: &SolarSystem) -> Vec<File> {
    let mut data: Vec<File> = Vec::new();
    let mut path: String;

    for i in 0..solar.mass.len() {
        path = format!("data/{}.csv", i); 
        data.push(File::create(path).expect("Unable to create file"));
        write!(data[i], "t,x,y,z, \n").expect("write fail lol");
    }

    data
}

fn record_sys_state(solar: &SolarSystem, hist_files: &mut Vec<File>, time: f64) {
    for i in 0..solar.mass.len() {
        write!(hist_files[i], "{},", time).expect("write fail lol");   
        for j in 0..3 {
            write!(hist_files[i], "{},", solar.position[i][j]).expect("write fail lol");   
        }
        write!(hist_files[i], "\n").expect("write fail lol"); 
    }
}

fn main() {
    let sun = body_init(4.385*f64::powi(10.0, 30), [0.0; 3], [0.0; 3]);
    let earth = body_init(5.972*f64::powi(10.0, 24), [1.49*f64::powi(10.0, 11), 0.0, 0.0], [0.0, 45000.0, 0.0,]);
    let moon = body_init(7.38*f64::powi(10.0, 22),  [1.49*f64::powi(10.0, 11) + 3.843999*f64::powi(10.0, 8), 0.0, 0.0], [0.0, 46022.0, 0.0,]);

    let mut solar_sys = solar_init(vec![sun, earth, moon]);
    let mut sim_hist_files = head_data_files(&solar_sys);
    let mut time = 0.0;
    
    // the actual sim
    for _i in 0..35064 {
        time += TIMESTEP;
        // update the system state (where all the real computation happens)
        update_system(&mut solar_sys);
        //write the new data to the data files
        record_sys_state(&solar_sys, &mut sim_hist_files, time)
    }

}
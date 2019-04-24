#[derive(Copy, Clone, Debug)]
pub struct PhysObject2d {
    //in kilograms
    pub mass: f64,
    //in meters
    pub pos: [f64; 2],
    //in meters per second
    pub vel: [f64; 2],
    //in coulombs
    pub charge: f64,
    //in newtons
    pub force: [f64; 2],
    //color
    pub color: [f32; 4]
}

impl PhysObject2d {
    pub fn new (mass: f64, pos: [f64; 2], vel: [f64; 2], charge: f64, color: [f32; 4]) -> PhysObject2d {
        PhysObject2d {
            mass: mass,
            pos: pos,
            vel: vel,
            charge: charge,
            force: [0.0, 0.0],
            color: color
        }
    }
}

pub fn phys_step (mut phys_objects: Vec<PhysObject2d>, time_step: f64) -> Vec<PhysObject2d>{
    //physics
    //applies force of outer object to inner object
    let g_const: f64 = 0.0000000000667408;
    let c_const: f64 = 8987551787.0;
    let merge_dist: f64 = 3.0;

    //reset forces
    for object in 0..phys_objects.len() {
        phys_objects[object].force = [0.0, 0.0];
    }

    //calculate cumulative forces on all objects
    for outer in 0..phys_objects.len() {
        for inner in 0..phys_objects.len() {
            if inner != outer {
                let distance: f64 = ((phys_objects[outer].pos[0] - phys_objects[inner].pos[0]).powi(2) + (phys_objects[outer].pos[1] - phys_objects[inner].pos[1]).powi(2)).sqrt();
                if distance <= merge_dist {

                } else {
                    //components
                    let h_scale: f64 = (phys_objects[outer].pos[0] - phys_objects[inner].pos[0]) / distance;
                    let v_scale: f64 = (phys_objects[outer].pos[1] - phys_objects[inner].pos[1]) / distance;

                    //forces
                    let g_force = g_const * phys_objects[outer].mass * phys_objects[inner].mass / distance.powi(2);
                    let e_force = c_const * phys_objects[outer].charge * phys_objects[inner].charge / distance.powi(2);

                    phys_objects[inner].force[0] = phys_objects[inner].force[0] + (g_force - e_force) * h_scale;
                    phys_objects[inner].force[1] = phys_objects[inner].force[1] + (g_force - e_force) * v_scale;
                }
            }
        }
    }

    //calculate acceleration per objects and new position
    for object in 0..phys_objects.len() {
        let dvx = time_step * phys_objects[object].force[0] / phys_objects[object].mass;
        let dvy = time_step * phys_objects[object].force[1] / phys_objects[object].mass;

        phys_objects[object].vel = [phys_objects[object].vel[0] + dvx, phys_objects[object].vel[1] + dvy];

        phys_objects[object].pos = [phys_objects[object].pos[0] + phys_objects[object].vel[0], phys_objects[object].pos[1] + phys_objects[object].vel[1]];
    }

    phys_objects
}

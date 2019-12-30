#[derive(Debug)]
pub struct Sphere {
    pub id: u32,
}

pub fn new() -> Sphere {
    Sphere { id: sphere_id() }
}


static mut SPHERE_ID: u32 = 0;

pub fn sphere_id() -> u32 {
    unsafe {
        SPHERE_ID += 1;
        SPHERE_ID
    }
}

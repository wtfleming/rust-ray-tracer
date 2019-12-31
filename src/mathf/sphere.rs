#[derive(Debug, Clone)]
pub struct Sphere {
    pub id: u32,
}

pub fn new() -> Sphere {
    Sphere { id: sphere_id() }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

static mut SPHERE_ID: u32 = 0;

pub fn sphere_id() -> u32 {
    unsafe {
        SPHERE_ID += 1;
        SPHERE_ID
    }
}

#[derive(Debug)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4 {
        Vector4 { x, y, z, w }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_vector4() {
        let vector = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
        assert_eq!(vector.w, 4.0);
    }
}

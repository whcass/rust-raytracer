pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y:u32, scene: &Scene)-> Ray{
        Ray{
            origin:Point::zero(),
            direction: Vector3::zero(),
        }
    }
}


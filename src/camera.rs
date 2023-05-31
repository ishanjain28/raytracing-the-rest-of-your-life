use {
    crate::types::{Ray, Vec3},
    rand::Rng,
};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f64,

    // position vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,

    shutter_open: f64,
    shutter_close: f64,
}

impl Camera {
    // vertical_fov is the viewable angle from top->bottom
    // look_from is basically camera position
    // look_at is the point where camera is looking
    // v_up is camera's up vector. i.e. it points upwards from the camera
    // orthogonal to look_from - look_at vector
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        vertical_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
        shutter_open: f64,
        shutter_close: f64,
    ) -> Self {
        // convert degree to radian
        let angle = vertical_fov * std::f64::consts::PI / 180.0;
        let half_height = (angle / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let lower_left_corner = origin
            - u * focus_distance * half_width
            - v * focus_distance * half_height
            - w * focus_distance;
        let horizontal = u * half_width * focus_distance * 2.0;
        let vertical = v * half_height * focus_distance * 2.0;
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
            w,
            shutter_open,
            shutter_close,
        }
    }

    pub fn get_ray<R: Rng + ?Sized>(&self, u: f64, v: f64, rng: &mut R) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = rng.gen_range(self.shutter_open..=self.shutter_close);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
            time,
        )
    }
}

fn random_in_unit_disk<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
    let mut p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);

    while p.dot(&p) >= 1.0 {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) * 2.0 - Vec3::new(1.0, 0.0, 0.0);
    }
    p
}

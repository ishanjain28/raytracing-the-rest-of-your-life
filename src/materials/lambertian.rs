use rand::prelude::SmallRng;

use crate::{
    hitable::HitRecord,
    materials::{random_point_in_unit_hemisphere, random_point_in_unit_sphere},
    types::{Ray, Vec3},
    Material, Texture,
};

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture + Send + Sync> Material for Lambertian<T> {
    fn scatter(
        &self,
        ray: &Ray,
        hit_rec: &HitRecord,
        rng: &mut SmallRng,
    ) -> (Vec3, f64, Option<Ray>) {
        let direction = random_point_in_unit_hemisphere(rng, &hit_rec.normal);

        let scattered_ray = Ray::new(hit_rec.p, direction.unit_vector(), ray.time());
        (
            self.albedo.value(hit_rec.u, hit_rec.v, hit_rec.p),
            0.5 / std::f64::consts::PI,
            Some(scattered_ray),
        )
    }

    fn scatter_pdf(&self, _ray: &Ray, hit_rec: &HitRecord, scatterd: &Ray) -> f64 {
        let cosine: f64 = hit_rec.normal.dot(&scatterd.direction.unit_vector());

        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }
}

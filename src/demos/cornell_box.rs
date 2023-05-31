use std::sync::Arc;

use crate::{
    demos::Demo,
    hitable::{
        hitable_list::HitableList,
        shapes::{Cuboid, RectBuilder, Sphere},
        Hitable,
    },
    materials::{DiffuseLight, Lambertian, MaterialBuilder, Metal},
    texture::Solid,
    types::Vec3,
    Camera,
};

pub struct CornellBox {}

impl Demo for CornellBox {
    type DemoT = HitableList;

    fn name(&self) -> &'static str {
        "cornell_box"
    }

    fn world(&self) -> Self::DemoT {
        let red = Lambertian::new(Solid::new(Vec3::new(0.65, 0.05, 0.05)));
        let white = Lambertian::new(Solid::new(Vec3::new(0.73, 0.73, 0.73)));
        let green = Lambertian::new(Solid::new(Vec3::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight::new(Solid::new(Vec3::splat(15)));

        let mut objects = HitableList { list: Vec::new() };

        objects.push(Arc::new(
            RectBuilder
                .y(0.0..=555.0)
                .z(0.0..=555.0)
                .x(555.0)
                .material(green),
        ));
        objects.push(Arc::new(
            RectBuilder
                .y(0.0..=555.0)
                .z(0.0..=555.0)
                .x(0.0)
                .material(red),
        ));
        objects.push(Arc::new(
            RectBuilder
                .x(213.0..=343.0)
                .z(227.0..=332.0)
                .y(554.0)
                .material(light),
        ));
        objects.push(Arc::new(
            RectBuilder
                .x(0.0..=555.0)
                .z(0.0..=555.0)
                .y(555.0)
                .material(white.clone()),
        ));
        objects.push(Arc::new(
            RectBuilder
                .x(0.0..=555.0)
                .z(0.0..=555.0)
                .y(0.0)
                .material(white.clone()),
        ));
        objects.push(Arc::new(
            RectBuilder
                .x(0.0..=555.0)
                .y(0.0..=555.0)
                .z(555.0)
                .material(white.clone()),
        ));

        objects.push(Arc::new(
            Cuboid::new(
                Vec3::splat(0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white.clone(),
            )
            .rotate_y(15.0)
            .translate(Vec3::new(265.0, 0.0, 295.0)),
        ));

        objects.push(Arc::new(
            Cuboid::new(Vec3::splat(0.0), Vec3::splat(165.0), white)
                .rotate_y(-18.0)
                .translate(Vec3::new(130.0, 0.0, 65.0)),
        ));

        objects
    }

    fn camera(&self, aspect_ratio: f64) -> Camera {
        let lookfrom = Vec3::new(278.0, 278.0, -800.0);
        let lookat = Vec3::new(278.0, 278.0, 0.0);
        let aperture = 0.0;
        let focus_distance = 40.0;
        Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            aspect_ratio,
            aperture,
            focus_distance,
            0.0,
            1.0,
        )
    }
}

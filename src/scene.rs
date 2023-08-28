use std::path::Path;

use nalgebra::Vector3;

use crate::{world::World, geometry::{Plane, Point, Sphere, Mesh}, color, material};

pub fn build_scene(w: &mut World) {
        // w.add_object(Box::new(
        //     Sphere::new_with_material(0.0, 2.0, -4.0, 1.5, 
        //         Box::new(crate::material::Diffuse::new(color::BLUE)))));
        // w.add_object(Box::new(
        //     Sphere::new_with_material(3.0, 0.0, -5.0, 1.0, 
        //         Box::new(crate::material::Metal::new(color::WHITE, 0.0)))));
        w.add_object(Box::new(Plane::new(
            Point::new(0., -1., 0.),
            Vector3::new(0., 1., 0.),
            // Box::new(crate::material::Diffuse::new(color::WHITE)),
            Box::new(crate::material::Dielectric::new(color::WHITE, 0.05, 2.417)),
        )));

        w.add_object(Box::new(
            Sphere::new_with_material(-3.0, 2.0, -5.0, 0.2, 
                Box::new(crate::material::Emmisive::new(color::ORANGE, 2.3)))));

        // w.add_object(Box::new(
        //     Sphere::new_with_material(-3.0, 0.7, -2.0, 1.0, 
        //         Box::new(crate::material::Dielectric::new(color::WHITE, 0.05, 1.4)))));

        let mut mesh = Mesh::from_obj(Path::new("assets/cow.obj"), Box::new(material::Dielectric::new(color::WHITE, 0.05, 2.1)));
        mesh.build_bvh();
        w.add_object( Box::new(mesh));

       let mut mesh = Mesh::from_obj(Path::new("assets/text.obj"), Box::new(material::Emmisive::new(color::WHITE, 2.3)));
        mesh.build_bvh();
        w.add_object( Box::new(mesh));

        // w.add_object(
        //     Box::new(
        //         Rectangle::new(
        //             Point::new(-10.0, 12.0, -10.0),
        //             Point::new(10.0, 3.0, 10.0),
        //             Box::new(crate::material::Diffuse::new(color::WHITE)),
        //         )
        //     )
        // );

}
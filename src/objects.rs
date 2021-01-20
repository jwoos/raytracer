use crate::ray::Ray;
use crate::vec3::{Color, Point, Vec3};
use std::rc::Rc;
use std::vec::Vec;

pub trait Material {
    fn scatter(&self, r: &Ray, record: &HitRecord) -> (bool, Color, Ray);
}

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_facing: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Point,
        normal: Vec3,
        t: f64,
        front_facing: bool,
        material: Rc<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            point,
            normal,
            t,
            front_facing,
            material,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_facing = Vec3::dot(r.get_direction(), &outward_normal) < 0.0;
        if self.front_facing {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t: (f64, f64)) -> (bool, Option<HitRecord>);
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: &Ray, t: (f64, f64)) -> (bool, Option<HitRecord>) {
        let mut hit = false;
        let mut closest = t.1;

        let mut record: Option<HitRecord> = None;

        for obj in &self.objects {
            let (inner_hit, hit_record_opt) = obj.hit(r, (t.0, closest));
            if inner_hit {
                hit = true;

                match hit_record_opt {
                    Some(rec) => {
                        closest = rec.t;
                        record = Some(rec);
                    }
                    _ => {
                        panic!("Should not get here");
                    }
                }
            }
        }

        return (hit, record);
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, record: &HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal.clone();
        }

        let ray = Ray::new(record.point, scatter_direction);
        let attenuation = self.albedo.clone();

        return (true, attenuation, ray);
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        let mut fuzz = f;
        if fuzz > 1.0 {
            fuzz = 1.0;
        }
        Metal {
            albedo: a,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, record: &HitRecord) -> (bool, Color, Ray) {
        let reflected = Vec3::reflect(&r.get_direction().unit(), &record.normal);
        let scattered = Ray::new(
            record.point,
            reflected + self.fuzz * Vec3::random_unit_vector(),
        );
        let attenuation = self.albedo.clone();

        return (
            Vec3::dot(scattered.get_direction(), &record.normal) > 0.0,
            attenuation,
            scattered,
        );
    }
}

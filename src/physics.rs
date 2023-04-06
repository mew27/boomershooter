use bevy::prelude::*;

#[derive(Component)]
pub struct AA_Hitbox {
    pub origin: Vec3,
    pub extent: Vec3,
}

#[derive(Component)]
pub struct Hitray(pub Ray);

impl AA_Hitbox {
    pub fn get_minmax(&self) -> (Vec3, Vec3) {
        (self.origin, self.origin + self.extent)
    }
}

//check ray collision with slab method, return tuple with ray distance of entry / exit point
pub fn check_collision(hitbox: &AA_Hitbox, hitray: &Hitray) -> Option<(f32, f32)> {
    let hitbox_minmax = hitbox.get_minmax();

    let (mut tmin, mut tmax) = (-f32::INFINITY, f32::INFINITY);

    // x slab collision
    if hitray.0.direction.x != 0. {
        let tx1 = (hitbox_minmax.0.x - hitray.0.origin.x) / hitray.0.direction.x;
        let tx2 = (hitbox_minmax.1.x - hitray.0.origin.x) / hitray.0.direction.x;

        tmin = tmin.max(tx1.min(tx2));
        tmax = tmax.min(tx1.max(tx2));
    }

    //y slab collision
    if hitray.0.direction.y != 0. {
        let ty1 = (hitbox_minmax.0.y - hitray.0.origin.y) / hitray.0.direction.y;
        let ty2 = (hitbox_minmax.1.y - hitray.0.origin.y) / hitray.0.direction.y;

        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));
    }

    //z slab collision
    if hitray.0.direction.z != 0. {
        let tz1 = (hitbox_minmax.0.z - hitray.0.origin.z) / hitray.0.direction.z;
        let tz2 = (hitbox_minmax.1.z - hitray.0.origin.z) / hitray.0.direction.z;

        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));
    }

    if tmax >= tmin {
        return Some((tmin, tmax));
    } else {
        return None;
    }
}

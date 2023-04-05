use bevy::prelude::*;

//check collision with slab method
pub fn check_collision(hitbox : &Transform, hitray : &Ray) -> bool {
    let hitbox_size   = Vec3 {x: 2.5, y: 2.5, z: 2.5};
    let hitbox_pos    = hitbox.translation;
    let hitbox_minmax = (hitbox_pos - hitbox_size, hitbox_pos + hitbox_size);
                
    //println!("hitbox_minmax = {:?}", hitbox_minmax);
    //println!("hitbox_pos    = {:?}", hitbox_pos);
    //println!("hitray        = {:?}", (hitray));

    let (mut tmin, mut tmax) = (- f32::INFINITY, f32::INFINITY);

    // x slab collision
    if hitray.direction.x != 0. {
        let tx1 = (hitbox_minmax.0.x - hitray.origin.x) / hitray.direction.x;
        let tx2 = (hitbox_minmax.1.x - hitray.origin.x) / hitray.direction.x;

        tmin = tmin.max(tx1.min(tx2));
        tmax = tmax.min(tx1.max(tx2));
    }

    //y slab collision
    if hitray.direction.y != 0. {
        let ty1 = (hitbox_minmax.0.y - hitray.origin.y) / hitray.direction.y;
        let ty2 = (hitbox_minmax.1.y - hitray.origin.y) / hitray.direction.y;

        tmin = tmin.max(ty1.min(ty2)); 
        tmax = tmax.min(ty1.max(ty2));
    }

    //z slab collision
    if hitray.direction.z != 0. {
        let tz1 = (hitbox_minmax.0.z - hitray.origin.z) / hitray.direction.z;
        let tz2 = (hitbox_minmax.1.z - hitray.origin.z) / hitray.direction.z;

        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));
    }

    if tmax >= tmin {
        return true;
    } else {
        return false;
    }
}

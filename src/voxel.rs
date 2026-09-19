//! Tiny CPU voxel showcase, not a general-purpose terrain engine.
use bevy::prelude::*;

pub struct Volume {
    side: i32,
    cells: Vec<[u8; 3]>,
}

impl Volume {
    pub fn new(side: i32, mut color: impl FnMut(Vec3) -> [u8; 3]) -> Self {
        let cells = (0..side * side * side)
            .map(|i| {
                let p = Vec3::new(
                    (i % side) as f32,
                    ((i / side) % side) as f32,
                    (i / (side * side)) as f32,
                );
                color(p + Vec3::splat(0.5 - side as f32 / 2.0))
            })
            .collect();
        Self { side, cells }
    }

    // Traverse voxel boundaries exactly; return the first solid cell and face normal.
    fn cast(&self, origin: Vec3, direction: Vec3) -> Option<([u8; 3], Vec3)> {
        let origin = origin + Vec3::splat(self.side as f32 / 2.0);
        let inverse = direction.recip();
        let a = -origin * inverse;
        let b = (Vec3::splat(self.side as f32) - origin) * inverse;
        let mut near = a.min(b);
        let mut far = a.max(b);
        // A ray parallel to a slab never crosses that slab's boundaries.
        for axis in 0..3 {
            if direction[axis].abs() < 1e-8 {
                if origin[axis] < 0.0 || origin[axis] >= self.side as f32 {
                    return None;
                }
                near[axis] = f32::NEG_INFINITY;
                far[axis] = f32::INFINITY;
            }
        }
        let entry = near.max_element().max(0.0);
        if entry > far.min_element() {
            return None;
        }
        let p = origin + direction * (entry + 0.0001);
        let mut cell = p.floor().as_ivec3();
        let step = direction.signum().as_ivec3();
        let boundary = cell.as_vec3()
            + Vec3::new(
                (direction.x > 0.0) as u8 as f32,
                (direction.y > 0.0) as u8 as f32,
                (direction.z > 0.0) as u8 as f32,
            );
        let mut next = (boundary - origin) * inverse;
        for axis in 0..3 {
            if direction[axis].abs() < 1e-8 {
                next[axis] = f32::INFINITY;
            }
        }
        let delta = inverse.abs();
        let mut normal = -direction;
        for _ in 0..self.side * 3 {
            if cell.min_element() < 0 || cell.max_element() >= self.side {
                return None;
            }
            let index = cell.x + self.side * (cell.y + self.side * cell.z);
            let color = self.cells[index as usize];
            if color != [0, 0, 0] {
                return Some((color, normal));
            }
            let axis = if next.x < next.y && next.x < next.z {
                0
            } else if next.y < next.z {
                1
            } else {
                2
            };
            cell[axis] += step[axis];
            next[axis] += delta[axis];
            normal = Vec3::ZERO;
            normal[axis] = -step[axis] as f32;
        }
        None
    }

    pub fn render(&self, pixels: &mut [u8], size: usize, extent: f32, rotation: Quat) {
        let direction = rotation * -Vec3::Z;
        let light = rotation * Vec3::new(-0.5, 0.8, 1.0).normalize();
        for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
            let x = ((i % size) as f32 + 0.5) / size as f32 * 2.0 - 1.0;
            let y = 1.0 - ((i / size) as f32 + 0.5) / size as f32 * 2.0;
            let origin = rotation * Vec3::new(x * extent, y * extent, self.side as f32 * 1.5);
            if let Some((color, normal)) = self.cast(origin, direction) {
                let shade = 0.42 + 0.58 * normal.dot(light).max(0.0);
                pixel.copy_from_slice(&[
                    (color[0] as f32 * shade) as u8,
                    (color[1] as f32 * shade) as u8,
                    (color[2] as f32 * shade) as u8,
                    255,
                ]);
            } else {
                pixel.fill(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn axis_aligned_rays_hit_faces_and_miss_outside_volume() {
        let volume = Volume::new(8, |p| {
            if p.abs().max_element() < 2.0 {
                [100, 150, 80]
            } else {
                [0; 3]
            }
        });
        for axis in [Vec3::X, Vec3::Y, Vec3::Z, -Vec3::X, -Vec3::Y, -Vec3::Z] {
            let (color, normal) = volume.cast(axis * 10.0, -axis).unwrap();
            assert_eq!(color, [100, 150, 80]);
            assert_eq!(normal, axis);
        }
        assert!(volume.cast(Vec3::new(9.0, 0.0, 10.0), -Vec3::Z).is_none());
    }
}

pub fn helicopter() -> Volume {
    Volume::new(40, |p| {
        let in_box = |lo: Vec3, hi: Vec3| p.cmpge(lo).all() && p.cmple(hi).all();
        if in_box(Vec3::new(-8., -3., -4.), Vec3::new(10., 4., 4.)) {
            if p.x > 3. && p.y > 0. {
                [99, 217, 226]
            } else if p.y > 2. {
                [179, 190, 105]
            } else if p.x < -3. && p.y > -1. && p.y < 1. && p.z > 3. {
                [255, 197, 72]
            } else {
                [99, 124, 64]
            }
        } else if in_box(Vec3::new(-18., 0., -1.), Vec3::new(-8., 2., 1.))
            || in_box(Vec3::new(-18., 0., -1.), Vec3::new(-16., 7., 1.))
        {
            [137, 157, 82]
        } else if (p.z.abs() > 3. && p.z.abs() < 5. && p.x.abs() < 10. && p.y > -7. && p.y < -6.)
            || (p.x.abs() > 4.
                && p.x.abs() < 6.
                && p.z.abs() > 3.
                && p.z.abs() < 4.
                && p.y > -6.
                && p.y < -3.)
        {
            [170, 185, 180]
        } else if p.x.abs() < 1. && p.z.abs() < 1. && p.y > 4. && p.y < 8. {
            [65, 76, 72]
        } else {
            [0, 0, 0]
        }
    })
}

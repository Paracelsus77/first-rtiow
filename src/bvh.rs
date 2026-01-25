use crate::{Aabb, HitRecord, Hittable, Interval, Primitive, Ray, random_range_u32};

pub enum BvhNode {
    Leaf {
        bbox: Aabb,
        object_offset: usize,
    },
    Interior {
        bbox: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

impl BvhNode {
    pub fn new(objects: &mut [Primitive], start_index: usize) -> Self {
        let span = objects.len();

        if span == 1 {
            return BvhNode::Leaf {
                bbox: objects[0].bounding_box(),
                object_offset: start_index,
            };
        }

        let axis = random_range_u32(0, 2);
        let comparator = |a: &Primitive, b: &Primitive| {
            let min_a = a.bounding_box().axis(axis).min;
            let min_b = b.bounding_box().axis(axis).min;
            min_a.total_cmp(&min_b)
        };

        objects.sort_unstable_by(comparator); // mutable sort

        let mid = span / 2;
        let (left_objects, right_objects) = objects.split_at_mut(span / 2);

        let left = BvhNode::new(left_objects, start_index);
        let right = BvhNode::new(right_objects, start_index + mid);

        let bbox_union = Aabb::union(left.bounding_box(), right.bounding_box());

        BvhNode::Interior {
            bbox: bbox_union,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_ray: Interval) -> Option<HitRecord> {
        todo!()
    }

    fn bounding_box(&self) -> Aabb {
        match self {
            BvhNode::Leaf {
                bbox,
                object_offset: _,
            } => *bbox,
            BvhNode::Interior {
                bbox,
                left: _,
                right: _,
            } => *bbox,
        }
    }
}

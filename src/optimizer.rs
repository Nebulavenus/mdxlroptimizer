use crate::traits::{InRangeFrames, OptimizeFrames};
use nebula_mdx::chunks::BytesTotalSize;
use nebula_mdx::MDLXModel;
use std::ops::RangeInclusive;

pub fn optimize_model(model: &mut MDLXModel, threshold: f32, linearize: bool, outside: bool) {
    let mut special_frames = Vec::<u32>::new();
    let mut anim_frame_ranges = Vec::<RangeInclusive<u32>>::new();
    let sequences = model.sequence_chunk.as_ref().unwrap();
    for anim in &sequences.data {
        special_frames.push(anim.interval_start);
        special_frames.push(anim.interval_end);
        anim_frame_ranges.push(anim.interval_start..=anim.interval_end);
    }

    if model.bone_chunk.is_some() {
        let bones = model.bone_chunk.as_mut().unwrap();
        for bone in bones.data.iter_mut() {
            if bone.node.translation.is_some() {
                let geoset = bone.node.translation.as_mut().unwrap();
                if outside {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                } else {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                    geoset.optimize(special_frames.clone(), threshold, linearize);
                }
            }
            if bone.node.rotation.is_some() {
                let geoset = bone.node.rotation.as_mut().unwrap();
                if outside {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                } else {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                    geoset.optimize(special_frames.clone(), threshold, linearize);
                }
            }
            if bone.node.scaling.is_some() {
                let geoset = bone.node.scaling.as_mut().unwrap();
                if outside {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                } else {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                    geoset.optimize(special_frames.clone(), threshold, linearize);
                }
            }

            bone.node.inclusive_size = bone.node.total_bytes_size() as u32;
        }
    }

    if model.helper_chunk.is_some() {
        let helpers = model.helper_chunk.as_mut().unwrap();
        for helper in helpers.data.iter_mut() {
            if helper.node.translation.is_some() {
                let geoset = helper.node.translation.as_mut().unwrap();
                if outside {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                } else {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                    geoset.optimize(special_frames.clone(), threshold, linearize);
                }
            }
            if helper.node.rotation.is_some() {
                let geoset = helper.node.rotation.as_mut().unwrap();
                if outside {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                } else {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                    geoset.optimize(special_frames.clone(), threshold, linearize);
                }
            }
            if helper.node.scaling.is_some() {
                let geoset = helper.node.scaling.as_mut().unwrap();
                if outside {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                } else {
                    geoset.in_range_frames(anim_frame_ranges.clone());
                    geoset.optimize(special_frames.clone(), threshold, linearize);
                }
            }

            helper.node.inclusive_size = helper.node.total_bytes_size() as u32;
        }
    }
}

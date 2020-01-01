use std::ops::RangeInclusive;
use nebula_mdx::MDLXModel;
use nebula_mdx::chunks::BytesTotalSize;
use crate::macros::*;

pub fn optimize_model(model: &mut MDLXModel, threshold: f32, linearize: bool, _outside: bool) {
    let mut special_frames = Vec::<u32>::new();
    let mut anim_frame_ranges = Vec::<RangeInclusive<u32>>::new();
    let sequences = model.sequence_chunk.as_ref().unwrap();
    for anim in &sequences.data {
        special_frames.push(anim.interval_start);
        special_frames.push(anim.interval_end);
        anim_frame_ranges.push(anim.interval_start..=anim.interval_end);
    }

    /*
    let key = geoset_track.time;
    let frame_in_range = anim_frame_ranges
        .iter()
        .any(|range| range.contains(&key));
    */

    if model.bone_chunk.is_some() {
        let bones = model.bone_chunk.as_mut().unwrap();
        for bone in bones.data.iter_mut() {
            if bone.node.geoset_translation.is_some() {
                let geoset = bone.node.geoset_translation.as_mut().unwrap();
                geoset.optimize(special_frames.clone(), threshold, linearize);
            }
            if bone.node.geoset_rotation.is_some() {
                let geoset = bone.node.geoset_rotation.as_mut().unwrap();
                geoset.optimize(special_frames.clone(), threshold, linearize);
            }
            if bone.node.geoset_scaling.is_some() {
                let geoset = bone.node.geoset_scaling.as_mut().unwrap();
                geoset.optimize(special_frames.clone(), threshold, linearize);
            }

            bone.node.inclusive_size = bone.node.total_bytes_size() as u32;
        }
    }

    if model.helper_chunk.is_some() {
        let helpers = model.helper_chunk.as_mut().unwrap();
        for helper in helpers.data.iter_mut() {
            if helper.node.geoset_translation.is_some() {
                let geoset = helper.node.geoset_translation.as_mut().unwrap();
                geoset.optimize(special_frames.clone(), threshold, linearize);
            }
            if helper.node.geoset_rotation.is_some() {
                let geoset = helper.node.geoset_rotation.as_mut().unwrap();
                geoset.optimize(special_frames.clone(), threshold, linearize);
            }
            if helper.node.geoset_scaling.is_some() {
                let geoset = helper.node.geoset_scaling.as_mut().unwrap();
                geoset.optimize(special_frames.clone(), threshold, linearize);
            }

            helper.node.inclusive_size = helper.node.total_bytes_size() as u32;
        }
    }
}

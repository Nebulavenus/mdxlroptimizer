use nebula_mdx::chunks::{
    GeosetRotation, GeosetRotationTrack, GeosetScaling, GeosetScalingTrack, GeosetTranslation,
    GeosetTranslationTrack,
};
use std::ops::RangeInclusive;

pub trait CompareValues {
    fn compare_values(&self, other: &Self, threshold: f32) -> bool;
}

macro_rules! compare_values_impl {
    ($name:ident) => {
        impl CompareValues for $name {
            fn compare_values(&self, other: &Self, threshold: f32) -> bool {
                let mut result = true;
                for (idx, value) in self.value.iter().enumerate() {
                    let other_value = other.value[idx];

                    let diff = value - other_value;

                    if diff < -threshold || diff > threshold {
                        result = false;
                    }
                }
                result
            }
        }
    };
}

compare_values_impl!(GeosetTranslationTrack);
compare_values_impl!(GeosetRotationTrack);
compare_values_impl!(GeosetScalingTrack);

pub trait OptimizeFrames {
    fn optimize(&mut self, special_frames: Vec<u32>, threshold: f32, linearize: bool);
}

macro_rules! optimize_frames_impl {
    ($name:ident) => {
        impl OptimizeFrames for $name {
            fn optimize(&mut self, special_frames: Vec<u32>, threshold: f32, linearize: bool) {
                if self.interpolation_type > 1 && linearize {
                    // Set to linear
                    self.interpolation_type = 1;

                    for track in self.data.iter_mut() {
                        track.in_tan = None;
                        track.out_tan = None;
                    }
                }

                if self.data.len() > 2 {
                    let mut result = Vec::new();

                    result.push(self.data.first().unwrap().clone());

                    for idx in 1..self.data.len() - 1 {
                        let first_track = self.data[idx - 1].clone();
                        let second_track = self.data[idx].clone();
                        let third_track = self.data[idx + 1].clone();

                        if special_frames.contains(&second_track.time)
                            || !first_track.compare_values(&second_track, threshold)
                            || !second_track.compare_values(&third_track, threshold)
                        {
                            result.push(second_track);
                        }
                    }

                    result.push(self.data.last().unwrap().clone());

                    self.number_of_tracks = result.len() as u32;
                    self.data = result;
                }
            }
        }
    };
}

optimize_frames_impl!(GeosetTranslation);
optimize_frames_impl!(GeosetRotation);
optimize_frames_impl!(GeosetScaling);

pub trait InRangeFrames {
    fn in_range_frames(&mut self, anim_frame_ranges: Vec<RangeInclusive<u32>>);
}

macro_rules! in_range_frames_impl {
    ($name:ident) => {
        impl InRangeFrames for $name {
            fn in_range_frames(&mut self, anim_frame_ranges: Vec<RangeInclusive<u32>>) {
                if !self.data.is_empty() {
                    let mut result = Vec::new();

                    for track in self.data.iter() {
                        let key = track.time;
                        let frame_in_range =
                            anim_frame_ranges.iter().any(|range| range.contains(&key));

                        if frame_in_range {
                            result.push(track.clone());
                        }
                    }

                    self.number_of_tracks = result.len() as u32;
                    self.data = result;
                }
            }
        }
    };
}

in_range_frames_impl!(GeosetTranslation);
in_range_frames_impl!(GeosetRotation);
in_range_frames_impl!(GeosetScaling);

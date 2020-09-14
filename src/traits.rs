use nebula_mdx::chunks::{Track, Transform, Vec3, Vec4};
use std::ops::RangeInclusive;

pub trait CompareValues {
    fn compare_values(&self, other: &Self, threshold: f32) -> bool;
}

impl CompareValues for Track<Vec3> {
    fn compare_values(&self, other: &Self, threshold: f32) -> bool {
        let mut result = true;

        let first = &self.value;
        let second = &other.value;
        let xdf = first.x - second.x;
        let ydf = first.y - second.y;
        let zdf = first.z - second.z;
        if xdf < -threshold || xdf > threshold {
            result = false;
        } else if ydf < -threshold || ydf > threshold {
            result = false;
        } else if zdf < -threshold || zdf > threshold {
            result = false;
        }

        result
    }
}

impl CompareValues for Track<Vec4> {
    fn compare_values(&self, other: &Self, threshold: f32) -> bool {
        let mut result = true;

        let first = &self.value;
        let second = &other.value;
        let xdf = first.x - second.x;
        let ydf = first.y - second.y;
        let zdf = first.z - second.z;
        let wdf = first.w - second.w;
        if xdf < -threshold || xdf > threshold {
            result = false;
        } else if ydf < -threshold || ydf > threshold {
            result = false;
        } else if zdf < -threshold || zdf > threshold {
            result = false;
        } else if wdf < -threshold || wdf > threshold {
            result = false;
        }

        result
    }
}

pub trait OptimizeFrames {
    fn optimize(&mut self, special_frames: Vec<u32>, threshold: f32, linearize: bool);
}

impl OptimizeFrames for Transform<Vec3> {
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
            let first = self.data.first().unwrap().clone();
            result.push(first);

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

            let last = self.data.last().unwrap().clone();
            result.push(last);

            self.number_of_tracks = result.len() as u32;
            self.data = result;
        }
    }
}

impl OptimizeFrames for Transform<Vec4> {
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
            let first = self.data.first().unwrap().clone();
            result.push(first);

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

            let last = self.data.last().unwrap().clone();
            result.push(last);

            self.number_of_tracks = result.len() as u32;
            self.data = result;
        }
    }
}

pub trait InRangeFrames {
    fn in_range_frames(&mut self, anim_frame_ranges: Vec<RangeInclusive<u32>>);
}

impl InRangeFrames for Transform<Vec3> {
    fn in_range_frames(&mut self, anim_frame_ranges: Vec<RangeInclusive<u32>>) {
        if !self.data.is_empty() {
            let mut result = Vec::new();

            for track in self.data.clone() {
                let key = track.time;
                let frame_in_range = anim_frame_ranges.iter().any(|range| range.contains(&key));

                if frame_in_range {
                    result.push(track);
                }
            }

            self.number_of_tracks = result.len() as u32;
            self.data = result;
        }
    }
}

impl InRangeFrames for Transform<Vec4> {
    fn in_range_frames(&mut self, anim_frame_ranges: Vec<RangeInclusive<u32>>) {
        if !self.data.is_empty() {
            let mut result = Vec::new();

            for track in self.data.clone() {
                let key = track.time;
                let frame_in_range = anim_frame_ranges.iter().any(|range| range.contains(&key));

                if frame_in_range {
                    result.push(track);
                }
            }

            self.number_of_tracks = result.len() as u32;
            self.data = result;
        }
    }
}

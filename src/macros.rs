use nebula_mdx::chunks::{
    GeosetTranslationTrack, GeosetRotationTrack, GeosetScalingTrack,
    GeosetTranslation, GeosetRotation, GeosetScaling,
};
use paste;

macro_rules! compare_values_impl {
    ($name:ident) => {
        paste::item! {
            pub struct [<Copy $name>]($name);

            impl From<$name> for [<Copy $name>] {
                fn from(from: $name) -> [<Copy $name>] {
                    [<Copy $name>](from)
                }
            }

            impl [<Copy $name>] {
                pub fn compare_values(&self, other: &Self, threshold: f32) -> bool {
                    let mut result = true;
                    for (idx, value) in self.0.value.iter().enumerate() {
                        let other_value = other.0.value[idx];

                        let diff = value - other_value;

                        if diff < -threshold || diff > threshold {
                            result = false;
                        }
                    }
                    result
                }
            }
        }
    };
}

compare_values_impl!(GeosetTranslationTrack);
compare_values_impl!(GeosetRotationTrack);
compare_values_impl!(GeosetScalingTrack);

macro_rules! optimize_impl {
    ($name:ident) => {
        paste::item! {
            pub struct [<Copy $name>]<'a>(&'a mut $name);

            impl<'a> From<&'a mut $name> for [<Copy $name>]<'a> {
                fn from(from: &'a mut $name) -> [<Copy $name>]<'a> {
                    [<Copy $name>](from)
                }
            }

            impl [<Copy $name>]<'_> {
                pub fn optimize(&mut self, special_frames: Vec<u32>, threshold: f32, linearize: bool) {
                    if self.0.interpolation_type > 1 && linearize {
                        // Set to linear
                        self.0.interpolation_type = 1;

                        for track in self.0.data.iter_mut() {
                            track.in_tan = None;
                            track.out_tan = None;
                        }
                    }

                    if self.0.data.len() > 2 {
                        let mut result = Vec::new();

                        result.push(self.0.data[0].clone());

                        for idx in 1..self.0.data.len() - 1 {
                            let first_track = [<Copy $name Track>]::from(self.0.data[idx - 1].clone());
                            let second_track = [<Copy $name Track>]::from(self.0.data[idx].clone());
                            let third_track = [<Copy $name Track>]::from(self.0.data[idx + 1].clone());

                            if   special_frames.contains(&second_track.0.time) ||
                                !first_track.compare_values(&second_track, threshold) ||
                                !second_track.compare_values(&third_track, threshold)
                            {
                                result.push(second_track.0);
                            }
                        }

                        result.push(self.0.data[self.0.data.len() - 1].clone());

                        self.0.number_of_tracks = result.len() as u32;
                        self.0.data = result;
                    }
                }
            }
        }
    };
}

optimize_impl!(GeosetTranslation);
optimize_impl!(GeosetRotation);
optimize_impl!(GeosetScaling);

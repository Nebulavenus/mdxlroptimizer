use scroll::{ctx, Pread, Pwrite, Endian};
use std::mem::size_of_val;

pub trait BytesTotalSize {
    fn total_bytes_size(&self) -> usize;
}

macro_rules! create_named_track {
    ($name:ident, $typ:ty, $size:expr) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $name {
            pub time: u32,
            pub value: [$typ; $size],
            pub in_tan: Option<[$typ; $size]>,
            pub out_tan: Option<[$typ; $size]>,
        }

        impl ctx::TryFromCtx<'_, Endian> for $name {
            type Error = scroll::Error;

            fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
                let offset = &mut 0;

                let time = src.gread_with::<u32>(offset, ctx)?;
                let mut value = [<$typ>::default(); $size];
                for id in 0..$size {
                    value[id] = src.gread_with::<$typ>(offset, ctx)?;
                }

                Ok(($name {
                    time,
                    value,
                    in_tan: None,
                    out_tan: None
                }, *offset))
            }
        }

        impl ctx::TryIntoCtx<Endian> for $name {
            type Error = scroll::Error;

            fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
                let offset = &mut 0;

                src.gwrite_with::<u32>(self.time, offset, ctx)?;
                for id in 0..$size {
                    src.gwrite_with::<$typ>(self.value[id], offset, ctx)?;
                }

                if self.in_tan.is_some() {
                    for id in 0..$size {
                        src.gwrite_with::<$typ>(self.in_tan.unwrap()[id], offset, ctx)?;
                    }
                }

                if self.out_tan.is_some() {
                    for id in 0..$size {
                        src.gwrite_with::<$typ>(self.out_tan.unwrap()[id], offset, ctx)?;
                    }
                }

                Ok(*offset)
            }
        }

        impl BytesTotalSize for $name {
            fn total_bytes_size(&self) -> usize {
                let mut result = 0usize;
                result += size_of_val(&self.time);
                result += size_of_val(&self.value);
                if self.in_tan.is_some() {
                    result += size_of_val(self.in_tan.as_ref().unwrap());
                }
                if self.out_tan.is_some() {
                    result += size_of_val(self.out_tan.as_ref().unwrap());
                }
                result
            }
        }

        impl $name {
            pub fn compare_values(&self, other: &Self, threshold: f32) -> bool {
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

macro_rules! create_named_translation {
   ($name:ident, $track:ty, $typ:ty, $size:expr) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $name {
            pub number_of_tracks: u32,
            pub interpolation_type: u32,
            pub global_sequence_id: u32,

            pub data: Vec<$track>,
        }

        impl ctx::TryFromCtx<'_, Endian> for $name {
            type Error = scroll::Error;

            fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
                let offset = &mut 0;

                let number_of_tracks = src.gread_with::<u32>(offset, ctx)?;
                let interpolation_type = src.gread_with::<u32>(offset, ctx)?;
                let global_sequence_id = src.gread_with::<u32>(offset, ctx)?;

                let mut data = Vec::new();
                for _ in 0..number_of_tracks {
                    let mut track = src.gread_with::<$track>(offset, ctx)?;
                    if interpolation_type > 1 {
                        let mut in_tan = [<$typ>::default(); $size];
                        for id in 0..$size {
                            in_tan[id] = src.gread_with::<$typ>(offset, ctx)?;
                        }
                        let mut out_tan = [<$typ>::default(); $size];
                        for id in 0..$size {
                            out_tan[id] = src.gread_with::<$typ>(offset, ctx)?;
                        }

                        track.in_tan = Some(in_tan);
                        track.out_tan = Some(out_tan);
                    }
                    data.push(track);
                }

                Ok(($name {
                    number_of_tracks,
                    interpolation_type,
                    global_sequence_id,
                    data,
                }, *offset))
            }
        }

        impl ctx::TryIntoCtx<Endian> for $name {
            type Error = scroll::Error;

            fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
                let offset = &mut 0;

                src.gwrite_with::<u32>(self.number_of_tracks, offset, ctx)?;
                src.gwrite_with::<u32>(self.interpolation_type, offset, ctx)?;
                src.gwrite_with::<u32>(self.global_sequence_id, offset, ctx)?;

                for track in self.data {
                    src.gwrite_with::<$track>(track, offset, ctx)?;
                }

                Ok(*offset)
            }
        }

        impl BytesTotalSize for $name {
            fn total_bytes_size(&self) -> usize {
                let mut result = 0usize;

                result += size_of_val(&self.number_of_tracks);
                result += size_of_val(&self.interpolation_type);
                result += size_of_val(&self.global_sequence_id);

                for track in &self.data {
                    result += track.total_bytes_size();
                }

                result
            }
        }

        impl $name {
            pub fn optimize(&mut self, special_frames: Vec<u32>, threshold: f32, linearize: bool) {

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

                    result.push(self.data[0].clone());

                    for idx in 1..self.data.len() - 1 {
                        let first_track = self.data[idx - 1].clone();
                        let second_track = self.data[idx].clone();
                        let third_track = self.data[idx + 1].clone();

                        if   special_frames.contains(&second_track.time) ||
                            !first_track.compare_values(&second_track, threshold) ||
                            !second_track.compare_values(&third_track, threshold)
                        {
                            result.push(second_track);
                        }
                    }

                    result.push(self.data[self.data.len() - 1].clone());

                    self.number_of_tracks = result.len() as u32;
                    self.data = result;
                }
            }
        }
    };
}

// KTAT - KTAR - KTAS
create_named_track!(TextureTranslationTrack, f32, 3);
create_named_track!(TextureRotationTrack, f32, 4);
create_named_track!(TextureScalingTrack, f32, 3);

create_named_translation!(TextureTranslation, TextureTranslationTrack, f32, 3);
create_named_translation!(TextureRotation, TextureRotationTrack, f32, 4);
create_named_translation!(TextureScaling, TextureScalingTrack, f32, 3);

// KGTR - KGRT - KGSC
create_named_track!(GeosetTranslationTrack, f32, 3);
create_named_track!(GeosetRotationTrack, f32, 4);
create_named_track!(GeosetScalingTrack, f32, 3);

create_named_translation!(GeosetTranslation, GeosetTranslationTrack, f32, 3);
create_named_translation!(GeosetRotation, GeosetRotationTrack, f32, 4);
create_named_translation!(GeosetScaling, GeosetScalingTrack, f32, 3);

// KGAO - KGAC
create_named_track!(GeosetAlphaTrack, f32, 1);
create_named_track!(GeosetColorTrack, f32, 3);
create_named_translation!(GeosetAlpha, GeosetAlphaTrack, f32, 1);
create_named_translation!(GeosetColor, GeosetColorTrack, f32, 3);

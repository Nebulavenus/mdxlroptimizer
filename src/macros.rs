use scroll::{ctx, Pread, Pwrite, LE, Endian};

macro_rules! create_named_track {
    ($name:ident, $typ:ty, $size:expr) => {
        #[derive(PartialEq, Debug)]
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
    };
}

macro_rules! create_named_translation {
   ($name:ident, $track:ty, $typ:ty, $size:expr) => {
        #[derive(PartialEq, Debug)]
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

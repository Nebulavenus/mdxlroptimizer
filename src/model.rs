use scroll::{ctx, Pread, Pwrite, LE, Endian};

const MDLX_TAG: u32 = 1481393229;

const VERS_TAG: u32 = 1397900630;
const MODL_TAG: u32 = 1279545165;
const SEQS_TAG: u32 = 1397835091;
const TEXS_TAG: u32 = 1398293844;
const PIVT_TAG: u32 = 1414941008;

pub fn read_mdx_file(data: Vec<u8>) {
    let offset = &mut 0usize;
    let mdlx_tag = data.gread_with::<u32>(offset, LE).unwrap();
    if mdlx_tag == MDLX_TAG {
        // Iterate over chunks
        while *offset < data.len() {
            dbg!(&offset);

            // For debug
            let mut tag_offset = offset.clone();
            let tag_buffer
                = (0..4).map(|_| data.gread::<u8>(&mut tag_offset).unwrap()).collect::<Vec<u8>>();
            let tag_name = String::from_utf8(tag_buffer).unwrap_or("NOTAG".to_string());

            let tag = data.gread_with::<u32>(offset, LE).unwrap();
            dbg!(format!("{:X}", &tag));
            dbg!(&tag);
            dbg!(&tag_name);
            handle_tag(tag, &data, offset).unwrap();
        }
    } else {
        // Error, not mdlx
    }
}

fn handle_tag(tag: u32, data: &[u8], offset: &mut usize) -> Result<(), scroll::Error> {
    match tag {
        VERS_TAG => {
            let version_chunk = data.gread_with::<VersionChunk>(offset, LE)?;
            dbg!(version_chunk);
        },
        MODL_TAG => {
            let model_chunk = data.gread_with::<ModelChunk>(offset, LE)?;
            dbg!(model_chunk);
        },
        SEQS_TAG => {
            let sequence_chunk = data.gread_with::<SequenceChunk>(offset, LE)?;
            dbg!(sequence_chunk);
        },
        TEXS_TAG => {
            let texture_chunk = data.gread_with::<TextureChunk>(offset, LE)?;
            dbg!(texture_chunk);
        },
        PIVT_TAG => {
            let pivot_chunk = data.gread_with::<PivotPointChunk>(offset, LE)?;
            dbg!(pivot_chunk);
        },
        _ => {
            //let chunk_size = data.gread_with::<u32>(offset, LE)?;
            //*offset += chunk_size as usize;
            //*offset += 4;
        },
    }
    Ok(())
}

#[derive(PartialEq, Debug)]
pub struct VersionChunk {
    pub chunk_size: u32,

    pub version: u32,
}

impl ctx::TryFromCtx<'_, Endian> for VersionChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;
        let version = src.gread_with::<u32>(offset, ctx)?;
        Ok((VersionChunk { chunk_size, version }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct ModelChunk {
    pub chunk_size: u32,

    pub name: String,
    pub unknown: u32,
    pub bounds_radius: f32,
    pub minimum_extent: [f32; 3],
    pub maximum_extent: [f32; 3],
    pub blend_time: u32,
}

impl ctx::TryFromCtx<'_, Endian> for ModelChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        // Name has fixed size -- blizzard why?
        let max_name_len = 336usize;
        let name = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let unknown = src.gread_with::<u32>(offset, ctx)?;
        let bounds_radius = src.gread_with::<f32>(offset, ctx)?;
        let minimum_extent = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];
        let maximum_extent = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];
        let blend_time = src.gread_with::<u32>(offset, ctx)?;
        Ok((ModelChunk {
            chunk_size,
            name,
            unknown,
            bounds_radius,
            minimum_extent,
            maximum_extent,
            blend_time,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct SequenceChunk {
    pub chunk_size: u32,

    // chunk_size / 132
    pub data: Vec<Sequence>,
}

impl ctx::TryFromCtx<'_, Endian> for SequenceChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        if let Some(sequence_count) = u32::checked_div(chunk_size.clone(), 132) {
            for _ in 0..sequence_count {
                let sequence = src.gread_with::<Sequence>(offset, ctx)?;
                data.push(sequence);
            }
        }

        Ok((SequenceChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct Sequence {
    pub name: String,
    pub interval_start: u32,
    pub interval_end: u32,
    pub move_speed: f32,
    pub non_looping: u32,
    pub rarity: f32,
    pub unknown: u32,
    pub bounds_radius: f32,
    pub minimum_extent: [f32; 3],
    pub maximum_extent: [f32; 3],
}

impl ctx::TryFromCtx<'_, Endian> for Sequence {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        // Name has fixed size
        let max_name_len = 80usize;
        let name = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let interval_start = src.gread_with::<u32>(offset, ctx)?;
        let interval_end = src.gread_with::<u32>(offset, ctx)?;
        let move_speed = src.gread_with::<f32>(offset, ctx)?;
        let non_looping = src.gread_with::<u32>(offset, ctx)?;
        let rarity = src.gread_with::<f32>(offset, ctx)?;
        let unknown = src.gread_with::<u32>(offset, ctx)?;
        let bounds_radius = src.gread_with::<f32>(offset, ctx)?;
        let minimum_extent = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];
        let maximum_extent = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];

        Ok((Sequence {
            name,
            interval_start,
            interval_end,
            move_speed,
            non_looping,
            rarity,
            unknown,
            bounds_radius,
            minimum_extent,
            maximum_extent,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct GlobalSequenceChunk {
    pub glbs: u32,
    pub chunk_size: u32,

    // chunk_size / 4
    pub data: Vec<GlobalSequence>,
}

#[derive(PartialEq, Debug)]
pub struct GlobalSequence {
    pub duration: u32,
}

#[derive(PartialEq, Debug)]
pub struct TextureChunk {
    pub chunk_size: u32,

    // chunk_size / 268
    pub data: Vec<Texture>,
}

impl ctx::TryFromCtx<'_, Endian> for TextureChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        if let Some(texture_count) = u32::checked_div(chunk_size.clone(), 268) {
            for _ in 0..texture_count {
                let texture = src.gread_with::<Texture>(offset, ctx)?;
                data.push(texture);
            }
        }

        Ok((TextureChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct Texture {
    pub replaceable_id: u32,
    pub file_name: String,
    pub unknown: u32,
    pub flags: u32,
}

impl ctx::TryFromCtx<'_, Endian> for Texture {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let replaceable_id = src.gread_with::<u32>(offset, ctx)?;

        // Name has fixed size
        let max_name_len = 256usize;
        let file_name = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let unknown = src.gread_with::<u32>(offset, ctx)?;
        let flags = src.gread_with::<u32>(offset, ctx)?;

        Ok((Texture {
            replaceable_id,
            file_name,
            unknown,
            flags,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct TextureAnimationChunk {
    pub txan: u32,
    pub chunk_size: u32,

    pub data: Vec<TextureAnimation>,
}

#[derive(PartialEq, Debug)]
pub struct TextureAnimation {
    pub inclusive_size: u32,
}

#[derive(PartialEq, Debug)]
pub struct TextureTranslation {
    pub ktat: u32,

    pub number_of_tracks: u32,
    pub interpolation_type: u32,
    pub global_sequence_id: u32,

    // number_of_tracks
    pub data: Vec<TranslationTrack>,
}

#[derive(PartialEq, Debug)]
pub struct TranslationTrack {
    pub time: u32,
    pub translation: [f32; 3],
    pub in_tan: Option<[f32; 3]>,
    pub out_tan: Option<[f32; 3]>,
}

#[derive(PartialEq, Debug)]
pub struct TextureRotation {
    pub ktar: u32,

    pub number_of_tracks: u32,
    pub interpolation_type: u32,
    pub global_sequence_id: u32,

    // number_of_tracks
    pub data: Vec<RotationTrack>,
}

#[derive(PartialEq, Debug)]
pub struct RotationTrack {
    pub time: u32,
    pub rotation: [f32; 4],
    pub in_tan: Option<[f32; 4]>,
    pub out_tan: Option<[f32; 4]>,
}

#[derive(PartialEq, Debug)]
pub struct TextureScaling {
    pub ktas: u32,

    pub number_of_tracks: u32,
    pub interpolation_type: u32,
    pub global_sequence_id: u32,

    // number_of_tracks
    pub data: Vec<ScalingTrack>,
}

#[derive(PartialEq, Debug)]
pub struct ScalingTrack {
    pub time: u32,
    pub translation: [f32; 3],
    pub in_tan: Option<[f32; 3]>,
    pub out_tan: Option<[f32; 3]>,
}

#[derive(PartialEq, Debug)]
pub struct GeosetChunk {
    pub geos: u32,
    pub chunk_size: u32,
}

#[derive(PartialEq, Debug)]
pub struct PivotPointChunk {
    pub chunk_size: u32,

    // chunk_size / 12
    pub data: Vec<PivotPoint>,
}

impl ctx::TryFromCtx<'_, Endian> for PivotPointChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        if let Some(pivot_point_count) = u32::checked_div(chunk_size.clone(), 12) {
            for _ in 0..pivot_point_count {
                let pivot_point = src.gread_with::<PivotPoint>(offset, ctx)?;
                data.push(pivot_point);
            }
        }

        Ok((PivotPointChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct PivotPoint {
    pub position: [f32; 3],
}

impl ctx::TryFromCtx<'_, Endian> for PivotPoint {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let position = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];

        Ok((PivotPoint {
            position,
        }, *offset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_mdx_file_api() {
        let raw_data = fs::read("testfiles/base_model.mdx").unwrap();
        dbg!(&raw_data.len());
        read_mdx_file(raw_data);
    }

}
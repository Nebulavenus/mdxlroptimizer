use scroll::{ctx, Pread, Pwrite, LE, Endian};
use crate::macros::{
    TextureTranslation, TextureRotation, TextureScaling,
    GeosetColor, GeosetAlpha,
    GeosetTranslation, GeosetScaling, GeosetRotation,
};

const MDLX_TAG: u32 = 1481393229;

const VERS_TAG: u32 = 1397900630;
const MODL_TAG: u32 = 1279545165;
const SEQS_TAG: u32 = 1397835091;
const GLBS_TAG: u32 = 1396853831;
const TEXS_TAG: u32 = 1398293844;
const TXAN_TAG: u32 = 1312905300;
const GEOS_TAG: u32 = 1397704007;
const GEOA_TAG: u32 = 1095714119;
const BONE_TAG: u32 = 1162760002;
const PIVT_TAG: u32 = 1414941008;

const KTAT_TAG: u32 = 1413567563;
const KTAR_TAG: u32 = 1380013131;
const KTAS_TAG: u32 = 1396790347;

const KGTR_TAG: u32 = 1381254987;
const KGRT_TAG: u32 = 1414678347;
const KGSC_TAG: u32 = 1129531211;

const KGAO_TAG: u32 = 1329678155;
const KGAC_TAG: u32 = 1128351563;

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
            //dbg!(version_chunk);
        },
        MODL_TAG => {
            let model_chunk = data.gread_with::<ModelChunk>(offset, LE)?;
            //dbg!(model_chunk);
        },
        SEQS_TAG => {
            let sequence_chunk = data.gread_with::<SequenceChunk>(offset, LE)?;
            //dbg!(sequence_chunk);
        },
        GLBS_TAG => {
            let global_sequence_chunk = data.gread_with::<GlobalSequenceChunk>(offset, LE)?;
            //dbg!(global_sequence_chunk);
        }
        TEXS_TAG => {
            let texture_chunk = data.gread_with::<TextureChunk>(offset, LE)?;
            //dbg!(texture_chunk);
        },
        TXAN_TAG => {
            let texture_animation_chunk = data.gread_with::<TextureAnimationChunk>(offset, LE)?;
            //dbg!(texture_animation_chunk);
        },
        GEOS_TAG => {
            let geoset_chunk = data.gread_with::<GeosetChunk>(offset, LE)?;
            //dbg!(geoset_chunk);
        },
        GEOA_TAG => {
            let geoset_animation_chunk = data.gread_with::<GeosetAnimationChunk>(offset, LE)?;
            //dbg!(geoset_animation_chunk);
        },
        BONE_TAG => {
            let bone_chunk = data.gread_with::<BoneChunk>(offset, LE)?;
            //dbg!(bone_chunk);
        },
        PIVT_TAG => {
            let pivot_chunk = data.gread_with::<PivotPointChunk>(offset, LE)?;
            //dbg!(pivot_chunk);
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
    pub chunk_size: u32,

    // chunk_size / 4
    pub data: Vec<GlobalSequence>,
}

impl ctx::TryFromCtx<'_, Endian> for GlobalSequenceChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        if let Some(sequence_count) = u32::checked_div(chunk_size.clone(), 4) {
            for _ in 0..sequence_count {
                let sequence = src.gread_with::<GlobalSequence>(offset, ctx)?;
                data.push(sequence);
            }
        }

        Ok((GlobalSequenceChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct GlobalSequence {
    pub duration: u32,
}

impl ctx::TryFromCtx<'_, Endian> for GlobalSequence {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let duration = src.gread_with::<u32>(offset, ctx)?;

        Ok((GlobalSequence {
            duration,
        }, *offset))
    }
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
    pub chunk_size: u32,

    pub data: Vec<TextureAnimation>,
}

impl ctx::TryFromCtx<'_, Endian> for TextureAnimationChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let texture_animation = src.gread_with::<TextureAnimation>(offset, ctx)?;
            total_size += texture_animation.inclusive_size;
            data.push(texture_animation);
        }

        Ok((TextureAnimationChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct TextureAnimation {
    pub inclusive_size: u32,

    pub texture_translation: Option<TextureTranslation>,
    pub texture_rotation: Option<TextureRotation>,
    pub texture_scaling: Option<TextureScaling>,
}

impl ctx::TryFromCtx<'_, Endian> for TextureAnimation {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx).unwrap();
        let mut texture_animation = TextureAnimation {
            inclusive_size,
            texture_translation: None,
            texture_rotation: None,
            texture_scaling: None
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();
            dbg!(format!("{:X}", &tag));
            dbg!(&tag);

            match tag {
                KTAT_TAG => {
                    let ktat = src.gread_with::<TextureTranslation>(offset, ctx)?;
                    //dbg!(&ktat);
                    texture_animation.texture_translation = Some(ktat);
                },
                KTAR_TAG => {
                    let ktar = src.gread_with::<TextureRotation>(offset, ctx)?;
                    //dbg!(&ktar);
                    texture_animation.texture_rotation = Some(ktar);
                },
                KTAS_TAG => {
                    let ktas = src.gread_with::<TextureScaling>(offset, ctx)?;
                    //dbg!(&ktas);
                    texture_animation.texture_scaling = Some(ktas);
                },
                _ => unreachable!(),
            }
        }

        Ok((texture_animation, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct GeosetChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for GeosetChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((GeosetChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct GeosetAnimationChunk {
    pub chunk_size: u32,

    pub data: Vec<GeosetAnimation>,
}

impl ctx::TryFromCtx<'_, Endian> for GeosetAnimationChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let geoset_animation = src.gread_with::<GeosetAnimation>(offset, ctx)?;
            total_size += geoset_animation.inclusive_size;
            data.push(geoset_animation);
        }

        Ok((GeosetAnimationChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct GeosetAnimation {
    pub inclusive_size: u32,

    pub alpha: f32,
    pub flags: u32,
    pub color: [f32; 3], // bgr
    pub geoset_id: u32,

    pub geoset_alpha: Option<GeosetAlpha>,
    pub geoset_color: Option<GeosetColor>,
}


impl ctx::TryFromCtx<'_, Endian> for GeosetAnimation {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx)?;
        let alpha = src.gread_with::<f32>(offset, ctx)?;
        let flags = src.gread_with::<u32>(offset, ctx)?;
        let color = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];
        let geoset_id = src.gread_with::<u32>(offset, ctx)?;

        let mut geoset_animation = GeosetAnimation {
            inclusive_size,
            alpha,
            flags,
            color,
            geoset_id,
            geoset_alpha: None,
            geoset_color: None
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();
            dbg!(format!("{:X}", &tag));
            dbg!(&tag);

            match tag {
                KGAO_TAG => {
                    let geoset_alpha = src.gread_with::<GeosetAlpha>(offset, ctx)?;
                    geoset_animation.geoset_alpha = Some(geoset_alpha);
                },
                KGAC_TAG => {
                    let geoset_color = src.gread_with::<GeosetColor>(offset, ctx)?;
                    geoset_animation.geoset_color = Some(geoset_color);
                },
                _ => unreachable!(),
            }
       }

        Ok((geoset_animation, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct BoneChunk {
    pub chunk_size: u32,

    pub data: Vec<Bone>,
}

impl ctx::TryFromCtx<'_, Endian> for BoneChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let bone = src.gread_with::<Bone>(offset, ctx)?;
            // Node inclusive_size + two u32 inside bone struct
            total_size += bone.node.inclusive_size + 4 + 4;
            data.push(bone);
        }

        Ok((BoneChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct Bone {
    pub node: Node,
    pub geoset_id: u32,
    pub geoset_animation_id: u32,
}

impl ctx::TryFromCtx<'_, Endian> for Bone {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let node = src.gread_with::<Node>(offset, ctx)?;
        let geoset_id = src.gread_with::<u32>(offset, ctx)?;
        let geoset_animation_id = src.gread_with::<u32>(offset, ctx)?;

        Ok((Bone {
            node,
            geoset_id,
            geoset_animation_id,
        }, *offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct Node {
    pub inclusive_size: u32,

    // max length 80
    pub name: String,
    pub object_id: u32,
    pub parent_id: u32,
    pub flags: u32,

    pub geoset_translation: Option<GeosetTranslation>,
    pub geoset_rotation: Option<GeosetRotation>,
    pub geoset_scaling: Option<GeosetScaling>,
}

impl ctx::TryFromCtx<'_, Endian> for Node {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx)?;

        // Name has fixed size
        let max_name_len = 80usize;
        let name = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let object_id = src.gread_with::<u32>(offset, ctx)?;
        let parent_id = src.gread_with::<u32>(offset, ctx)?;
        let flags = src.gread_with::<u32>(offset, ctx)?;

        let mut node = Node {
            inclusive_size,
            name,
            object_id,
            parent_id,
            flags,
            geoset_translation: None,
            geoset_rotation: None,
            geoset_scaling: None,
        };

        while (*offset as u32) < inclusive_size {

            let tag = src.gread_with::<u32>(offset, LE).unwrap();
            dbg!(format!("{:X}", &tag));
            dbg!(&tag);

            match tag {
                KGTR_TAG => {
                    let geoset_translation = src.gread_with::<GeosetTranslation>(offset, ctx)?;
                    node.geoset_translation = Some(geoset_translation);
                },
                KGRT_TAG => {
                    let geoset_rotation = src.gread_with::<GeosetRotation>(offset, ctx)?;
                    node.geoset_rotation = Some(geoset_rotation);
                },
                KGSC_TAG => {
                    let geoset_scaling = src.gread_with::<GeosetScaling>(offset, ctx)?;
                    node.geoset_scaling = Some(geoset_scaling);
                },
                _ => unreachable!(),
            }
        }

        Ok((node, *offset))
    }
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
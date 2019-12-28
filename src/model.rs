use scroll::{ctx, Pread, Pwrite, LE, Endian};
use crate::macros::{
    TextureTranslation, TextureRotation, TextureScaling,
    GeosetColor, GeosetAlpha, BytesTotalSize,
    GeosetTranslation, GeosetScaling, GeosetRotation,
};
use std::mem::size_of_val;

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
const MTLS_TAG: u32 = 1397511245;
const LITE_TAG: u32 = 1163151692;
const HELP_TAG: u32 = 1347175752;
const ATCH_TAG: u32 = 1212372033;
const PIVT_TAG: u32 = 1414941008;
const PREM_TAG: u32 = 1296388688;
const PRE2_TAG: u32 = 843403856;
const RIBB_TAG: u32 = 1111640402;
const EVTS_TAG: u32 = 1398036037;
const CAMS_TAG: u32 = 1397571907;
const CLID_TAG: u32 = 1145654339;

const KTAT_TAG: u32 = 1413567563;
const KTAR_TAG: u32 = 1380013131;
const KTAS_TAG: u32 = 1396790347;

const KGTR_TAG: u32 = 1381254987;
const KGRT_TAG: u32 = 1414678347;
const KGSC_TAG: u32 = 1129531211;

const KGAO_TAG: u32 = 1329678155;
const KGAC_TAG: u32 = 1128351563;

#[derive(PartialEq, Debug, Default)]
pub struct MDLXModel {
    pub version_chunk: Option<VersionChunk>,
    pub model_chunk: Option<ModelChunk>,
    pub sequence_chunk: Option<SequenceChunk>,
    pub global_sequence_chunk: Option<GlobalSequenceChunk>,
    pub texture_chunk: Option<TextureChunk>,
    pub texture_animation_chunk: Option<TextureAnimationChunk>,
    pub geoset_chunk: Option<GeosetChunk>,
    pub geoset_animation_chunk: Option<GeosetAnimationChunk>,
    pub bone_chunk: Option<BoneChunk>,
    pub light_chunk: Option<LightChunk>,
    pub helper_chunk: Option<HelperChunk>,
    pub attachment_chunk: Option<AttachmentChunk>,
    pub pivot_point_chunk: Option<PivotPointChunk>,
    pub particle_emitter_chunk: Option<ParticleEmitterChunk>,
    pub particle_emitter2_chunk: Option<ParticleEmitter2Chunk>,
    pub ribbon_emitter_chunk: Option<RibbonEmitterChunk>,
    pub event_object_chunk: Option<EventObjectChunk>,
    pub camera_chunk: Option<CameraChunk>,
    pub collision_shape_chunk: Option<CollisionShapeChunk>,
    pub material_chunk: Option<MaterialChunk>,
}

macro_rules! calculate_chunk_size_impl {
    ($name:ident) => {
        impl $name {
            // Chunk size is a struct size without chunk_size itself.
            pub fn calculate_chunk_size(&mut self) {
                //dbg!($name);
                dbg!(&self.chunk_size);
                self.chunk_size = self.total_bytes_size() as u32 - 4;
                dbg!(&self.chunk_size);
            }
        }
    };
}

calculate_chunk_size_impl!(VersionChunk);
calculate_chunk_size_impl!(ModelChunk);
calculate_chunk_size_impl!(SequenceChunk);
calculate_chunk_size_impl!(GlobalSequenceChunk);
calculate_chunk_size_impl!(TextureChunk);
calculate_chunk_size_impl!(TextureAnimationChunk);
calculate_chunk_size_impl!(GeosetChunk);
calculate_chunk_size_impl!(GeosetAnimationChunk);
calculate_chunk_size_impl!(BoneChunk);
calculate_chunk_size_impl!(LightChunk);
calculate_chunk_size_impl!(HelperChunk);
calculate_chunk_size_impl!(AttachmentChunk);
calculate_chunk_size_impl!(PivotPointChunk);
calculate_chunk_size_impl!(ParticleEmitterChunk);
calculate_chunk_size_impl!(ParticleEmitter2Chunk);
calculate_chunk_size_impl!(RibbonEmitterChunk);
calculate_chunk_size_impl!(EventObjectChunk);
calculate_chunk_size_impl!(CameraChunk);
calculate_chunk_size_impl!(CollisionShapeChunk);
calculate_chunk_size_impl!(MaterialChunk);

impl MDLXModel {
    pub fn read_mdx_file(data: Vec<u8>) -> Result<MDLXModel, scroll::Error> {
        let offset = &mut 0usize;
        let mdlx_tag = data.gread_with::<u32>(offset, LE)?;
        if mdlx_tag == MDLX_TAG {
            let mut result = MDLXModel::default();

            // Iterate over chunks
            while *offset < data.len() {
                dbg!(&offset);

                // For debug
                let mut tag_offset = offset.clone();
                let tag_buffer
                    = (0..4).map(|_| data.gread::<u8>(&mut tag_offset).unwrap()).collect::<Vec<u8>>();
                let tag_name = String::from_utf8(tag_buffer).unwrap_or("NOTAG".to_string());

                let tag = data.gread_with::<u32>(offset, LE)?;
                dbg!(format!("{:X}", &tag));
                dbg!(&tag);
                dbg!(&tag_name);

                result.handle_tag(tag, &data, offset)?;
            }

            Ok(result)
        } else {
            Err(scroll::Error::Custom("Not correct MDLX file".to_string()))
        }
    }

    pub fn write_mdx_file(mut model: MDLXModel) -> Result<Vec<u8>, scroll::Error> {
        // Get total size of mdx file
        model.correct_chunk_size();
        let total_size = model.model_total_size();

        // Create vec with capacity and set it len to total size
        let mut data = Vec::<u8>::with_capacity(total_size);
        unsafe {
            data.set_len(total_size);
        }

        // Begin to write fields
        let offset = &mut 0usize;

        data.gwrite_with::<u32>(MDLX_TAG, offset, LE)?;

        if model.version_chunk.is_some() {
            data.gwrite_with::<u32>(VERS_TAG, offset, LE)?;
            data.gwrite_with::<VersionChunk>(model.version_chunk.unwrap(), offset, LE)?;
        }
        if model.model_chunk.is_some() {
            data.gwrite_with::<u32>(MODL_TAG, offset, LE)?;
            data.gwrite_with::<ModelChunk>(model.model_chunk.unwrap(), offset, LE)?;
        }
        if model.sequence_chunk.is_some() {
            data.gwrite_with::<u32>(SEQS_TAG, offset, LE)?;
            data.gwrite_with::<SequenceChunk>(model.sequence_chunk.unwrap(), offset, LE)?;
        }
        if model.global_sequence_chunk.is_some() {
            data.gwrite_with::<u32>(GLBS_TAG, offset, LE)?;
            data.gwrite_with::<GlobalSequenceChunk>(model.global_sequence_chunk.unwrap(), offset, LE)?;
        }
        if model.texture_chunk.is_some() {
            data.gwrite_with::<u32>(TEXS_TAG, offset, LE)?;
            data.gwrite_with::<TextureChunk>(model.texture_chunk.unwrap(), offset, LE)?;
        }
        if model.texture_animation_chunk.is_some() {
            data.gwrite_with::<u32>(TXAN_TAG, offset, LE)?;
            data.gwrite_with::<TextureAnimationChunk>(model.texture_animation_chunk.unwrap(), offset, LE)?;
        }
        if model.geoset_chunk.is_some() {
            data.gwrite_with::<u32>(GEOS_TAG, offset, LE)?;
            data.gwrite_with::<GeosetChunk>(model.geoset_chunk.unwrap(), offset, LE)?;
        }
        if model.geoset_animation_chunk.is_some() {
            data.gwrite_with::<u32>(GEOA_TAG, offset, LE)?;
            data.gwrite_with::<GeosetAnimationChunk>(model.geoset_animation_chunk.unwrap(), offset, LE)?;
        }
        if model.bone_chunk.is_some() {
            data.gwrite_with::<u32>(BONE_TAG, offset, LE)?;
            data.gwrite_with::<BoneChunk>(model.bone_chunk.unwrap(), offset, LE)?;
        }
        if model.light_chunk.is_some() {
            data.gwrite_with::<u32>(LITE_TAG, offset, LE)?;
            data.gwrite_with::<LightChunk>(model.light_chunk.unwrap(), offset, LE)?;
        }
        if model.helper_chunk.is_some() {
            data.gwrite_with::<u32>(HELP_TAG, offset, LE)?;
            data.gwrite_with::<HelperChunk>(model.helper_chunk.unwrap(), offset, LE)?;
        }
        if model.attachment_chunk.is_some() {
            data.gwrite_with::<u32>(ATCH_TAG, offset, LE)?;
            data.gwrite_with::<AttachmentChunk>(model.attachment_chunk.unwrap(), offset, LE)?;
        }
        if model.pivot_point_chunk.is_some() {
            data.gwrite_with::<u32>(PIVT_TAG, offset, LE)?;
            data.gwrite_with::<PivotPointChunk>(model.pivot_point_chunk.unwrap(), offset, LE)?;
        }
        if model.particle_emitter_chunk.is_some() {
            data.gwrite_with::<u32>(PREM_TAG, offset, LE)?;
            data.gwrite_with::<ParticleEmitterChunk>(model.particle_emitter_chunk.unwrap(), offset, LE)?;
        }
        if model.particle_emitter2_chunk.is_some() {
            data.gwrite_with::<u32>(PRE2_TAG, offset, LE)?;
            data.gwrite_with::<ParticleEmitter2Chunk>(model.particle_emitter2_chunk.unwrap(), offset, LE)?;
        }
        if model.ribbon_emitter_chunk.is_some() {
            data.gwrite_with::<u32>(RIBB_TAG, offset, LE)?;
            data.gwrite_with::<RibbonEmitterChunk>(model.ribbon_emitter_chunk.unwrap(), offset, LE)?;
        }
        if model.event_object_chunk.is_some() {
            data.gwrite_with::<u32>(EVTS_TAG, offset, LE)?;
            data.gwrite_with::<EventObjectChunk>(model.event_object_chunk.unwrap(), offset, LE)?;
        }
        if model.camera_chunk.is_some() {
            data.gwrite_with::<u32>(CAMS_TAG, offset, LE)?;
            data.gwrite_with::<CameraChunk>(model.camera_chunk.unwrap(), offset, LE)?;
        }
        if model.collision_shape_chunk.is_some() {
            data.gwrite_with::<u32>(CLID_TAG, offset, LE)?;
            data.gwrite_with::<CollisionShapeChunk>(model.collision_shape_chunk.unwrap(), offset, LE)?;
        }
        if model.material_chunk.is_some() {
            data.gwrite_with::<u32>(MTLS_TAG, offset, LE)?;
            data.gwrite_with::<MaterialChunk>(model.material_chunk.unwrap(), offset, LE)?;
        }

        // Return result
        Ok(data)
    }

    fn correct_chunk_size(&mut self) {
        if self.version_chunk.is_some() {
            let version = self.version_chunk.as_mut().unwrap();
            version.calculate_chunk_size();
        }
        if self.model_chunk.is_some() {
            let model = self.model_chunk.as_mut().unwrap();
            model.calculate_chunk_size();
        }
        if self.sequence_chunk.is_some() {
            let sequence = self.sequence_chunk.as_mut().unwrap();
            sequence.calculate_chunk_size();
        }
        if self.global_sequence_chunk.is_some() {
            let global_sequence = self.global_sequence_chunk.as_mut().unwrap();
            global_sequence.calculate_chunk_size();
        }
        if self.texture_chunk.is_some() {
            let texture = self.texture_chunk.as_mut().unwrap();
            texture.calculate_chunk_size();
        }
        if self.texture_animation_chunk.is_some() {
            let texture_animation = self.texture_animation_chunk.as_mut().unwrap();
            texture_animation.calculate_chunk_size();
        }
        if self.geoset_chunk.is_some() {
            let geoset = self.geoset_chunk.as_mut().unwrap();
            geoset.calculate_chunk_size();
        }
        if self.geoset_animation_chunk.is_some() {
            let geoset_animation = self.geoset_animation_chunk.as_mut().unwrap();
            geoset_animation.calculate_chunk_size();
        }
        if self.bone_chunk.is_some() {
            let bone = self.bone_chunk.as_mut().unwrap();
            bone.calculate_chunk_size();
        }
        if self.light_chunk.is_some() {
            let light = self.light_chunk.as_mut().unwrap();
            light.calculate_chunk_size();
        }
        if self.helper_chunk.is_some() {
            let helper = self.helper_chunk.as_mut().unwrap();
            helper.calculate_chunk_size();
        }
        if self.attachment_chunk.is_some() {
            let attachment = self.attachment_chunk.as_mut().unwrap();
            attachment.calculate_chunk_size();
        }
        if self.pivot_point_chunk.is_some() {
            let pivot = self.pivot_point_chunk.as_mut().unwrap();
            pivot.calculate_chunk_size();
        }
        if self.particle_emitter_chunk.is_some() {
            let particle_emitter = self.particle_emitter_chunk.as_mut().unwrap();
            particle_emitter.calculate_chunk_size();
        }
        if self.particle_emitter2_chunk.is_some() {
            let particle_emitter2 = self.particle_emitter2_chunk.as_mut().unwrap();
            particle_emitter2.calculate_chunk_size();
        }
        if self.ribbon_emitter_chunk.is_some() {
            let ribbon_emitter = self.ribbon_emitter_chunk.as_mut().unwrap();
            ribbon_emitter.calculate_chunk_size();
        }
        if self.event_object_chunk.is_some() {
            let event_object = self.event_object_chunk.as_mut().unwrap();
            event_object.calculate_chunk_size();
        }
        if self.camera_chunk.is_some() {
            let camera = self.camera_chunk.as_mut().unwrap();
            camera.calculate_chunk_size();
        }
        if self.collision_shape_chunk.is_some() {
            let collision_shape = self.collision_shape_chunk.as_mut().unwrap();
            collision_shape.calculate_chunk_size();
        }
        if self.material_chunk.is_some() {
            let material = self.material_chunk.as_mut().unwrap();
            material.calculate_chunk_size();
        }
    }

    fn model_total_size(&self) -> usize {
        let mut result = 0usize;
        // MDLX_TAG
        result += 4;
        // Tag + size of bytes inside structs
        if self.version_chunk.is_some() {
            let version = self.version_chunk.as_ref().unwrap();
            result += 4;
            result += version.total_bytes_size();
        }
        if self.model_chunk.is_some() {
            let model = self.model_chunk.as_ref().unwrap();
            result += 4;
            result += model.total_bytes_size();
        }
        if self.sequence_chunk.is_some() {
            let sequence = self.sequence_chunk.as_ref().unwrap();
            result += 4;
            result += sequence.total_bytes_size();
        }
        if self.global_sequence_chunk.is_some() {
            let global_sequence = self.global_sequence_chunk.as_ref().unwrap();
            result += 4;
            result += global_sequence.total_bytes_size();
        }
        if self.texture_chunk.is_some() {
            let texture = self.texture_chunk.as_ref().unwrap();
            result += 4;
            result += texture.total_bytes_size();
        }
        if self.texture_animation_chunk.is_some() {
            let texture_animation = self.texture_animation_chunk.as_ref().unwrap();
            result += 4;
            result += texture_animation.total_bytes_size();
        }
        if self.geoset_chunk.is_some() {
            let geoset = self.geoset_chunk.as_ref().unwrap();
            result += 4;
            result += geoset.total_bytes_size();
        }
        if self.geoset_animation_chunk.is_some() {
            let geoset_animation = self.geoset_animation_chunk.as_ref().unwrap();
            result += 4;
            result += geoset_animation.total_bytes_size();
        }
        if self.bone_chunk.is_some() {
            let bone = self.bone_chunk.as_ref().unwrap();
            result += 4;
            result += bone.total_bytes_size();
        }
        if self.light_chunk.is_some() {
            let light = self.light_chunk.as_ref().unwrap();
            result += 4;
            result += light.total_bytes_size();
        }
        if self.helper_chunk.is_some() {
            let helper = self.helper_chunk.as_ref().unwrap();
            result += 4;
            result += helper.total_bytes_size();
        }
        if self.attachment_chunk.is_some() {
            let attachment = self.attachment_chunk.as_ref().unwrap();
            result += 4;
            result += attachment.total_bytes_size();
        }
        if self.pivot_point_chunk.is_some() {
            let pivot = self.pivot_point_chunk.as_ref().unwrap();
            result += 4;
            result += pivot.total_bytes_size();
        }
        if self.particle_emitter_chunk.is_some() {
            let particle_emitter = self.particle_emitter_chunk.as_ref().unwrap();
            result += 4;
            result += particle_emitter.total_bytes_size();
        }
        if self.particle_emitter2_chunk.is_some() {
            let particle_emitter2 = self.particle_emitter2_chunk.as_ref().unwrap();
            result += 4;
            result += particle_emitter2.total_bytes_size();
        }
        if self.ribbon_emitter_chunk.is_some() {
            let ribbon_emitter = self.ribbon_emitter_chunk.as_ref().unwrap();
            result += 4;
            result += ribbon_emitter.total_bytes_size();
        }
        if self.event_object_chunk.is_some() {
            let event_object = self.event_object_chunk.as_ref().unwrap();
            result += 4;
            result += event_object.total_bytes_size();
        }
        if self.camera_chunk.is_some() {
            let camera = self.camera_chunk.as_ref().unwrap();
            result += 4;
            result += camera.total_bytes_size();
        }
        if self.collision_shape_chunk.is_some() {
            let collision_shape = self.collision_shape_chunk.as_ref().unwrap();
            result += 4;
            result += collision_shape.total_bytes_size();
        }
        if self.material_chunk.is_some() {
            let material = self.material_chunk.as_ref().unwrap();
            result += 4;
            result += material.total_bytes_size();
        }

        result
    }

    fn handle_tag(&mut self, tag: u32, data: &[u8], offset: &mut usize) -> Result<(), scroll::Error> {
        match tag {
            VERS_TAG => {
                let version_chunk = data.gread_with::<VersionChunk>(offset, LE)?;
                self.version_chunk = Some(version_chunk);
            },
            MODL_TAG => {
                let model_chunk = data.gread_with::<ModelChunk>(offset, LE)?;
                self.model_chunk = Some(model_chunk);
            },
            SEQS_TAG => {
                let sequence_chunk = data.gread_with::<SequenceChunk>(offset, LE)?;
                self.sequence_chunk = Some(sequence_chunk);
            },
            GLBS_TAG => {
                let global_sequence_chunk = data.gread_with::<GlobalSequenceChunk>(offset, LE)?;
                self.global_sequence_chunk = Some(global_sequence_chunk);
            }
            TEXS_TAG => {
                let texture_chunk = data.gread_with::<TextureChunk>(offset, LE)?;
                self.texture_chunk = Some(texture_chunk);
            },
            TXAN_TAG => {
                let texture_animation_chunk = data.gread_with::<TextureAnimationChunk>(offset, LE)?;
                self.texture_animation_chunk = Some(texture_animation_chunk);
            },
            GEOS_TAG => {
                let geoset_chunk = data.gread_with::<GeosetChunk>(offset, LE)?;
                self.geoset_chunk = Some(geoset_chunk);
            },
            GEOA_TAG => {
                let geoset_animation_chunk = data.gread_with::<GeosetAnimationChunk>(offset, LE)?;
                self.geoset_animation_chunk = Some(geoset_animation_chunk);
            },
            BONE_TAG => {
                let bone_chunk = data.gread_with::<BoneChunk>(offset, LE)?;
                self.bone_chunk = Some(bone_chunk);
            },
            LITE_TAG => {
                let light_chunk = data.gread_with::<LightChunk>(offset, LE)?;
                self.light_chunk = Some(light_chunk);
            },
            HELP_TAG => {
                let helper_chunk = data.gread_with::<HelperChunk>(offset, LE)?;
                self.helper_chunk = Some(helper_chunk);
            },
            ATCH_TAG => {
                let attachment_chunk = data.gread_with::<AttachmentChunk>(offset, LE)?;
                self.attachment_chunk = Some(attachment_chunk);
            },
            PIVT_TAG => {
                let pivot_chunk = data.gread_with::<PivotPointChunk>(offset, LE)?;
                self.pivot_point_chunk = Some(pivot_chunk);
            },
            PREM_TAG => {
                let particle_emitter_chunk = data.gread_with::<ParticleEmitterChunk>(offset, LE)?;
                self.particle_emitter_chunk = Some(particle_emitter_chunk);
            },
            PRE2_TAG => {
                let particle_emitter2_chunk = data.gread_with::<ParticleEmitter2Chunk>(offset, LE)?;
                self.particle_emitter2_chunk = Some(particle_emitter2_chunk);
            },
            RIBB_TAG => {
                let ribbon_emitter_chunk = data.gread_with::<RibbonEmitterChunk>(offset, LE)?;
                self.ribbon_emitter_chunk = Some(ribbon_emitter_chunk);
            },
            EVTS_TAG => {
                let event_object_chunk = data.gread_with::<EventObjectChunk>(offset, LE)?;
                self.event_object_chunk = Some(event_object_chunk);
            },
            CAMS_TAG => {
                let camera_chunk = data.gread_with::<CameraChunk>(offset, LE)?;
                self.camera_chunk = Some(camera_chunk);
            },
            CLID_TAG => {
                let collision_shape_chunk = data.gread_with::<CollisionShapeChunk>(offset, LE)?;
                self.collision_shape_chunk = Some(collision_shape_chunk);
            },
            MTLS_TAG => {
                let material_chunk = data.gread_with::<MaterialChunk>(offset, LE)?;
                self.material_chunk = Some(material_chunk);
            },
            _ => unreachable!(),
        }
        Ok(())
    }
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

impl ctx::TryIntoCtx<Endian> for VersionChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;
        src.gwrite_with::<u32>(self.version, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for VersionChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += size_of_val(&self.version);

        result
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

impl ctx::TryIntoCtx<Endian> for ModelChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        // Name has fixed size
        let max_name_len = 336usize;
        let null_offset = &mut offset.clone();
        for _ in 0..max_name_len {
            src.gwrite_with::<u8>(0x0, null_offset, LE)?;
        }
        // FIX THIS IN SCROLL LIB
        src.gwrite_with::<&str>(self.name.as_ref(), &mut offset.clone(), ())?.to_string();
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.unknown, offset, ctx)?;
        src.gwrite_with::<f32>(self.bounds_radius, offset, ctx)?;
        for id in 0..3 {
            src.gwrite_with::<f32>(self.minimum_extent[id], offset, ctx)?;
        }
        for id in 0..3 {
            src.gwrite_with::<f32>(self.maximum_extent[id], offset, ctx)?;
        }
        src.gwrite_with::<u32>(self.blend_time, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for ModelChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        let max_name_len = 336usize;
        result += max_name_len;

        result += size_of_val(&self.unknown);
        result += size_of_val(&self.bounds_radius);
        result += size_of_val(&self.minimum_extent);
        result += size_of_val(&self.maximum_extent);
        result += size_of_val(&self.blend_time);

        result
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

impl ctx::TryIntoCtx<Endian> for SequenceChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for sequence in self.data {
            src.gwrite_with::<Sequence>(sequence, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for SequenceChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for sequence in &self.data {
            result += sequence.total_bytes_size();
        }

        result
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

impl ctx::TryIntoCtx<Endian> for Sequence {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        // Name has fixed size
        let max_name_len = 80usize;
        let null_offset = &mut 0usize;
        for _ in 0..max_name_len {
            src.gwrite_with::<u8>(0x0, null_offset, LE)?;
        }
        src.gwrite_with::<&str>(self.name.as_ref(), &mut offset.clone(), ())?.to_string();
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.interval_start, offset, ctx)?;
        src.gwrite_with::<u32>(self.interval_end, offset, ctx)?;
        src.gwrite_with::<f32>(self.move_speed, offset, ctx)?;
        src.gwrite_with::<u32>(self.non_looping, offset, ctx)?;
        src.gwrite_with::<f32>(self.rarity, offset, ctx)?;
        src.gwrite_with::<u32>(self.unknown, offset, ctx)?;
        src.gwrite_with::<f32>(self.bounds_radius, offset, ctx)?;
        for id in 0..3 {
            src.gwrite_with::<f32>(self.minimum_extent[id], offset, ctx)?;
        }
        for id in 0..3 {
            src.gwrite_with::<f32>(self.maximum_extent[id], offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for Sequence {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        let max_name_len = 80usize;
        result += max_name_len;

        result += size_of_val(&self.interval_start);
        result += size_of_val(&self.interval_end);
        result += size_of_val(&self.move_speed);
        result += size_of_val(&self.non_looping);
        result += size_of_val(&self.rarity);
        result += size_of_val(&self.unknown);
        result += size_of_val(&self.bounds_radius);
        result += size_of_val(&self.minimum_extent);
        result += size_of_val(&self.maximum_extent);

        result
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

impl ctx::TryIntoCtx<Endian> for GlobalSequenceChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for global_sequence in self.data {
            src.gwrite_with::<GlobalSequence>(global_sequence, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for GlobalSequenceChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for global_sequence in &self.data {
            result += global_sequence.total_bytes_size();
        }

        result
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

impl ctx::TryIntoCtx<Endian> for GlobalSequence {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.duration, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for GlobalSequence {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.duration);

        result
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

impl ctx::TryIntoCtx<Endian> for TextureChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for texture in self.data {
            src.gwrite_with::<Texture>(texture, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for TextureChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for texture in &self.data {
            result += texture.total_bytes_size();
        }

        result
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

impl ctx::TryIntoCtx<Endian> for Texture {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.replaceable_id, offset, ctx)?;

        // Name has fixed size
        let max_name_len = 256usize;
        let null_offset = &mut offset.clone();
        for _ in 0..max_name_len {
            src.gwrite_with::<u8>(0x0, null_offset, LE)?;
        }
        src.gwrite_with::<&str>(self.file_name.as_ref(), &mut offset.clone(), ())?.to_string();
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.unknown, offset, ctx)?;
        src.gwrite_with::<u32>(self.flags, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Texture {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.replaceable_id);
        let max_name_len = 256usize;
        result += max_name_len;
        result += size_of_val(&self.unknown);
        result += size_of_val(&self.flags);

        result
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

impl ctx::TryIntoCtx<Endian> for TextureAnimationChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for texture_animation in self.data {
            src.gwrite_with::<TextureAnimation>(texture_animation, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for TextureAnimationChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for texture_animation in &self.data {
            result += texture_animation.total_bytes_size();
        }

        result
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

impl ctx::TryIntoCtx<Endian> for TextureAnimation {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.inclusive_size, offset, ctx)?;

        if self.texture_translation.is_some() {
            src.gwrite_with::<u32>(KTAT_TAG, offset, ctx)?;
            src.gwrite_with::<TextureTranslation>(self.texture_translation.unwrap(), offset, ctx)?;
        }
        if self.texture_rotation.is_some() {
            src.gwrite_with::<u32>(KTAR_TAG, offset, ctx)?;
            src.gwrite_with::<TextureRotation>(self.texture_rotation.unwrap(), offset, ctx)?;
        }
        if self.texture_scaling.is_some() {
            src.gwrite_with::<u32>(KTAS_TAG, offset, ctx)?;
            src.gwrite_with::<TextureScaling>(self.texture_scaling.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for TextureAnimation {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);

        if self.texture_translation.is_some() {
            result += 4;
            result += self.texture_translation.as_ref().unwrap().total_bytes_size();
        }
        if self.texture_rotation.is_some() {
            result += 4;
            result += self.texture_rotation.as_ref().unwrap().total_bytes_size();
        }
        if self.texture_scaling.is_some() {
            result += 4;
            result += self.texture_scaling.as_ref().unwrap().total_bytes_size();
        }

        result
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

impl ctx::TryIntoCtx<Endian> for GeosetChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for GeosetChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
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

impl ctx::TryIntoCtx<Endian> for GeosetAnimationChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for geoset_animation in self.data {
            src.gwrite_with::<GeosetAnimation>(geoset_animation, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for GeosetAnimationChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for geoset_animation in &self.data {
            result += geoset_animation.total_bytes_size();
        }

        result
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

impl ctx::TryIntoCtx<Endian> for GeosetAnimation {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.inclusive_size, offset, ctx)?;
        src.gwrite_with::<f32>(self.alpha, offset, ctx)?;
        src.gwrite_with::<u32>(self.flags, offset, ctx)?;
        for id in 0..3 {
            src.gwrite_with::<f32>(self.color[id], offset, ctx)?;
        }
        src.gwrite_with::<u32>(self.geoset_id, offset, ctx)?;

        if self.geoset_alpha.is_some() {
            src.gwrite_with::<u32>(KGAO_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetAlpha>(self.geoset_alpha.unwrap(), offset, ctx)?;
        }
        if self.geoset_color.is_some() {
            src.gwrite_with::<u32>(KGAC_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetColor>(self.geoset_color.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for GeosetAnimation {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);
        result += size_of_val(&self.alpha);
        result += size_of_val(&self.flags);
        result += size_of_val(&self.color);
        result += size_of_val(&self.geoset_id);

        if self.geoset_alpha.is_some() {
            result += 4;
            result += self.geoset_alpha.as_ref().unwrap().total_bytes_size();
        }
        if self.geoset_color.is_some() {
            result += 4;
            result += self.geoset_color.as_ref().unwrap().total_bytes_size();
        }

        result
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
            total_size += bone.node.inclusive_size + 4 + 4;
            data.push(bone);
        }

        Ok((BoneChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for BoneChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for bone in self.data {
            src.gwrite_with::<Bone>(bone, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for BoneChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for bone in &self.data {
            result += bone.total_bytes_size();
        }

        result
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

impl ctx::TryIntoCtx<Endian> for Bone {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<Node>(self.node, offset, ctx)?;
        src.gwrite_with::<u32>(self.geoset_id, offset, ctx)?;
        src.gwrite_with::<u32>(self.geoset_animation_id, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Bone {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += &self.node.total_bytes_size();
        result += size_of_val(&self.geoset_id);
        result += size_of_val(&self.geoset_animation_id);

        result
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

impl ctx::TryIntoCtx<Endian> for Node {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.inclusive_size, offset, ctx)?;

        // Name has fixed size
        let max_name_len = 80usize;
        let null_offset = &mut offset.clone();
        for _ in 0..max_name_len {
            src.gwrite_with::<u8>(0x0, null_offset, LE)?;
        }
        src.gwrite_with::<&str>(self.name.as_ref(), &mut offset.clone(), ())?.to_string();
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.object_id, offset, ctx)?;
        src.gwrite_with::<u32>(self.parent_id, offset, ctx)?;
        src.gwrite_with::<u32>(self.flags, offset, ctx)?;

        if self.geoset_translation.is_some() {
            src.gwrite_with::<u32>(KGTR_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetTranslation>(self.geoset_translation.unwrap(), offset, ctx)?;
        }
        if self.geoset_rotation.is_some() {
            src.gwrite_with::<u32>(KGRT_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetRotation>(self.geoset_rotation.unwrap(), offset, ctx)?;
        }
        if self.geoset_scaling.is_some() {
            src.gwrite_with::<u32>(KGSC_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetScaling>(self.geoset_scaling.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for Node {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);

        let max_name_len = 80usize;
        result += max_name_len;

        result += size_of_val(&self.object_id);
        result += size_of_val(&self.parent_id);
        result += size_of_val(&self.flags);

        if self.geoset_translation.is_some() {
            result += 4;
            result += self.geoset_translation.as_ref().unwrap().total_bytes_size();
        }
        if self.geoset_rotation.is_some() {
            result += 4;
            result += self.geoset_rotation.as_ref().unwrap().total_bytes_size();
        }
        if self.geoset_scaling.is_some() {
            result += 4;
            result += self.geoset_scaling.as_ref().unwrap().total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct LightChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for LightChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((LightChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for LightChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for LightChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct HelperChunk {
    pub chunk_size: u32,

    pub data: Vec<Helper>,
}

impl ctx::TryFromCtx<'_, Endian> for HelperChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let helper = src.gread_with::<Helper>(offset, ctx)?;
            total_size += helper.node.inclusive_size;
            data.push(helper);
        }

        Ok((HelperChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for HelperChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for helper in self.data {
            src.gwrite_with::<Helper>(helper, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for HelperChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for helper in &self.data {
            result += helper.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Helper {
    pub node: Node,
}

impl ctx::TryFromCtx<'_, Endian> for Helper {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let node = src.gread_with::<Node>(offset, ctx)?;

        Ok((Helper {
            node,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Helper {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<Node>(self.node, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Helper {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += &self.node.total_bytes_size();

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct AttachmentChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for AttachmentChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((AttachmentChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for AttachmentChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for AttachmentChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
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

impl ctx::TryIntoCtx<Endian> for PivotPointChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for pivot_point in self.data {
            src.gwrite_with::<PivotPoint>(pivot_point, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for PivotPointChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for pivot_point in &self.data {
            result += pivot_point.total_bytes_size();
        }

        result
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

impl ctx::TryIntoCtx<Endian> for PivotPoint {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        for id in 0..3 {
            src.gwrite_with::<f32>(self.position[id], offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for PivotPoint {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.position);

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct ParticleEmitterChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for ParticleEmitterChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((ParticleEmitterChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for ParticleEmitterChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for ParticleEmitterChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct ParticleEmitter2Chunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for ParticleEmitter2Chunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((ParticleEmitter2Chunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for ParticleEmitter2Chunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for ParticleEmitter2Chunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct RibbonEmitterChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for RibbonEmitterChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((RibbonEmitterChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for RibbonEmitterChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for RibbonEmitterChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct EventObjectChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for EventObjectChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((EventObjectChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for EventObjectChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for EventObjectChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct CameraChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for CameraChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((CameraChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for CameraChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for CameraChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct CollisionShapeChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for CollisionShapeChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((CollisionShapeChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for CollisionShapeChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for CollisionShapeChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct MaterialChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

impl ctx::TryFromCtx<'_, Endian> for MaterialChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((MaterialChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for MaterialChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for MaterialChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_mdx_file_api() {
        //let raw_data = fs::read("testfiles/base_model.mdx").unwrap();
        //let raw_data = fs::read("testfiles/druidcat.mdx").unwrap();
        //let raw_data = fs::read("testfiles/herochaos.mdx").unwrap();
        let raw_data = fs::read("testfiles/chaoswarrior.mdx").unwrap();
        let model = MDLXModel::read_mdx_file(raw_data.clone()).unwrap();
        dbg!(&raw_data.len());
        dbg!(&model.model_total_size());

        let mut bytes = MDLXModel::write_mdx_file(model).unwrap();
        dbg!(&bytes.len());
        fs::write("testfiles/chaoswarrior_resave.mdx", bytes).unwrap();
    }

}
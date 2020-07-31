#![allow(dead_code, unused_variables, non_snake_case)]

use crate::driver;
use crate::va;
use crate::vdpau;

const VDP_DECODER_PROFILE_MPEG1: vdpau::VdpDecoderProfile = 0;
const VDP_DECODER_PROFILE_MPEG2_SIMPLE: vdpau::VdpDecoderProfile = 1;
const VDP_DECODER_PROFILE_MPEG2_MAIN: vdpau::VdpDecoderProfile = 2;
const VDP_DECODER_PROFILE_H264_BASELINE: vdpau::VdpDecoderProfile = 6;
const VDP_DECODER_PROFILE_H264_MAIN: vdpau::VdpDecoderProfile = 7;
const VDP_DECODER_PROFILE_H264_HIGH: vdpau::VdpDecoderProfile = 8;
const VDP_DECODER_PROFILE_VC1_SIMPLE: vdpau::VdpDecoderProfile = 9;
const VDP_DECODER_PROFILE_VC1_MAIN: vdpau::VdpDecoderProfile = 10;
const VDP_DECODER_PROFILE_VC1_ADVANCED: vdpau::VdpDecoderProfile = 11;
const VDP_DECODER_PROFILE_MPEG4_PART2_SP: vdpau::VdpDecoderProfile = 12;
const VDP_DECODER_PROFILE_MPEG4_PART2_ASP: vdpau::VdpDecoderProfile = 13;
const VDP_DECODER_PROFILE_DIVX4_QMOBILE: vdpau::VdpDecoderProfile = 14;
const VDP_DECODER_PROFILE_DIVX4_MOBILE: vdpau::VdpDecoderProfile = 15;
const VDP_DECODER_PROFILE_DIVX4_HOME_THEATER: vdpau::VdpDecoderProfile = 16;
const VDP_DECODER_PROFILE_DIVX4_HD_1080P: vdpau::VdpDecoderProfile = 17;
const VDP_DECODER_PROFILE_DIVX5_QMOBILE: vdpau::VdpDecoderProfile = 18;
const VDP_DECODER_PROFILE_DIVX5_MOBILE: vdpau::VdpDecoderProfile = 19;
const VDP_DECODER_PROFILE_DIVX5_HOME_THEATER: vdpau::VdpDecoderProfile = 20;
const VDP_DECODER_PROFILE_DIVX5_HD_1080P: vdpau::VdpDecoderProfile = 21;
const VDP_DECODER_PROFILE_H264_CONSTRAINED_BASELINE: vdpau::VdpDecoderProfile = 22;
const VDP_DECODER_PROFILE_H264_EXTENDED: vdpau::VdpDecoderProfile = 23;
const VDP_DECODER_PROFILE_H264_PROGRESSIVE_HIGH: vdpau::VdpDecoderProfile = 24;
const VDP_DECODER_PROFILE_H264_CONSTRAINED_HIGH: vdpau::VdpDecoderProfile = 25;
const VDP_DECODER_PROFILE_H264_HIGH_444_PREDICTIVE: vdpau::VdpDecoderProfile = 26;
const VDP_DECODER_PROFILE_VP9_PROFILE_0: vdpau::VdpDecoderProfile = 27;
const VDP_DECODER_PROFILE_VP9_PROFILE_1: vdpau::VdpDecoderProfile = 28;
const VDP_DECODER_PROFILE_VP9_PROFILE_2: vdpau::VdpDecoderProfile = 29;
const VDP_DECODER_PROFILE_VP9_PROFILE_3: vdpau::VdpDecoderProfile = 30;
const VDP_DECODER_PROFILE_HEVC_MAIN: vdpau::VdpDecoderProfile = 100;
const VDP_DECODER_PROFILE_HEVC_MAIN_10: vdpau::VdpDecoderProfile = 101;
const VDP_DECODER_PROFILE_HEVC_MAIN_STILL: vdpau::VdpDecoderProfile = 102;
const VDP_DECODER_PROFILE_HEVC_MAIN_12: vdpau::VdpDecoderProfile = 103;
const VDP_DECODER_PROFILE_HEVC_MAIN_444: vdpau::VdpDecoderProfile = 104;
const VDP_DECODER_PROFILE_HEVC_MAIN_444_10: vdpau::VdpDecoderProfile = 105;
const VDP_DECODER_PROFILE_HEVC_MAIN_444_12: vdpau::VdpDecoderProfile = 106;

pub(crate) fn get_vdpau_profile(
    profile: va::VAProfile,
) -> Result<vdpau::VdpDecoderProfile, va::VAStatus> {
    match profile {
        va::VAProfile_VAProfileMPEG2Simple => Ok(VDP_DECODER_PROFILE_MPEG2_SIMPLE),
        va::VAProfile_VAProfileMPEG2Main => Ok(VDP_DECODER_PROFILE_MPEG2_MAIN),

        va::VAProfile_VAProfileMPEG4Simple => Ok(VDP_DECODER_PROFILE_MPEG4_PART2_SP),
        va::VAProfile_VAProfileMPEG4AdvancedSimple => Ok(VDP_DECODER_PROFILE_MPEG4_PART2_ASP),

        va::VAProfile_VAProfileH264Baseline => Ok(VDP_DECODER_PROFILE_H264_BASELINE),
        va::VAProfile_VAProfileH264Main => Ok(VDP_DECODER_PROFILE_H264_MAIN),
        va::VAProfile_VAProfileH264High => Ok(VDP_DECODER_PROFILE_H264_HIGH),

        va::VAProfile_VAProfileVC1Simple => Ok(VDP_DECODER_PROFILE_VC1_SIMPLE),
        va::VAProfile_VAProfileVC1Main => Ok(VDP_DECODER_PROFILE_VC1_MAIN),
        va::VAProfile_VAProfileVC1Advanced => Ok(VDP_DECODER_PROFILE_VC1_ADVANCED),

        va::VAProfile_VAProfileVP9Profile0 => Ok(VDP_DECODER_PROFILE_VP9_PROFILE_0),
        _ => Err(va::VA_STATUS_ERROR_UNSUPPORTED_PROFILE as _),
    }
}

fn is_va_profile_supported(
    ctx: va::VADriverContextP,
    profile: va::VAProfile,
) -> Result<(), va::VAStatus> {
    let driver = unsafe { &*((*ctx).pDriverData as *mut driver::Driver) };

    let vdpau_profile = get_vdpau_profile(profile)?;

    let mut is_supported = vdpau::VDP_FALSE;
    let mut max_level = 0;
    let mut max_references = 0;
    let mut max_width = 0;
    let mut max_height = 0;
    if !unsafe {
        driver.vdpau_loader.decoder_query_capabilities.unwrap()(
            driver.vdpau_device,
            vdpau_profile,
            &mut is_supported as *mut _ as *mut _,
            &mut max_level,
            &mut max_references,
            &mut max_width,
            &mut max_height,
        )
    }
    .eq(&vdpau::VdpStatus_VDP_STATUS_OK)
        || is_supported != vdpau::VDP_TRUE
    {
        return Err(va::VA_STATUS_ERROR_UNSUPPORTED_PROFILE as _);
    }

    Ok(())
}

pub(crate) extern "C" fn vaQueryConfigProfiles(
    ctx: va::VADriverContextP,
    profile_list: *mut va::VAProfile,
    num_profiles: *mut ::std::os::raw::c_int,
) -> va::VAStatus {
    let va_profiles = [
        va::VAProfile_VAProfileMPEG2Simple,
        va::VAProfile_VAProfileMPEG2Main,
        va::VAProfile_VAProfileMPEG4Simple,
        va::VAProfile_VAProfileMPEG4AdvancedSimple,
        va::VAProfile_VAProfileMPEG4Main,
        va::VAProfile_VAProfileH264Baseline,
        va::VAProfile_VAProfileH264Main,
        va::VAProfile_VAProfileH264High,
        va::VAProfile_VAProfileVC1Simple,
        va::VAProfile_VAProfileVC1Main,
        va::VAProfile_VAProfileVC1Advanced,
        va::VAProfile_VAProfileVP9Profile0,
    ];
    let mut profile_index = 0;
    va_profiles.iter().enumerate().for_each(|(i, va_profile)| {
        if is_va_profile_supported(ctx, *va_profile).is_ok() {
            unsafe { profile_list.add(profile_index).write(*va_profile) };
            profile_index += 1;
        }
    });
    unsafe { num_profiles.write(profile_index as _) };
    va::VA_STATUS_SUCCESS as _
}
pub(crate) extern "C" fn vaQueryConfigEntrypoints(
    ctx: va::VADriverContextP,
    profile: va::VAProfile,
    entrypoint_list: *mut va::VAEntrypoint,
    num_entrypoints: *mut ::std::os::raw::c_int,
) -> va::VAStatus {
    match get_vdpau_profile(profile) {
        Ok(profile) => match profile as va::VAProfile {
            va::VAProfile_VAProfileMPEG2Simple
            | va::VAProfile_VAProfileMPEG2Main
            | va::VAProfile_VAProfileMPEG4Simple
            | va::VAProfile_VAProfileMPEG4AdvancedSimple
            | va::VAProfile_VAProfileMPEG4Main
            | va::VAProfile_VAProfileH264Baseline
            | va::VAProfile_VAProfileH264Main
            | va::VAProfile_VAProfileH264High
            | va::VAProfile_VAProfileVC1Simple
            | va::VAProfile_VAProfileVC1Main
            | va::VAProfile_VAProfileVC1Advanced
            | va::VAProfile_VAProfileVP9Profile0 => unsafe {
                entrypoint_list.write(va::VAEntrypoint_VAEntrypointVLD);
                num_entrypoints.write(1);
            },
            _ => unsafe {
                num_entrypoints.write(0);
            },
        },
        Err(error) => {
            return error;
        }
    };
    va::VA_STATUS_SUCCESS as _
}
pub(crate) extern "C" fn vaGetConfigAttributes(
    ctx: va::VADriverContextP,
    profile: va::VAProfile,
    entrypoint: va::VAEntrypoint,
    attrib_list: *mut va::VAConfigAttrib,
    num_attribs: ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaCreateConfig(
    ctx: va::VADriverContextP,
    profile: va::VAProfile,
    entrypoint: va::VAEntrypoint,
    attrib_list: *mut va::VAConfigAttrib,
    num_attribs: ::std::os::raw::c_int,
    config_id: *mut va::VAConfigID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaDestroyConfig(
    ctx: va::VADriverContextP,
    config_id: va::VAConfigID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaQueryConfigAttributes(
    ctx: va::VADriverContextP,
    config_id: va::VAConfigID,
    profile: *mut va::VAProfile,
    entrypoint: *mut va::VAEntrypoint,
    attrib_list: *mut va::VAConfigAttrib,
    num_attribs: *mut ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaCreateSurfaces(
    ctx: va::VADriverContextP,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    format: ::std::os::raw::c_int,
    num_surfaces: ::std::os::raw::c_int,
    surfaces: *mut va::VASurfaceID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaDestroySurfaces(
    ctx: va::VADriverContextP,
    surface_list: *mut va::VASurfaceID,
    num_surfaces: ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaCreateContext(
    ctx: va::VADriverContextP,
    config_id: va::VAConfigID,
    picture_width: ::std::os::raw::c_int,
    picture_height: ::std::os::raw::c_int,
    flag: ::std::os::raw::c_int,
    render_targets: *mut va::VASurfaceID,
    num_render_targets: ::std::os::raw::c_int,
    context: *mut va::VAContextID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaDestroyContext(
    ctx: va::VADriverContextP,
    context: va::VAContextID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaCreateBuffer(
    ctx: va::VADriverContextP,
    context: va::VAContextID,
    type_: va::VABufferType,
    size: ::std::os::raw::c_uint,
    num_elements: ::std::os::raw::c_uint,
    data: *mut ::std::os::raw::c_void,
    buf_id: *mut va::VABufferID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaBufferSetNumElements(
    ctx: va::VADriverContextP,
    buf_id: va::VABufferID,
    num_elements: ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaMapBuffer(
    ctx: va::VADriverContextP,
    buf_id: va::VABufferID,
    pbuf: *mut *mut ::std::os::raw::c_void,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaUnmapBuffer(
    ctx: va::VADriverContextP,
    buf_id: va::VABufferID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaDestroyBuffer(
    ctx: va::VADriverContextP,
    buffer_id: va::VABufferID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaBeginPicture(
    ctx: va::VADriverContextP,
    context: va::VAContextID,
    render_target: va::VASurfaceID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaRenderPicture(
    ctx: va::VADriverContextP,
    context: va::VAContextID,
    buffers: *mut va::VABufferID,
    num_buffers: ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaEndPicture(
    ctx: va::VADriverContextP,
    context: va::VAContextID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaSyncSurface(
    ctx: va::VADriverContextP,
    render_target: va::VASurfaceID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaQuerySurfaceStatus(
    ctx: va::VADriverContextP,
    render_target: va::VASurfaceID,
    status: *mut va::VASurfaceStatus,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaQuerySurfaceError(
    ctx: va::VADriverContextP,
    render_target: va::VASurfaceID,
    error_status: va::VAStatus,
    error_info: *mut *mut ::std::os::raw::c_void,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaPutSurface(
    ctx: va::VADriverContextP,
    surface: va::VASurfaceID,
    draw: *mut ::std::os::raw::c_void,
    srcx: ::std::os::raw::c_short,
    srcy: ::std::os::raw::c_short,
    srcw: ::std::os::raw::c_ushort,
    srch: ::std::os::raw::c_ushort,
    destx: ::std::os::raw::c_short,
    desty: ::std::os::raw::c_short,
    destw: ::std::os::raw::c_ushort,
    desth: ::std::os::raw::c_ushort,
    cliprects: *mut va::VARectangle,
    number_cliprects: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaQueryImageFormats(
    ctx: va::VADriverContextP,
    format_list: *mut va::VAImageFormat,
    num_formats: *mut ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaCreateImage(
    ctx: va::VADriverContextP,
    format: *mut va::VAImageFormat,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    image: *mut va::VAImage,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaDeriveImage(
    ctx: va::VADriverContextP,
    surface: va::VASurfaceID,
    image: *mut va::VAImage,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaDestroyImage(
    ctx: va::VADriverContextP,
    image: va::VAImageID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaSetImagePalette(
    ctx: va::VADriverContextP,
    image: va::VAImageID,
    palette: *mut ::std::os::raw::c_uchar,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaGetImage(
    ctx: va::VADriverContextP,
    surface: va::VASurfaceID,
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
    width: ::std::os::raw::c_uint,
    height: ::std::os::raw::c_uint,
    image: va::VAImageID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaPutImage(
    ctx: va::VADriverContextP,
    surface: va::VASurfaceID,
    image: va::VAImageID,
    src_x: ::std::os::raw::c_int,
    src_y: ::std::os::raw::c_int,
    src_width: ::std::os::raw::c_uint,
    src_height: ::std::os::raw::c_uint,
    dest_x: ::std::os::raw::c_int,
    dest_y: ::std::os::raw::c_int,
    dest_width: ::std::os::raw::c_uint,
    dest_height: ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaQuerySubpictureFormats(
    ctx: va::VADriverContextP,
    format_list: *mut va::VAImageFormat,
    flags: *mut ::std::os::raw::c_uint,
    num_formats: *mut ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaCreateSubpicture(
    ctx: va::VADriverContextP,
    image: va::VAImageID,
    subpicture: *mut va::VASubpictureID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaDestroySubpicture(
    ctx: va::VADriverContextP,
    subpicture: va::VASubpictureID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaSetSubpictureImage(
    ctx: va::VADriverContextP,
    subpicture: va::VASubpictureID,
    image: va::VAImageID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaSetSubpictureChromakey(
    ctx: va::VADriverContextP,
    subpicture: va::VASubpictureID,
    chromakey_min: ::std::os::raw::c_uint,
    chromakey_max: ::std::os::raw::c_uint,
    chromakey_mask: ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaSetSubpictureGlobalAlpha(
    ctx: va::VADriverContextP,
    subpicture: va::VASubpictureID,
    global_alpha: f32,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaAssociateSubpicture(
    ctx: va::VADriverContextP,
    subpicture: va::VASubpictureID,
    target_surfaces: *mut va::VASurfaceID,
    num_surfaces: ::std::os::raw::c_int,
    src_x: ::std::os::raw::c_short,
    src_y: ::std::os::raw::c_short,
    src_width: ::std::os::raw::c_ushort,
    src_height: ::std::os::raw::c_ushort,
    dest_x: ::std::os::raw::c_short,
    dest_y: ::std::os::raw::c_short,
    dest_width: ::std::os::raw::c_ushort,
    dest_height: ::std::os::raw::c_ushort,
    flags: ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaDeassociateSubpicture(
    ctx: va::VADriverContextP,
    subpicture: va::VASubpictureID,
    target_surfaces: *mut va::VASurfaceID,
    num_surfaces: ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaQueryDisplayAttributes(
    ctx: va::VADriverContextP,
    attr_list: *mut va::VADisplayAttribute,
    num_attributes: *mut ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaGetDisplayAttributes(
    ctx: va::VADriverContextP,
    attr_list: *mut va::VADisplayAttribute,
    num_attributes: ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaSetDisplayAttributes(
    ctx: va::VADriverContextP,
    attr_list: *mut va::VADisplayAttribute,
    num_attributes: ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaBufferInfo(
    ctx: va::VADriverContextP,
    buf_id: va::VABufferID,
    type_: *mut va::VABufferType,
    size: *mut ::std::os::raw::c_uint,
    num_elements: *mut ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaLockSurface(
    ctx: va::VADriverContextP,
    surface: va::VASurfaceID,
    fourcc: *mut ::std::os::raw::c_uint,
    luma_stride: *mut ::std::os::raw::c_uint,
    chroma_u_stride: *mut ::std::os::raw::c_uint,
    chroma_v_stride: *mut ::std::os::raw::c_uint,
    luma_offset: *mut ::std::os::raw::c_uint,
    chroma_u_offset: *mut ::std::os::raw::c_uint,
    chroma_v_offset: *mut ::std::os::raw::c_uint,
    buffer_name: *mut ::std::os::raw::c_uint,
    buffer: *mut *mut ::std::os::raw::c_void,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaUnlockSurface(
    ctx: va::VADriverContextP,
    surface: va::VASurfaceID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaGetSurfaceAttributes(
    dpy: va::VADriverContextP,
    config: va::VAConfigID,
    attrib_list: *mut va::VASurfaceAttrib,
    num_attribs: ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaCreateSurfaces2(
    ctx: va::VADriverContextP,
    format: ::std::os::raw::c_uint,
    width: ::std::os::raw::c_uint,
    height: ::std::os::raw::c_uint,
    surfaces: *mut va::VASurfaceID,
    num_surfaces: ::std::os::raw::c_uint,
    attrib_list: *mut va::VASurfaceAttrib,
    num_attribs: ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaQuerySurfaceAttributes(
    dpy: va::VADriverContextP,
    config: va::VAConfigID,
    attrib_list: *mut va::VASurfaceAttrib,
    num_attribs: *mut ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaAcquireBufferHandle(
    ctx: va::VADriverContextP,
    buf_id: va::VABufferID,
    buf_info: *mut va::VABufferInfo,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaReleaseBufferHandle(
    ctx: va::VADriverContextP,
    buf_id: va::VABufferID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaCreateMFContext(
    ctx: va::VADriverContextP,
    mfe_context: *mut va::VAMFContextID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaMFAddContext(
    ctx: va::VADriverContextP,
    mf_context: va::VAMFContextID,
    context: va::VAContextID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaMFReleaseContext(
    ctx: va::VADriverContextP,
    mf_context: va::VAMFContextID,
    context: va::VAContextID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaMFSubmit(
    ctx: va::VADriverContextP,
    mf_context: va::VAMFContextID,
    contexts: *mut va::VAContextID,
    num_contexts: ::std::os::raw::c_int,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaCreateBuffer2(
    ctx: va::VADriverContextP,
    context: va::VAContextID,
    type_: va::VABufferType,
    width: ::std::os::raw::c_uint,
    height: ::std::os::raw::c_uint,
    unit_size: *mut ::std::os::raw::c_uint,
    pitch: *mut ::std::os::raw::c_uint,
    buf_id: *mut va::VABufferID,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaQueryProcessingRate(
    ctx: va::VADriverContextP,
    config_id: va::VAConfigID,
    proc_buf: *mut va::VAProcessingRateParameter,
    processing_rate: *mut ::std::os::raw::c_uint,
) -> va::VAStatus {
    todo!();
}
pub(crate) extern "C" fn vaExportSurfaceHandle(
    ctx: va::VADriverContextP,
    surface_id: va::VASurfaceID,
    mem_type: u32,
    flags: u32,
    descriptor: *mut ::std::os::raw::c_void,
) -> va::VAStatus {
    todo!();
}

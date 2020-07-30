#![allow(dead_code, unused_variables, non_snake_case)]

use crate::va::*;

pub extern "C" fn vaQueryConfigProfiles(
    ctx: VADriverContextP,
    profile_list: *mut VAProfile,
    num_profiles: *mut ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaQueryConfigEntrypoints(
    ctx: VADriverContextP,
    profile: VAProfile,
    entrypoint_list: *mut VAEntrypoint,
    num_entrypoints: *mut ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaGetConfigAttributes(
    ctx: VADriverContextP,
    profile: VAProfile,
    entrypoint: VAEntrypoint,
    attrib_list: *mut VAConfigAttrib,
    num_attribs: ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaCreateConfig(
    ctx: VADriverContextP,
    profile: VAProfile,
    entrypoint: VAEntrypoint,
    attrib_list: *mut VAConfigAttrib,
    num_attribs: ::std::os::raw::c_int,
    config_id: *mut VAConfigID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaDestroyConfig(ctx: VADriverContextP, config_id: VAConfigID) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaQueryConfigAttributes(
    ctx: VADriverContextP,
    config_id: VAConfigID,
    profile: *mut VAProfile,
    entrypoint: *mut VAEntrypoint,
    attrib_list: *mut VAConfigAttrib,
    num_attribs: *mut ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaCreateSurfaces(
    ctx: VADriverContextP,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    format: ::std::os::raw::c_int,
    num_surfaces: ::std::os::raw::c_int,
    surfaces: *mut VASurfaceID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaDestroySurfaces(
    ctx: VADriverContextP,
    surface_list: *mut VASurfaceID,
    num_surfaces: ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaCreateContext(
    ctx: VADriverContextP,
    config_id: VAConfigID,
    picture_width: ::std::os::raw::c_int,
    picture_height: ::std::os::raw::c_int,
    flag: ::std::os::raw::c_int,
    render_targets: *mut VASurfaceID,
    num_render_targets: ::std::os::raw::c_int,
    context: *mut VAContextID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaDestroyContext(ctx: VADriverContextP, context: VAContextID) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaCreateBuffer(
    ctx: VADriverContextP,
    context: VAContextID,
    type_: VABufferType,
    size: ::std::os::raw::c_uint,
    num_elements: ::std::os::raw::c_uint,
    data: *mut ::std::os::raw::c_void,
    buf_id: *mut VABufferID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaBufferSetNumElements(
    ctx: VADriverContextP,
    buf_id: VABufferID,
    num_elements: ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaMapBuffer(
    ctx: VADriverContextP,
    buf_id: VABufferID,
    pbuf: *mut *mut ::std::os::raw::c_void,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaUnmapBuffer(ctx: VADriverContextP, buf_id: VABufferID) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaDestroyBuffer(ctx: VADriverContextP, buffer_id: VABufferID) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaBeginPicture(
    ctx: VADriverContextP,
    context: VAContextID,
    render_target: VASurfaceID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaRenderPicture(
    ctx: VADriverContextP,
    context: VAContextID,
    buffers: *mut VABufferID,
    num_buffers: ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaEndPicture(ctx: VADriverContextP, context: VAContextID) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaSyncSurface(ctx: VADriverContextP, render_target: VASurfaceID) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaQuerySurfaceStatus(
    ctx: VADriverContextP,
    render_target: VASurfaceID,
    status: *mut VASurfaceStatus,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaQuerySurfaceError(
    ctx: VADriverContextP,
    render_target: VASurfaceID,
    error_status: VAStatus,
    error_info: *mut *mut ::std::os::raw::c_void,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaPutSurface(
    ctx: VADriverContextP,
    surface: VASurfaceID,
    draw: *mut ::std::os::raw::c_void,
    srcx: ::std::os::raw::c_short,
    srcy: ::std::os::raw::c_short,
    srcw: ::std::os::raw::c_ushort,
    srch: ::std::os::raw::c_ushort,
    destx: ::std::os::raw::c_short,
    desty: ::std::os::raw::c_short,
    destw: ::std::os::raw::c_ushort,
    desth: ::std::os::raw::c_ushort,
    cliprects: *mut VARectangle,
    number_cliprects: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaQueryImageFormats(
    ctx: VADriverContextP,
    format_list: *mut VAImageFormat,
    num_formats: *mut ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaCreateImage(
    ctx: VADriverContextP,
    format: *mut VAImageFormat,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    image: *mut VAImage,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaDeriveImage(
    ctx: VADriverContextP,
    surface: VASurfaceID,
    image: *mut VAImage,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaDestroyImage(ctx: VADriverContextP, image: VAImageID) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaSetImagePalette(
    ctx: VADriverContextP,
    image: VAImageID,
    palette: *mut ::std::os::raw::c_uchar,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaGetImage(
    ctx: VADriverContextP,
    surface: VASurfaceID,
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
    width: ::std::os::raw::c_uint,
    height: ::std::os::raw::c_uint,
    image: VAImageID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaPutImage(
    ctx: VADriverContextP,
    surface: VASurfaceID,
    image: VAImageID,
    src_x: ::std::os::raw::c_int,
    src_y: ::std::os::raw::c_int,
    src_width: ::std::os::raw::c_uint,
    src_height: ::std::os::raw::c_uint,
    dest_x: ::std::os::raw::c_int,
    dest_y: ::std::os::raw::c_int,
    dest_width: ::std::os::raw::c_uint,
    dest_height: ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaQuerySubpictureFormats(
    ctx: VADriverContextP,
    format_list: *mut VAImageFormat,
    flags: *mut ::std::os::raw::c_uint,
    num_formats: *mut ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaCreateSubpicture(
    ctx: VADriverContextP,
    image: VAImageID,
    subpicture: *mut VASubpictureID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaDestroySubpicture(
    ctx: VADriverContextP,
    subpicture: VASubpictureID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaSetSubpictureImage(
    ctx: VADriverContextP,
    subpicture: VASubpictureID,
    image: VAImageID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaSetSubpictureChromakey(
    ctx: VADriverContextP,
    subpicture: VASubpictureID,
    chromakey_min: ::std::os::raw::c_uint,
    chromakey_max: ::std::os::raw::c_uint,
    chromakey_mask: ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaSetSubpictureGlobalAlpha(
    ctx: VADriverContextP,
    subpicture: VASubpictureID,
    global_alpha: f32,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaAssociateSubpicture(
    ctx: VADriverContextP,
    subpicture: VASubpictureID,
    target_surfaces: *mut VASurfaceID,
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
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaDeassociateSubpicture(
    ctx: VADriverContextP,
    subpicture: VASubpictureID,
    target_surfaces: *mut VASurfaceID,
    num_surfaces: ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaQueryDisplayAttributes(
    ctx: VADriverContextP,
    attr_list: *mut VADisplayAttribute,
    num_attributes: *mut ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaGetDisplayAttributes(
    ctx: VADriverContextP,
    attr_list: *mut VADisplayAttribute,
    num_attributes: ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaSetDisplayAttributes(
    ctx: VADriverContextP,
    attr_list: *mut VADisplayAttribute,
    num_attributes: ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaBufferInfo(
    ctx: VADriverContextP,
    buf_id: VABufferID,
    type_: *mut VABufferType,
    size: *mut ::std::os::raw::c_uint,
    num_elements: *mut ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaLockSurface(
    ctx: VADriverContextP,
    surface: VASurfaceID,
    fourcc: *mut ::std::os::raw::c_uint,
    luma_stride: *mut ::std::os::raw::c_uint,
    chroma_u_stride: *mut ::std::os::raw::c_uint,
    chroma_v_stride: *mut ::std::os::raw::c_uint,
    luma_offset: *mut ::std::os::raw::c_uint,
    chroma_u_offset: *mut ::std::os::raw::c_uint,
    chroma_v_offset: *mut ::std::os::raw::c_uint,
    buffer_name: *mut ::std::os::raw::c_uint,
    buffer: *mut *mut ::std::os::raw::c_void,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaUnlockSurface(ctx: VADriverContextP, surface: VASurfaceID) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaGetSurfaceAttributes(
    dpy: VADriverContextP,
    config: VAConfigID,
    attrib_list: *mut VASurfaceAttrib,
    num_attribs: ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaCreateSurfaces2(
    ctx: VADriverContextP,
    format: ::std::os::raw::c_uint,
    width: ::std::os::raw::c_uint,
    height: ::std::os::raw::c_uint,
    surfaces: *mut VASurfaceID,
    num_surfaces: ::std::os::raw::c_uint,
    attrib_list: *mut VASurfaceAttrib,
    num_attribs: ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaQuerySurfaceAttributes(
    dpy: VADriverContextP,
    config: VAConfigID,
    attrib_list: *mut VASurfaceAttrib,
    num_attribs: *mut ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaAcquireBufferHandle(
    ctx: VADriverContextP,
    buf_id: VABufferID,
    buf_info: *mut VABufferInfo,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaReleaseBufferHandle(ctx: VADriverContextP, buf_id: VABufferID) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaCreateMFContext(
    ctx: VADriverContextP,
    mfe_context: *mut VAMFContextID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaMFAddContext(
    ctx: VADriverContextP,
    mf_context: VAMFContextID,
    context: VAContextID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaMFReleaseContext(
    ctx: VADriverContextP,
    mf_context: VAMFContextID,
    context: VAContextID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaMFSubmit(
    ctx: VADriverContextP,
    mf_context: VAMFContextID,
    contexts: *mut VAContextID,
    num_contexts: ::std::os::raw::c_int,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaCreateBuffer2(
    ctx: VADriverContextP,
    context: VAContextID,
    type_: VABufferType,
    width: ::std::os::raw::c_uint,
    height: ::std::os::raw::c_uint,
    unit_size: *mut ::std::os::raw::c_uint,
    pitch: *mut ::std::os::raw::c_uint,
    buf_id: *mut VABufferID,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaQueryProcessingRate(
    ctx: VADriverContextP,
    config_id: VAConfigID,
    proc_buf: *mut VAProcessingRateParameter,
    processing_rate: *mut ::std::os::raw::c_uint,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}
pub extern "C" fn vaExportSurfaceHandle(
    ctx: VADriverContextP,
    surface_id: VASurfaceID,
    mem_type: u32,
    flags: u32,
    descriptor: *mut ::std::os::raw::c_void,
) -> VAStatus {
    dbg!();
    VA_STATUS_ERROR_UNKNOWN as _
}

#![warn(clippy::all)]

mod debug;
mod driver;
mod va_backend;
use va_backend as va;
mod vdpau;
mod vdpau_loader;
mod vdpau_x11;
mod xlib;

const VDPAU_MAX_PROFILES: usize = 12;
const VDPAU_MAX_ENTRYPOINTS: usize = 5;
const VDPAU_MAX_CONFIG_ATTRIBUTES: usize = 10;
const VDPAU_MAX_IMAGE_FORMATS: usize = 10;
const VDPAU_MAX_SUBPICTURE_FORMATS: usize = 6;
const VDPAU_MAX_DISPLAY_ATTRIBUTES: usize = 6;

pub(crate) fn va_driver_init(ctx: va::VADriverContextP) -> Result<(), va::VAStatus> {
    let mut ctx = unsafe { &mut *ctx };

    let mut driver_data = driver::Driver::new(ctx)?;
    ctx.pDriverData = &mut driver_data as *mut _ as _;
    std::mem::forget(driver_data);

    ctx.version_major = 1;
    ctx.version_minor = 8;
    ctx.max_profiles = VDPAU_MAX_PROFILES as _;
    ctx.max_entrypoints = VDPAU_MAX_ENTRYPOINTS as _;
    ctx.max_attributes = VDPAU_MAX_CONFIG_ATTRIBUTES as _;
    ctx.max_image_formats = VDPAU_MAX_IMAGE_FORMATS as _;
    ctx.max_subpic_formats = VDPAU_MAX_SUBPICTURE_FORMATS as _;
    ctx.max_display_attributes = VDPAU_MAX_DISPLAY_ATTRIBUTES as _;
    ctx.str_vendor = b"nvidia-va-driver\0".as_ptr() as _;

    let vtable = unsafe { &mut *ctx.vtable };
    vtable.vaTerminate = Some(terminate);
    vtable.vaQueryConfigProfiles = Some(debug::vaQueryConfigProfiles);
    vtable.vaQueryConfigEntrypoints = Some(debug::vaQueryConfigEntrypoints);
    vtable.vaGetConfigAttributes = Some(debug::vaGetConfigAttributes);
    vtable.vaCreateConfig = Some(debug::vaCreateConfig);
    vtable.vaDestroyConfig = Some(debug::vaDestroyConfig);
    vtable.vaQueryConfigAttributes = Some(debug::vaQueryConfigAttributes);
    vtable.vaCreateSurfaces = Some(debug::vaCreateSurfaces);
    vtable.vaDestroySurfaces = Some(debug::vaDestroySurfaces);
    vtable.vaCreateContext = Some(debug::vaCreateContext);
    vtable.vaDestroyContext = Some(debug::vaDestroyContext);
    vtable.vaCreateBuffer = Some(debug::vaCreateBuffer);
    vtable.vaBufferSetNumElements = Some(debug::vaBufferSetNumElements);
    vtable.vaMapBuffer = Some(debug::vaMapBuffer);
    vtable.vaUnmapBuffer = Some(debug::vaUnmapBuffer);
    vtable.vaDestroyBuffer = Some(debug::vaDestroyBuffer);
    vtable.vaBeginPicture = Some(debug::vaBeginPicture);
    vtable.vaRenderPicture = Some(debug::vaRenderPicture);
    vtable.vaEndPicture = Some(debug::vaEndPicture);
    vtable.vaSyncSurface = Some(debug::vaSyncSurface);
    vtable.vaQuerySurfaceStatus = Some(debug::vaQuerySurfaceStatus);
    vtable.vaQuerySurfaceError = Some(debug::vaQuerySurfaceError);
    vtable.vaPutSurface = Some(debug::vaPutSurface);
    vtable.vaQueryImageFormats = Some(debug::vaQueryImageFormats);
    vtable.vaCreateImage = Some(debug::vaCreateImage);
    vtable.vaDeriveImage = Some(debug::vaDeriveImage);
    vtable.vaDestroyImage = Some(debug::vaDestroyImage);
    vtable.vaSetImagePalette = Some(debug::vaSetImagePalette);
    vtable.vaGetImage = Some(debug::vaGetImage);
    vtable.vaPutImage = Some(debug::vaPutImage);
    vtable.vaQuerySubpictureFormats = Some(debug::vaQuerySubpictureFormats);
    vtable.vaCreateSubpicture = Some(debug::vaCreateSubpicture);
    vtable.vaDestroySubpicture = Some(debug::vaDestroySubpicture);
    vtable.vaSetSubpictureImage = Some(debug::vaSetSubpictureImage);
    vtable.vaSetSubpictureChromakey = Some(debug::vaSetSubpictureChromakey);
    vtable.vaSetSubpictureGlobalAlpha = Some(debug::vaSetSubpictureGlobalAlpha);
    vtable.vaAssociateSubpicture = Some(debug::vaAssociateSubpicture);
    vtable.vaDeassociateSubpicture = Some(debug::vaDeassociateSubpicture);
    vtable.vaQueryDisplayAttributes = Some(debug::vaQueryDisplayAttributes);
    vtable.vaGetDisplayAttributes = Some(debug::vaGetDisplayAttributes);
    vtable.vaSetDisplayAttributes = Some(debug::vaSetDisplayAttributes);
    vtable.vaBufferInfo = Some(debug::vaBufferInfo);
    vtable.vaLockSurface = Some(debug::vaLockSurface);
    vtable.vaUnlockSurface = Some(debug::vaUnlockSurface);
    vtable.vaGetSurfaceAttributes = Some(debug::vaGetSurfaceAttributes);
    vtable.vaCreateSurfaces2 = Some(debug::vaCreateSurfaces2);
    vtable.vaQuerySurfaceAttributes = Some(debug::vaQuerySurfaceAttributes);
    vtable.vaAcquireBufferHandle = Some(debug::vaAcquireBufferHandle);
    vtable.vaReleaseBufferHandle = Some(debug::vaReleaseBufferHandle);
    vtable.vaCreateMFContext = Some(debug::vaCreateMFContext);
    vtable.vaMFAddContext = Some(debug::vaMFAddContext);
    vtable.vaMFReleaseContext = Some(debug::vaMFReleaseContext);
    vtable.vaMFSubmit = Some(debug::vaMFSubmit);
    vtable.vaCreateBuffer2 = Some(debug::vaCreateBuffer2);
    vtable.vaQueryProcessingRate = Some(debug::vaQueryProcessingRate);
    vtable.vaExportSurfaceHandle = Some(debug::vaExportSurfaceHandle);
    Ok(())
}

#[no_mangle]
pub extern "C" fn __vaDriverInit_1_8(ctx: va::VADriverContextP) -> va::VAStatus {
    match va_driver_init(ctx) {
        Ok(_) => va::VA_STATUS_SUCCESS as _,
        Err(e) => e,
    }
}

pub(crate) extern "C" fn terminate(ctx: va::VADriverContextP) -> va::VAStatus {
    unsafe { ((*ctx).pDriverData as *mut driver::Driver).drop_in_place() };
    va::VA_STATUS_SUCCESS as _
}

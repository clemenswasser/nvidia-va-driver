#![warn(clippy::all)]

mod vtable;
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
    vtable.vaQueryConfigProfiles = Some(vtable::vaQueryConfigProfiles);
    vtable.vaQueryConfigEntrypoints = Some(vtable::vaQueryConfigEntrypoints);
    vtable.vaGetConfigAttributes = Some(vtable::vaGetConfigAttributes);
    vtable.vaCreateConfig = Some(vtable::vaCreateConfig);
    vtable.vaDestroyConfig = Some(vtable::vaDestroyConfig);
    vtable.vaQueryConfigAttributes = Some(vtable::vaQueryConfigAttributes);
    vtable.vaCreateSurfaces = Some(vtable::vaCreateSurfaces);
    vtable.vaDestroySurfaces = Some(vtable::vaDestroySurfaces);
    vtable.vaCreateContext = Some(vtable::vaCreateContext);
    vtable.vaDestroyContext = Some(vtable::vaDestroyContext);
    vtable.vaCreateBuffer = Some(vtable::vaCreateBuffer);
    vtable.vaBufferSetNumElements = Some(vtable::vaBufferSetNumElements);
    vtable.vaMapBuffer = Some(vtable::vaMapBuffer);
    vtable.vaUnmapBuffer = Some(vtable::vaUnmapBuffer);
    vtable.vaDestroyBuffer = Some(vtable::vaDestroyBuffer);
    vtable.vaBeginPicture = Some(vtable::vaBeginPicture);
    vtable.vaRenderPicture = Some(vtable::vaRenderPicture);
    vtable.vaEndPicture = Some(vtable::vaEndPicture);
    vtable.vaSyncSurface = Some(vtable::vaSyncSurface);
    vtable.vaQuerySurfaceStatus = Some(vtable::vaQuerySurfaceStatus);
    vtable.vaQuerySurfaceError = Some(vtable::vaQuerySurfaceError);
    vtable.vaPutSurface = Some(vtable::vaPutSurface);
    vtable.vaQueryImageFormats = Some(vtable::vaQueryImageFormats);
    vtable.vaCreateImage = Some(vtable::vaCreateImage);
    vtable.vaDeriveImage = Some(vtable::vaDeriveImage);
    vtable.vaDestroyImage = Some(vtable::vaDestroyImage);
    vtable.vaSetImagePalette = Some(vtable::vaSetImagePalette);
    vtable.vaGetImage = Some(vtable::vaGetImage);
    vtable.vaPutImage = Some(vtable::vaPutImage);
    vtable.vaQuerySubpictureFormats = Some(vtable::vaQuerySubpictureFormats);
    vtable.vaCreateSubpicture = Some(vtable::vaCreateSubpicture);
    vtable.vaDestroySubpicture = Some(vtable::vaDestroySubpicture);
    vtable.vaSetSubpictureImage = Some(vtable::vaSetSubpictureImage);
    vtable.vaSetSubpictureChromakey = Some(vtable::vaSetSubpictureChromakey);
    vtable.vaSetSubpictureGlobalAlpha = Some(vtable::vaSetSubpictureGlobalAlpha);
    vtable.vaAssociateSubpicture = Some(vtable::vaAssociateSubpicture);
    vtable.vaDeassociateSubpicture = Some(vtable::vaDeassociateSubpicture);
    vtable.vaQueryDisplayAttributes = Some(vtable::vaQueryDisplayAttributes);
    vtable.vaGetDisplayAttributes = Some(vtable::vaGetDisplayAttributes);
    vtable.vaSetDisplayAttributes = Some(vtable::vaSetDisplayAttributes);
    vtable.vaBufferInfo = Some(vtable::vaBufferInfo);
    vtable.vaLockSurface = Some(vtable::vaLockSurface);
    vtable.vaUnlockSurface = Some(vtable::vaUnlockSurface);
    vtable.vaGetSurfaceAttributes = Some(vtable::vaGetSurfaceAttributes);
    vtable.vaCreateSurfaces2 = Some(vtable::vaCreateSurfaces2);
    vtable.vaQuerySurfaceAttributes = Some(vtable::vaQuerySurfaceAttributes);
    vtable.vaAcquireBufferHandle = Some(vtable::vaAcquireBufferHandle);
    vtable.vaReleaseBufferHandle = Some(vtable::vaReleaseBufferHandle);
    vtable.vaCreateMFContext = Some(vtable::vaCreateMFContext);
    vtable.vaMFAddContext = Some(vtable::vaMFAddContext);
    vtable.vaMFReleaseContext = Some(vtable::vaMFReleaseContext);
    vtable.vaMFSubmit = Some(vtable::vaMFSubmit);
    vtable.vaCreateBuffer2 = Some(vtable::vaCreateBuffer2);
    vtable.vaQueryProcessingRate = Some(vtable::vaQueryProcessingRate);
    vtable.vaExportSurfaceHandle = Some(vtable::vaExportSurfaceHandle);
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

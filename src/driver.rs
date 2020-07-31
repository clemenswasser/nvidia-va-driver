use crate::va;
use crate::vdpau;
use crate::vdpau_loader;
use crate::vdpau_x11;
use crate::xlib;

pub(crate) struct Driver {
    pub(crate) vdpau_loader: vdpau_loader::VdpauLoader,
    pub(crate) vdpau_device: vdpau::VdpDevice,
    pub(crate) x11_display: *mut xlib::Display,
    pub(crate) x11_screen: usize,
}

impl Driver {
    pub(crate) fn new(ctx: &mut va::VADriverContext) -> Result<Self, va::VAStatus> {
        let mut x11_display = ctx.native_dpy as *mut xlib::Display;
        let x11_screen = ctx.x11_screen;

        let x11_display_name = unsafe { xlib::XDisplayString(x11_display) };
        let dedicated_display = unsafe { xlib::XOpenDisplay(x11_display_name) };
        if dedicated_display.is_null() {
            eprintln!("Failed to create dedicated X11 display!");
        } else {
            x11_display = dedicated_display;
        }

        let mut vdpau_device = vdpau::VDP_INVALID_HANDLE;
        let mut vdp_get_proc_address = unsafe { std::mem::zeroed::<vdpau::VdpGetProcAddress>() };

        if !unsafe {
            vdpau_x11::vdp_device_create_x11(
                x11_display as _,
                x11_screen,
                &mut vdpau_device,
                &mut vdp_get_proc_address,
            )
        }
        .eq(&vdpau::VdpStatus_VDP_STATUS_OK)
        {
            return Err(va::VA_STATUS_ERROR_UNKNOWN as _);
        }

        let vdpau_loader = vdpau_loader::VdpauLoader::load(&vdp_get_proc_address, vdpau_device)?;

        let mut api_version = 0;
        let result = unsafe { vdpau_loader.get_api_version.unwrap()(&mut api_version) };
        if result != vdpau::VdpStatus_VDP_STATUS_OK && result != vdpau::VDPAU_VERSION {
            eprintln!("Failed to get_api_version");
            return Err(va::VA_STATUS_ERROR_UNKNOWN as _);
        }

        let mut information_string = std::ptr::null();
        if !unsafe { vdpau_loader.get_information_string.unwrap()(&mut information_string) }
            .eq(&vdpau::VdpStatus_VDP_STATUS_OK)
        {
            eprintln!("Failed to get_information_string");
            return Err(va::VA_STATUS_ERROR_UNKNOWN as _);
        }
        let info = unsafe { std::ffi::CStr::from_ptr(information_string) }
            .to_str()
            .unwrap();
        println!("{}", info);
        
        Ok(Self {
            vdpau_loader,
            vdpau_device,
            x11_display,
            x11_screen: x11_screen as _,
        })
    }
}

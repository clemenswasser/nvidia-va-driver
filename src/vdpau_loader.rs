#![allow(dead_code)]

use crate::va;
use crate::vdpau;

const VDP_FUNC_ID_GET_ERROR_STRING: vdpau::VdpFuncId = 0;
const VDP_FUNC_ID_GET_PROC_ADDRESS: vdpau::VdpFuncId = 1;
const VDP_FUNC_ID_GET_API_VERSION: vdpau::VdpFuncId = 2;
const VDP_FUNC_ID_GET_INFORMATION_STRING: vdpau::VdpFuncId = 4;
const VDP_FUNC_ID_DEVICE_DESTROY: vdpau::VdpFuncId = 5;
const VDP_FUNC_ID_VIDEO_SURFACE_QUERY_CAPABILITIES: vdpau::VdpFuncId = 7;
const VDP_FUNC_ID_VIDEO_SURFACE_QUERY_GET_PUT_BITS_Y_CB_CR_CAPABILITIES: vdpau::VdpFuncId = 8;
const VDP_FUNC_ID_VIDEO_SURFACE_CREATE: vdpau::VdpFuncId = 9;
const VDP_FUNC_ID_VIDEO_SURFACE_DESTROY: vdpau::VdpFuncId = 10;
const VDP_FUNC_ID_VIDEO_SURFACE_GET_PARAMETERS: vdpau::VdpFuncId = 11;
const VDP_FUNC_ID_VIDEO_SURFACE_GET_BITS_Y_CB_CR: vdpau::VdpFuncId = 12;
const VDP_FUNC_ID_VIDEO_SURFACE_PUT_BITS_Y_CB_CR: vdpau::VdpFuncId = 13;
const VDP_FUNC_ID_OUTPUT_SURFACE_QUERY_CAPABILITIES: vdpau::VdpFuncId = 14;
const VDP_FUNC_ID_OUTPUT_SURFACE_QUERY_GET_PUT_BITS_NATIVE_CAPABILITIES: vdpau::VdpFuncId = 15;
const VDP_FUNC_ID_OUTPUT_SURFACE_QUERY_PUT_BITS_INDEXED_CAPABILITIES: vdpau::VdpFuncId = 16;
const VDP_FUNC_ID_OUTPUT_SURFACE_QUERY_PUT_BITS_Y_CB_CR_CAPABILITIES: vdpau::VdpFuncId = 17;
const VDP_FUNC_ID_OUTPUT_SURFACE_CREATE: vdpau::VdpFuncId = 18;
const VDP_FUNC_ID_OUTPUT_SURFACE_DESTROY: vdpau::VdpFuncId = 19;
const VDP_FUNC_ID_OUTPUT_SURFACE_GET_PARAMETERS: vdpau::VdpFuncId = 20;
const VDP_FUNC_ID_OUTPUT_SURFACE_GET_BITS_NATIVE: vdpau::VdpFuncId = 21;
const VDP_FUNC_ID_OUTPUT_SURFACE_PUT_BITS_NATIVE: vdpau::VdpFuncId = 22;
const VDP_FUNC_ID_OUTPUT_SURFACE_PUT_BITS_INDEXED: vdpau::VdpFuncId = 23;
const VDP_FUNC_ID_OUTPUT_SURFACE_PUT_BITS_Y_CB_CR: vdpau::VdpFuncId = 24;
const VDP_FUNC_ID_BITMAP_SURFACE_QUERY_CAPABILITIES: vdpau::VdpFuncId = 25;
const VDP_FUNC_ID_BITMAP_SURFACE_CREATE: vdpau::VdpFuncId = 26;
const VDP_FUNC_ID_BITMAP_SURFACE_DESTROY: vdpau::VdpFuncId = 27;
const VDP_FUNC_ID_BITMAP_SURFACE_GET_PARAMETERS: vdpau::VdpFuncId = 28;
const VDP_FUNC_ID_BITMAP_SURFACE_PUT_BITS_NATIVE: vdpau::VdpFuncId = 29;
const VDP_FUNC_ID_OUTPUT_SURFACE_RENDER_OUTPUT_SURFACE: vdpau::VdpFuncId = 33;
const VDP_FUNC_ID_OUTPUT_SURFACE_RENDER_BITMAP_SURFACE: vdpau::VdpFuncId = 34;
const VDP_FUNC_ID_DECODER_QUERY_CAPABILITIES: vdpau::VdpFuncId = 36;
const VDP_FUNC_ID_DECODER_CREATE: vdpau::VdpFuncId = 37;
const VDP_FUNC_ID_DECODER_DESTROY: vdpau::VdpFuncId = 38;
const VDP_FUNC_ID_DECODER_GET_PARAMETERS: vdpau::VdpFuncId = 39;
const VDP_FUNC_ID_DECODER_RENDER: vdpau::VdpFuncId = 40;
const VDP_FUNC_ID_VIDEO_MIXER_QUERY_FEATURE_SUPPORT: vdpau::VdpFuncId = 41;
const VDP_FUNC_ID_VIDEO_MIXER_QUERY_PARAMETER_SUPPORT: vdpau::VdpFuncId = 42;
const VDP_FUNC_ID_VIDEO_MIXER_QUERY_ATTRIBUTE_SUPPORT: vdpau::VdpFuncId = 43;
const VDP_FUNC_ID_VIDEO_MIXER_QUERY_PARAMETER_VALUE_RANGE: vdpau::VdpFuncId = 44;
const VDP_FUNC_ID_VIDEO_MIXER_QUERY_ATTRIBUTE_VALUE_RANGE: vdpau::VdpFuncId = 45;
const VDP_FUNC_ID_VIDEO_MIXER_CREATE: vdpau::VdpFuncId = 46;
const VDP_FUNC_ID_VIDEO_MIXER_SET_FEATURE_ENABLES: vdpau::VdpFuncId = 47;
const VDP_FUNC_ID_VIDEO_MIXER_SET_ATTRIBUTE_VALUES: vdpau::VdpFuncId = 48;
const VDP_FUNC_ID_VIDEO_MIXER_GET_FEATURE_SUPPORT: vdpau::VdpFuncId = 49;
const VDP_FUNC_ID_VIDEO_MIXER_GET_FEATURE_ENABLES: vdpau::VdpFuncId = 50;
const VDP_FUNC_ID_VIDEO_MIXER_GET_PARAMETER_VALUES: vdpau::VdpFuncId = 51;
const VDP_FUNC_ID_VIDEO_MIXER_GET_ATTRIBUTE_VALUES: vdpau::VdpFuncId = 52;
const VDP_FUNC_ID_VIDEO_MIXER_DESTROY: vdpau::VdpFuncId = 53;
const VDP_FUNC_ID_VIDEO_MIXER_RENDER: vdpau::VdpFuncId = 54;
const VDP_FUNC_ID_PRESENTATION_QUEUE_TARGET_DESTROY: vdpau::VdpFuncId = 55;
const VDP_FUNC_ID_PRESENTATION_QUEUE_CREATE: vdpau::VdpFuncId = 56;
const VDP_FUNC_ID_PRESENTATION_QUEUE_DESTROY: vdpau::VdpFuncId = 57;
const VDP_FUNC_ID_PRESENTATION_QUEUE_SET_BACKGROUND_COLOR: vdpau::VdpFuncId = 58;
const VDP_FUNC_ID_PRESENTATION_QUEUE_GET_BACKGROUND_COLOR: vdpau::VdpFuncId = 59;
const VDP_FUNC_ID_PRESENTATION_QUEUE_GET_TIME: vdpau::VdpFuncId = 62;
const VDP_FUNC_ID_PRESENTATION_QUEUE_DISPLAY: vdpau::VdpFuncId = 63;
const VDP_FUNC_ID_PRESENTATION_QUEUE_BLOCK_UNTIL_SURFACE_IDLE: vdpau::VdpFuncId = 64;
const VDP_FUNC_ID_PRESENTATION_QUEUE_QUERY_SURFACE_STATUS: vdpau::VdpFuncId = 65;
const VDP_FUNC_ID_PREEMPTION_CALLBACK_REGISTER: vdpau::VdpFuncId = 66;

pub(crate) struct VdpauLoader {
    pub(crate) get_error_string: vdpau::VdpGetErrorString,
    pub(crate) get_proc_address: vdpau::VdpGetProcAddress,
    pub(crate) get_api_version: vdpau::VdpGetApiVersion,
    pub(crate) get_information_string: vdpau::VdpGetInformationString,
    pub(crate) device_destroy: vdpau::VdpDeviceDestroy,
    pub(crate) video_surface_query_capabilities: vdpau::VdpVideoSurfaceQueryCapabilities,
    pub(crate) video_surface_query_get_put_bits_y_cb_cr_capabilities:
        vdpau::VdpVideoSurfaceQueryGetPutBitsYCbCrCapabilities,
    pub(crate) video_surface_create: vdpau::VdpVideoSurfaceCreate,
    pub(crate) video_surface_destroy: vdpau::VdpVideoSurfaceDestroy,
    pub(crate) video_surface_get_parameters: vdpau::VdpVideoSurfaceGetParameters,
    pub(crate) video_surface_get_bits_y_cb_cr: vdpau::VdpVideoSurfaceGetBitsYCbCr,
    pub(crate) video_surface_put_bits_y_cb_cr: vdpau::VdpVideoSurfacePutBitsYCbCr,
    pub(crate) output_surface_query_capabilities: vdpau::VdpOutputSurfaceQueryCapabilities,
    pub(crate) output_surface_query_get_put_bits_native_capabilities:
        vdpau::VdpOutputSurfaceQueryGetPutBitsNativeCapabilities,
    pub(crate) output_surface_query_put_bits_indexed_capabilities:
        vdpau::VdpOutputSurfaceQueryPutBitsIndexedCapabilities,
    pub(crate) output_surface_query_put_bits_y_cb_cr_capabilities:
        vdpau::VdpOutputSurfaceQueryPutBitsYCbCrCapabilities,
    pub(crate) output_surface_create: vdpau::VdpOutputSurfaceCreate,
    pub(crate) output_surface_destroy: vdpau::VdpOutputSurfaceDestroy,
    pub(crate) output_surface_get_parameters: vdpau::VdpOutputSurfaceGetParameters,
    pub(crate) output_surface_get_bits_native: vdpau::VdpOutputSurfaceGetBitsNative,
    pub(crate) output_surface_put_bits_native: vdpau::VdpOutputSurfacePutBitsNative,
    pub(crate) output_surface_put_bits_indexed: vdpau::VdpOutputSurfacePutBitsIndexed,
    pub(crate) output_surface_put_bits_y_cb_cr: vdpau::VdpOutputSurfacePutBitsYCbCr,
    pub(crate) bitmap_surface_query_capabilities: vdpau::VdpBitmapSurfaceQueryCapabilities,
    pub(crate) bitmap_surface_create: vdpau::VdpBitmapSurfaceCreate,
    pub(crate) bitmap_surface_destroy: vdpau::VdpBitmapSurfaceDestroy,
    pub(crate) bitmap_surface_get_parameters: vdpau::VdpBitmapSurfaceGetParameters,
    pub(crate) bitmap_surface_put_bits_native: vdpau::VdpBitmapSurfacePutBitsNative,
    pub(crate) output_surface_render_output_surface: vdpau::VdpOutputSurfaceRenderOutputSurface,
    pub(crate) output_surface_render_bitmap_surface: vdpau::VdpOutputSurfaceRenderBitmapSurface,
    pub(crate) decoder_query_capabilities: vdpau::VdpDecoderQueryCapabilities,
    pub(crate) decoder_create: vdpau::VdpDecoderCreate,
    pub(crate) decoder_destroy: vdpau::VdpDecoderDestroy,
    pub(crate) decoder_get_parameters: vdpau::VdpDecoderGetParameters,
    pub(crate) decoder_render: vdpau::VdpDecoderRender,
    pub(crate) video_mixer_query_feature_support: vdpau::VdpVideoMixerQueryFeatureSupport,
    pub(crate) video_mixer_query_parameter_support: vdpau::VdpVideoMixerQueryParameterSupport,
    pub(crate) video_mixer_query_attribute_support: vdpau::VdpVideoMixerQueryAttributeSupport,
    pub(crate) video_mixer_query_parameter_value_range:
        vdpau::VdpVideoMixerQueryParameterValueRange,
    pub(crate) video_mixer_query_attribute_value_range:
        vdpau::VdpVideoMixerQueryAttributeValueRange,
    pub(crate) video_mixer_create: vdpau::VdpVideoMixerCreate,
    pub(crate) video_mixer_set_feature_enables: vdpau::VdpVideoMixerSetFeatureEnables,
    pub(crate) video_mixer_set_attribute_values: vdpau::VdpVideoMixerSetAttributeValues,
    pub(crate) video_mixer_get_feature_support: vdpau::VdpVideoMixerGetFeatureSupport,
    pub(crate) video_mixer_get_feature_enables: vdpau::VdpVideoMixerGetFeatureEnables,
    pub(crate) video_mixer_get_parameter_values: vdpau::VdpVideoMixerGetParameterValues,
    pub(crate) video_mixer_get_attribute_values: vdpau::VdpVideoMixerGetAttributeValues,
    pub(crate) video_mixer_destroy: vdpau::VdpVideoMixerDestroy,
    pub(crate) video_mixer_render: vdpau::VdpVideoMixerRender,
    pub(crate) presentation_queue_target_destroy: vdpau::VdpPresentationQueueTargetDestroy,
    pub(crate) presentation_queue_create: vdpau::VdpPresentationQueueCreate,
    pub(crate) presentation_queue_destroy: vdpau::VdpPresentationQueueDestroy,
    pub(crate) presentation_queue_set_background_color:
        vdpau::VdpPresentationQueueSetBackgroundColor,
    pub(crate) presentation_queue_get_background_color:
        vdpau::VdpPresentationQueueGetBackgroundColor,
    pub(crate) presentation_queue_get_time: vdpau::VdpPresentationQueueGetTime,
    pub(crate) presentation_queue_display: vdpau::VdpPresentationQueueDisplay,
    pub(crate) presentation_queue_block_until_surface_idle:
        vdpau::VdpPresentationQueueBlockUntilSurfaceIdle,
    pub(crate) presentation_queue_query_surface_status:
        vdpau::VdpPresentationQueueQuerySurfaceStatus,
    pub(crate) preemption_callback_register: vdpau::VdpPreemptionCallbackRegister,
}

impl VdpauLoader {
    pub(crate) fn load(
        vdp_get_proc_address: &vdpau::VdpGetProcAddress,
        vdp_device: u32,
    ) -> Result<Self, va::VAStatus> {
        let get_error_string = Self::load_t::<vdpau::VdpGetErrorString>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_GET_ERROR_STRING,
        )?;
        let get_proc_address = Self::load_t::<vdpau::VdpGetProcAddress>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_GET_PROC_ADDRESS,
        )?;
        let get_api_version = Self::load_t::<vdpau::VdpGetApiVersion>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_GET_API_VERSION,
        )?;
        let get_information_string = Self::load_t::<vdpau::VdpGetInformationString>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_GET_INFORMATION_STRING,
        )?;
        let device_destroy = Self::load_t::<vdpau::VdpDeviceDestroy>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_DEVICE_DESTROY,
        )?;
        let video_surface_query_capabilities =
            Self::load_t::<vdpau::VdpVideoSurfaceQueryCapabilities>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_SURFACE_QUERY_CAPABILITIES,
            )?;
        let video_surface_query_get_put_bits_y_cb_cr_capabilities =
            Self::load_t::<vdpau::VdpVideoSurfaceQueryGetPutBitsYCbCrCapabilities>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_SURFACE_QUERY_GET_PUT_BITS_Y_CB_CR_CAPABILITIES,
            )?;
        let video_surface_create = Self::load_t::<vdpau::VdpVideoSurfaceCreate>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_SURFACE_CREATE,
        )?;
        let video_surface_destroy = Self::load_t::<vdpau::VdpVideoSurfaceDestroy>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_SURFACE_DESTROY,
        )?;
        let video_surface_get_parameters = Self::load_t::<vdpau::VdpVideoSurfaceGetParameters>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_SURFACE_GET_PARAMETERS,
        )?;
        let video_surface_get_bits_y_cb_cr = Self::load_t::<vdpau::VdpVideoSurfaceGetBitsYCbCr>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_SURFACE_GET_BITS_Y_CB_CR,
        )?;
        let video_surface_put_bits_y_cb_cr = Self::load_t::<vdpau::VdpVideoSurfacePutBitsYCbCr>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_SURFACE_PUT_BITS_Y_CB_CR,
        )?;
        let output_surface_query_capabilities =
            Self::load_t::<vdpau::VdpOutputSurfaceQueryCapabilities>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_OUTPUT_SURFACE_QUERY_CAPABILITIES,
            )?;
        let output_surface_query_get_put_bits_native_capabilities =
            Self::load_t::<vdpau::VdpOutputSurfaceQueryGetPutBitsNativeCapabilities>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_OUTPUT_SURFACE_QUERY_GET_PUT_BITS_NATIVE_CAPABILITIES,
            )?;
        let output_surface_query_put_bits_indexed_capabilities =
            Self::load_t::<vdpau::VdpOutputSurfaceQueryPutBitsIndexedCapabilities>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_OUTPUT_SURFACE_QUERY_PUT_BITS_INDEXED_CAPABILITIES,
            )?;
        let output_surface_query_put_bits_y_cb_cr_capabilities =
            Self::load_t::<vdpau::VdpOutputSurfaceQueryPutBitsYCbCrCapabilities>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_OUTPUT_SURFACE_QUERY_PUT_BITS_Y_CB_CR_CAPABILITIES,
            )?;
        let output_surface_create = Self::load_t::<vdpau::VdpOutputSurfaceCreate>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_OUTPUT_SURFACE_CREATE,
        )?;
        let output_surface_destroy = Self::load_t::<vdpau::VdpOutputSurfaceDestroy>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_OUTPUT_SURFACE_DESTROY,
        )?;
        let output_surface_get_parameters = Self::load_t::<vdpau::VdpOutputSurfaceGetParameters>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_OUTPUT_SURFACE_GET_PARAMETERS,
        )?;
        let output_surface_get_bits_native = Self::load_t::<vdpau::VdpOutputSurfaceGetBitsNative>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_OUTPUT_SURFACE_GET_BITS_NATIVE,
        )?;
        let output_surface_put_bits_native = Self::load_t::<vdpau::VdpOutputSurfacePutBitsNative>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_OUTPUT_SURFACE_PUT_BITS_NATIVE,
        )?;
        let output_surface_put_bits_indexed = Self::load_t::<vdpau::VdpOutputSurfacePutBitsIndexed>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_OUTPUT_SURFACE_PUT_BITS_INDEXED,
        )?;
        let output_surface_put_bits_y_cb_cr = Self::load_t::<vdpau::VdpOutputSurfacePutBitsYCbCr>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_OUTPUT_SURFACE_PUT_BITS_Y_CB_CR,
        )?;
        let bitmap_surface_query_capabilities =
            Self::load_t::<vdpau::VdpBitmapSurfaceQueryCapabilities>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_BITMAP_SURFACE_QUERY_CAPABILITIES,
            )?;
        let bitmap_surface_create = Self::load_t::<vdpau::VdpBitmapSurfaceCreate>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_BITMAP_SURFACE_CREATE,
        )?;
        let bitmap_surface_destroy = Self::load_t::<vdpau::VdpBitmapSurfaceDestroy>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_BITMAP_SURFACE_DESTROY,
        )?;
        let bitmap_surface_get_parameters = Self::load_t::<vdpau::VdpBitmapSurfaceGetParameters>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_BITMAP_SURFACE_GET_PARAMETERS,
        )?;
        let bitmap_surface_put_bits_native = Self::load_t::<vdpau::VdpBitmapSurfacePutBitsNative>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_BITMAP_SURFACE_PUT_BITS_NATIVE,
        )?;
        let output_surface_render_output_surface =
            Self::load_t::<vdpau::VdpOutputSurfaceRenderOutputSurface>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_OUTPUT_SURFACE_RENDER_OUTPUT_SURFACE,
            )?;
        let output_surface_render_bitmap_surface =
            Self::load_t::<vdpau::VdpOutputSurfaceRenderBitmapSurface>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_OUTPUT_SURFACE_RENDER_BITMAP_SURFACE,
            )?;
        let decoder_query_capabilities = Self::load_t::<vdpau::VdpDecoderQueryCapabilities>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_DECODER_QUERY_CAPABILITIES,
        )?;
        let decoder_create = Self::load_t::<vdpau::VdpDecoderCreate>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_DECODER_CREATE,
        )?;
        let decoder_destroy = Self::load_t::<vdpau::VdpDecoderDestroy>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_DECODER_DESTROY,
        )?;
        let decoder_get_parameters = Self::load_t::<vdpau::VdpDecoderGetParameters>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_DECODER_GET_PARAMETERS,
        )?;
        let decoder_render = Self::load_t::<vdpau::VdpDecoderRender>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_DECODER_RENDER,
        )?;
        let video_mixer_query_feature_support =
            Self::load_t::<vdpau::VdpVideoMixerQueryFeatureSupport>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_MIXER_QUERY_FEATURE_SUPPORT,
            )?;
        let video_mixer_query_parameter_support =
            Self::load_t::<vdpau::VdpVideoMixerQueryParameterSupport>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_MIXER_QUERY_PARAMETER_SUPPORT,
            )?;
        let video_mixer_query_attribute_support =
            Self::load_t::<vdpau::VdpVideoMixerQueryAttributeSupport>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_MIXER_QUERY_ATTRIBUTE_SUPPORT,
            )?;
        let video_mixer_query_parameter_value_range =
            Self::load_t::<vdpau::VdpVideoMixerQueryParameterValueRange>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_MIXER_QUERY_PARAMETER_VALUE_RANGE,
            )?;
        let video_mixer_query_attribute_value_range =
            Self::load_t::<vdpau::VdpVideoMixerQueryAttributeValueRange>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_MIXER_QUERY_ATTRIBUTE_VALUE_RANGE,
            )?;
        let video_mixer_create = Self::load_t::<vdpau::VdpVideoMixerCreate>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_MIXER_CREATE,
        )?;
        let video_mixer_set_feature_enables = Self::load_t::<vdpau::VdpVideoMixerSetFeatureEnables>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_MIXER_SET_FEATURE_ENABLES,
        )?;
        let video_mixer_set_attribute_values =
            Self::load_t::<vdpau::VdpVideoMixerSetAttributeValues>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_MIXER_SET_ATTRIBUTE_VALUES,
            )?;
        let video_mixer_get_feature_support = Self::load_t::<vdpau::VdpVideoMixerGetFeatureSupport>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_MIXER_GET_FEATURE_SUPPORT,
        )?;
        let video_mixer_get_feature_enables = Self::load_t::<vdpau::VdpVideoMixerGetFeatureEnables>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_MIXER_GET_FEATURE_ENABLES,
        )?;
        let video_mixer_get_parameter_values =
            Self::load_t::<vdpau::VdpVideoMixerGetParameterValues>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_MIXER_GET_PARAMETER_VALUES,
            )?;
        let video_mixer_get_attribute_values =
            Self::load_t::<vdpau::VdpVideoMixerGetAttributeValues>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_VIDEO_MIXER_GET_ATTRIBUTE_VALUES,
            )?;
        let video_mixer_destroy = Self::load_t::<vdpau::VdpVideoMixerDestroy>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_MIXER_DESTROY,
        )?;
        let video_mixer_render = Self::load_t::<vdpau::VdpVideoMixerRender>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_VIDEO_MIXER_RENDER,
        )?;
        let presentation_queue_target_destroy =
            Self::load_t::<vdpau::VdpPresentationQueueTargetDestroy>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_PRESENTATION_QUEUE_TARGET_DESTROY,
            )?;
        let presentation_queue_create = Self::load_t::<vdpau::VdpPresentationQueueCreate>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_PRESENTATION_QUEUE_CREATE,
        )?;
        let presentation_queue_destroy = Self::load_t::<vdpau::VdpPresentationQueueDestroy>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_PRESENTATION_QUEUE_DESTROY,
        )?;
        let presentation_queue_set_background_color =
            Self::load_t::<vdpau::VdpPresentationQueueSetBackgroundColor>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_PRESENTATION_QUEUE_SET_BACKGROUND_COLOR,
            )?;
        let presentation_queue_get_background_color =
            Self::load_t::<vdpau::VdpPresentationQueueGetBackgroundColor>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_PRESENTATION_QUEUE_GET_BACKGROUND_COLOR,
            )?;
        let presentation_queue_get_time = Self::load_t::<vdpau::VdpPresentationQueueGetTime>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_PRESENTATION_QUEUE_GET_TIME,
        )?;
        let presentation_queue_display = Self::load_t::<vdpau::VdpPresentationQueueDisplay>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_PRESENTATION_QUEUE_DISPLAY,
        )?;
        let presentation_queue_block_until_surface_idle =
            Self::load_t::<vdpau::VdpPresentationQueueBlockUntilSurfaceIdle>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_PRESENTATION_QUEUE_BLOCK_UNTIL_SURFACE_IDLE,
            )?;
        let presentation_queue_query_surface_status =
            Self::load_t::<vdpau::VdpPresentationQueueQuerySurfaceStatus>(
                vdp_get_proc_address,
                vdp_device,
                VDP_FUNC_ID_PRESENTATION_QUEUE_QUERY_SURFACE_STATUS,
            )?;
        let preemption_callback_register = Self::load_t::<vdpau::VdpPreemptionCallbackRegister>(
            vdp_get_proc_address,
            vdp_device,
            VDP_FUNC_ID_PREEMPTION_CALLBACK_REGISTER,
        )?;

        Ok(Self {
            get_error_string,
            get_proc_address,
            get_api_version,
            get_information_string,
            device_destroy,
            video_surface_query_capabilities,
            video_surface_query_get_put_bits_y_cb_cr_capabilities,
            video_surface_create,
            video_surface_destroy,
            video_surface_get_parameters,
            video_surface_get_bits_y_cb_cr,
            video_surface_put_bits_y_cb_cr,
            output_surface_query_capabilities,
            output_surface_query_get_put_bits_native_capabilities,
            output_surface_query_put_bits_indexed_capabilities,
            output_surface_query_put_bits_y_cb_cr_capabilities,
            output_surface_create,
            output_surface_destroy,
            output_surface_get_parameters,
            output_surface_get_bits_native,
            output_surface_put_bits_native,
            output_surface_put_bits_indexed,
            output_surface_put_bits_y_cb_cr,
            bitmap_surface_query_capabilities,
            bitmap_surface_create,
            bitmap_surface_destroy,
            bitmap_surface_get_parameters,
            bitmap_surface_put_bits_native,
            output_surface_render_output_surface,
            output_surface_render_bitmap_surface,
            decoder_query_capabilities,
            decoder_create,
            decoder_destroy,
            decoder_get_parameters,
            decoder_render,
            video_mixer_query_feature_support,
            video_mixer_query_parameter_support,
            video_mixer_query_attribute_support,
            video_mixer_query_parameter_value_range,
            video_mixer_query_attribute_value_range,
            video_mixer_create,
            video_mixer_set_feature_enables,
            video_mixer_set_attribute_values,
            video_mixer_get_feature_support,
            video_mixer_get_feature_enables,
            video_mixer_get_parameter_values,
            video_mixer_get_attribute_values,
            video_mixer_destroy,
            video_mixer_render,
            presentation_queue_target_destroy,
            presentation_queue_create,
            presentation_queue_destroy,
            presentation_queue_set_background_color,
            presentation_queue_get_background_color,
            presentation_queue_get_time,
            presentation_queue_display,
            presentation_queue_block_until_surface_idle,
            presentation_queue_query_surface_status,
            preemption_callback_register,
        })
    }

    fn load_t<T>(
        vdp_get_proc_address: &vdpau::VdpGetProcAddress,
        vdp_device: u32,
        function_id: u32,
    ) -> Result<T, va::VAStatus> {
        let mut function = unsafe { std::mem::zeroed::<T>() };
        let load_code = unsafe {
            vdp_get_proc_address.unwrap()(
                vdp_device,
                function_id,
                &mut function as *mut T as *mut _,
            )
        };

        if !load_code.eq(&vdpau::VdpStatus_VDP_STATUS_OK) {
            match load_code {
                vdpau::VdpStatus_VDP_STATUS_OK => eprintln!("VdpStatus_VDP_STATUS_OK"),
                vdpau::VdpStatus_VDP_STATUS_NO_IMPLEMENTATION => {
                    eprintln!("VdpStatus_VDP_STATUS_NO_IMPLEMENTATION")
                }
                vdpau::VdpStatus_VDP_STATUS_DISPLAY_PREEMPTED => {
                    eprintln!("VdpStatus_VDP_STATUS_DISPLAY_PREEMPTED")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_HANDLE => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_HANDLE")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_POINTER => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_POINTER")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_CHROMA_TYPE => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_CHROMA_TYPE")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_Y_CB_CR_FORMAT => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_Y_CB_CR_FORMAT")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_RGBA_FORMAT => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_RGBA_FORMAT")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_INDEXED_FORMAT => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_INDEXED_FORMAT")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_COLOR_STANDARD => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_COLOR_STANDARD")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_COLOR_TABLE_FORMAT => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_COLOR_TABLE_FORMAT")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_BLEND_FACTOR => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_BLEND_FACTOR")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_BLEND_EQUATION => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_BLEND_EQUATION")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_FLAG => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_FLAG")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_DECODER_PROFILE => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_DECODER_PROFILE")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_VIDEO_MIXER_FEATURE => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_VIDEO_MIXER_FEATURE")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_VIDEO_MIXER_PARAMETER => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_VIDEO_MIXER_PARAMETER")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_VIDEO_MIXER_ATTRIBUTE => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_VIDEO_MIXER_ATTRIBUTE")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_VIDEO_MIXER_PICTURE_STRUCTURE => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_VIDEO_MIXER_PICTURE_STRUCTURE")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_FUNC_ID => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_FUNC_ID")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_SIZE => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_SIZE")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_VALUE => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_VALUE")
                }
                vdpau::VdpStatus_VDP_STATUS_INVALID_STRUCT_VERSION => {
                    eprintln!("VdpStatus_VDP_STATUS_INVALID_STRUCT_VERSION")
                }
                vdpau::VdpStatus_VDP_STATUS_RESOURCES => {
                    eprintln!("VdpStatus_VDP_STATUS_RESOURCES")
                }
                vdpau::VdpStatus_VDP_STATUS_HANDLE_DEVICE_MISMATCH => {
                    eprintln!("VdpStatus_VDP_STATUS_HANDLE_DEVICE_MISMATCH")
                }
                vdpau::VdpStatus_VDP_STATUS_ERROR => eprintln!("VdpStatus_VDP_STATUS_ERROR"),
                26_u32..=u32::MAX => eprintln!("Other ERROR!"),
            }
            return Err(va::VA_STATUS_ERROR_UNKNOWN as _);
        }
        Ok(function)
    }
}

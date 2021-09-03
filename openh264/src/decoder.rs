use crate::error::NativeErrorExt;
use crate::Error;
use openh264_sys2::{
    videoFormatI420, ISVCDecoder, ISVCDecoderVtbl, SBufferInfo, SDecodingParam, SParserBsInfo, SSysMEMBuffer, WelsCreateDecoder,
    WelsDestroyDecoder, DECODER_OPTION, DECODER_OPTION_NUM_OF_THREADS, DECODING_STATE,
};
use std::os::raw::{c_int, c_long, c_uchar, c_void};
use std::ptr::{addr_of_mut, null, null_mut};

/// Convenience wrapper with guaranteed function pointers for easy access.
///
/// This struct automatically handles `WelsCreateDecoder` and `WelsDestroyDecoder`.
#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Debug)]
struct DecoderRawAPI {
    decoder_ptr: *mut *const ISVCDecoderVtbl,
    initialize: unsafe extern "C" fn(arg1: *mut ISVCDecoder, pParam: *const SDecodingParam) -> c_long,
    uninitialize: unsafe extern "C" fn(arg1: *mut ISVCDecoder) -> c_long,
    decode_frame: unsafe extern "C" fn(arg1: *mut ISVCDecoder, pSrc: *const c_uchar, iSrcLen: c_int, ppDst: *mut *mut c_uchar, pStride: *mut c_int, iWidth: *mut c_int, iHeight: *mut c_int) -> DECODING_STATE,
    decode_frame_no_delay: unsafe extern "C" fn(arg1: *mut ISVCDecoder, pSrc: *const c_uchar, iSrcLen: c_int, ppDst: *mut *mut c_uchar, pDstInfo: *mut SBufferInfo) -> DECODING_STATE,
    decode_frame2: unsafe extern "C" fn(arg1: *mut ISVCDecoder, pSrc: *const c_uchar, iSrcLen: c_int, ppDst: *mut *mut c_uchar, pDstInfo: *mut SBufferInfo) -> DECODING_STATE,
    flush_frame:  unsafe extern "C" fn(arg1: *mut ISVCDecoder, ppDst: *mut *mut c_uchar, pDstInfo: *mut SBufferInfo) -> DECODING_STATE,
    decode_parser: unsafe extern "C" fn(arg1: *mut ISVCDecoder, pSrc: *const c_uchar, iSrcLen: c_int, pDstInfo: *mut SParserBsInfo) -> DECODING_STATE,
    decode_frame_ex: unsafe extern "C" fn(arg1: *mut ISVCDecoder, pSrc: *const c_uchar, iSrcLen: c_int, pDst: *mut c_uchar, iDstStride: c_int, iDstLen: *mut c_int, iWidth: *mut c_int, iHeight: *mut c_int, iColorFormat: *mut c_int) -> DECODING_STATE,
    set_option: unsafe extern "C" fn(arg1: *mut ISVCDecoder, eOptionId: DECODER_OPTION, pOption: *mut c_void) -> c_long,
    get_option: unsafe extern "C" fn(arg1: *mut ISVCDecoder, eOptionId: DECODER_OPTION, pOption: *mut c_void) -> c_long,
}

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
#[allow(non_snake_case)]
#[allow(unused)]
impl DecoderRawAPI {
    fn new() -> Result<Self, Error> {
        unsafe {
            let mut decoder_ptr = null::<ISVCDecoderVtbl>() as *mut *const ISVCDecoderVtbl;

            WelsCreateDecoder(&mut decoder_ptr as *mut *mut *const ISVCDecoderVtbl).ok()?;

            let e = || {
                Error::msg("VTable missing function.")
            };

            Ok(DecoderRawAPI {
                decoder_ptr,
                initialize: (*(*decoder_ptr)).Initialize.ok_or(e())?,
                uninitialize: (*(*decoder_ptr)).Uninitialize.ok_or(e())?,
                decode_frame: (*(*decoder_ptr)).DecodeFrame.ok_or(e())?,
                decode_frame_no_delay: (*(*decoder_ptr)).DecodeFrameNoDelay.ok_or(e())?,
                decode_frame2: (*(*decoder_ptr)).DecodeFrame2.ok_or(e())?,
                flush_frame: (*(*decoder_ptr)).FlushFrame.ok_or(e())?,
                decode_parser: (*(*decoder_ptr)).DecodeParser.ok_or(e())?,
                decode_frame_ex: (*(*decoder_ptr)).DecodeFrameEx.ok_or(e())?,
                set_option: (*(*decoder_ptr)).SetOption.ok_or(e())?,
                get_option: (*(*decoder_ptr)).GetOption.ok_or(e())?,
            })
        }
    }

    unsafe fn initialize(&self, pParam: *const SDecodingParam) -> c_long {
        (self.initialize)(self.decoder_ptr, pParam)
    }

    unsafe fn uninitialize(&self, ) -> c_long {
        (self.uninitialize)(self.decoder_ptr)
    }

    unsafe fn decode_frame(&self, Src: *const c_uchar, iSrcLen: c_int, ppDst: *mut *mut c_uchar, pStride: *mut c_int, iWidth: *mut c_int, iHeight: *mut c_int) -> DECODING_STATE {
        (self.decode_frame)(self.decoder_ptr, Src, iSrcLen, ppDst, pStride, iWidth, iHeight)
    }

    unsafe fn decode_frame_no_delay(&self, pSrc: *const c_uchar, iSrcLen: c_int, ppDst: *mut *mut c_uchar, pDstInfo: *mut SBufferInfo) -> DECODING_STATE {
        (self.decode_frame_no_delay)(self.decoder_ptr, pSrc, iSrcLen, ppDst, pDstInfo)
    }

    unsafe fn decode_frame2(&self, pSrc: *const c_uchar, iSrcLen: c_int, ppDst: *mut *mut c_uchar, pDstInfo: *mut SBufferInfo) -> DECODING_STATE {
        (self.decode_frame2)(self.decoder_ptr, pSrc, iSrcLen, ppDst, pDstInfo)
    }

    unsafe fn flush_frame(&self, ppDst: *mut *mut c_uchar, pDstInfo: *mut SBufferInfo) -> DECODING_STATE {
        (self.flush_frame)(self.decoder_ptr, ppDst, pDstInfo)
    }

    unsafe fn decode_parser(&self, pSrc: *const c_uchar, iSrcLen: c_int, pDstInfo: *mut SParserBsInfo) -> DECODING_STATE {
        (self.decode_parser)(self.decoder_ptr, pSrc, iSrcLen, pDstInfo)
    }

    unsafe fn decode_frame_ex(&self, pSrc: *const c_uchar, iSrcLen: c_int, pDst: *mut c_uchar, iDstStride: c_int, iDstLen: *mut c_int, iWidth: *mut c_int, iHeight: *mut c_int, iColorFormat: *mut c_int) -> DECODING_STATE {
        (self.decode_frame_ex)(self.decoder_ptr, pSrc, iSrcLen, pDst, iDstStride, iDstLen, iWidth, iHeight, iColorFormat)
    }

    unsafe fn set_option(&self, eOptionId: DECODER_OPTION, pOption: *mut c_void) -> c_long {
        (self.set_option)(self.decoder_ptr, eOptionId, pOption)
    }

    unsafe fn get_option(&self, eOptionId: DECODER_OPTION, pOption: *mut c_void) -> c_long {
        (self.get_option)(self.decoder_ptr, eOptionId, pOption)
    }
}

impl Drop for DecoderRawAPI {
    fn drop(&mut self) {
        // Safe because when we drop the pointer must have been initialized, and we aren't clone.
        unsafe {
            WelsDestroyDecoder(self.decoder_ptr);
        }
    }
}

/// Configuration for the [`Decoder`].
///
/// Setting missing? Please file a PR!
#[derive(Default, Copy, Clone, Debug)]
pub struct DecoderConfig {
    params: SDecodingParam,
    num_threads: i32,
}

impl DecoderConfig {
    /// Creates a new default encoder config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of threads.
    ///
    /// # Safety
    ///
    /// Right now it seems to be unclear if setting a thread count is entirely safe.
    /// If you have proof either way, please file an PR removing the `unsafe` marker, or this section.
    pub unsafe fn num_threads(mut self, num_threads: u32) -> Self {
        self.num_threads = num_threads as i32;
        self
    }
}

/// An [OpenH264](https://github.com/cisco/openh264) decoder, converts packets to YUV.
#[derive(Debug)]
pub struct Decoder {
    raw_api: DecoderRawAPI,
}

impl Decoder {
    /// Create a decoder with default settings.
    pub fn new() -> Result<Self, Error> {
        Self::with_config(DecoderConfig::new())
    }

    /// Create a decoder with the provided configuration.
    pub fn with_config(mut config: DecoderConfig) -> Result<Self, Error> {
        let raw = DecoderRawAPI::new()?;

        unsafe {
            raw.initialize(&config.params).ok()?;
            raw.set_option(DECODER_OPTION_NUM_OF_THREADS, addr_of_mut!(config.num_threads).cast())
                .ok()?;
        };

        Ok(Self { raw_api: raw })
    }

    /// Decodes a complete H.264 bitstream and returns the latest picture.
    ///
    /// This function can be called with:
    ///
    /// - only a complete SPS / PPS header (usually the first some 30 bytes of a H.264 stream)
    /// - the headers and series of complete frames
    /// - new frames after previous headers and frames were successfully decoded.
    ///
    /// In each case, it will return the decoded image in YUV format.
    ///
    /// # Errors
    ///
    /// The function returns and error if any of the packets is incomplete, e.g., was truncated.
    pub fn decode_no_delay(&mut self, packet: &[u8]) -> Result<DecodedYUV, Error> {
        let mut dst = [null_mut(); 3];
        let mut buffer_info = SBufferInfo::default();

        unsafe {
            self.raw_api
                .decode_frame_no_delay(packet.as_ptr(), packet.len() as i32, &mut dst as *mut _, &mut buffer_info)
                .ok()?;

            if !buffer_info.iBufferStatus == 1 {
                return Err(Error::msg("Buffer status not valid"));
            }

            let info = buffer_info.UsrData.sSystemBuffer;

            // https://github.com/cisco/openh264/issues/2379
            let y = std::slice::from_raw_parts(dst[0], (info.iHeight * info.iStride[0]) as usize);
            let u = std::slice::from_raw_parts(dst[1], (info.iHeight * info.iStride[1] / 2) as usize);
            let v = std::slice::from_raw_parts(dst[2], (info.iHeight * info.iStride[1] / 2) as usize);

            Ok(DecodedYUV { info, y, u, v })
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        // Safe because when we drop the pointer must have been initialized.
        unsafe {
            self.raw_api.uninitialize();
        }
    }
}

/// Frame returned by the [`Decoder`] and provides safe data access.
pub struct DecodedYUV<'a> {
    info: SSysMEMBuffer,

    y: &'a [u8],
    u: &'a [u8],
    v: &'a [u8],
}

impl<'a> DecodedYUV<'a> {
    /// Returns the Y (luma) array, including padding.
    ///
    /// You can use [`strides_yuv()`](Self::strides_yuv) to compute unpadded pixel positions.
    pub fn y_with_stride(&self) -> &'a [u8] {
        self.y
    }

    /// Returns the U (blue projection) array, including padding.
    ///
    /// You can use [`strides_yuv()`](Self::strides_yuv) to compute unpadded pixel positions.
    pub fn u_with_stride(&self) -> &'a [u8] {
        self.u
    }

    /// Returns the V (red projection) array, including padding.
    ///
    /// You can use [`strides_yuv()`](Self::strides_yuv) to compute unpadded pixel positions.
    pub fn v_with_stride(&self) -> &'a [u8] {
        self.v
    }

    /// Returns the unpadded, image size in pixels when using [`write_rgb8()`](Self::write_rgb8).
    pub fn dimension_rgb(&self) -> (usize, usize) {
        (self.info.iWidth as usize, self.info.iHeight as usize)
    }

    /// Returns the unpadded Y size.
    ///
    /// This may or may not be smaller than the image size.
    pub fn dimension_y(&self) -> (usize, usize) {
        (self.info.iWidth as usize, self.info.iHeight as usize)
    }

    /// Returns the unpadded U size.
    ///
    /// This is often smaller (by half) than the image size.
    pub fn dimension_u(&self) -> (usize, usize) {
        (self.info.iWidth as usize / 2, self.info.iHeight as usize / 2)
    }

    /// Returns the unpadded V size.
    ///
    /// This is often smaller (by half) than the image size.
    pub fn dimension_v(&self) -> (usize, usize) {
        (self.info.iWidth as usize / 2, self.info.iHeight as usize / 2)
    }

    /// Returns strides for the (Y,U,V) arrays.
    pub fn strides_yuv(&self) -> (usize, usize, usize) {
        (
            self.info.iStride[0] as usize,
            self.info.iStride[1] as usize,
            self.info.iStride[1] as usize,
        )
    }

    /// Writes the image into a byte buffer of size `w*h*3`.
    pub fn write_rgb8(&self, target: &mut [u8]) -> Result<(), Error> {
        let dim = self.dimension_rgb();
        let strides = self.strides_yuv();
        let wanted = dim.0 * dim.1 * 3;

        // This needs some love, and better architecture.
        assert_eq!(self.info.iFormat, videoFormatI420 as i32);

        if target.len() != wanted as usize {
            return Err(Error::msg(&format!(
                "Target RGB8 array does not match image dimensions. Wanted: {} * {} * 3 = {}, got {}",
                dim.0,
                dim.1,
                wanted,
                target.len()
            )));
        }

        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let base_tgt = (y * dim.0 + x) * 3;
                let base_y = y * strides.0 + x;
                let base_u = (y / 2 * strides.1) + (x / 2);
                let base_v = (y / 2 * strides.2) + (x / 2);

                let rgb_pixel = &mut target[base_tgt..base_tgt + 3];

                let y = self.y[base_y] as f32;
                let u = self.u[base_u] as f32;
                let v = self.v[base_v] as f32;

                rgb_pixel[0] = (y + 1.402 * (v - 128.0)) as u8;
                rgb_pixel[1] = (y - 0.344 * (u - 128.0) - 0.714 * (v - 128.0)) as u8;
                rgb_pixel[2] = (y + 1.772 * (u - 128.0)) as u8;
            }
        }

        Ok(())
    }

    pub fn write_rgba8(&self, target: &mut [u8]) -> Result<(), Error> {
        let dim = self.dimension_rgb();
        let strides = self.strides_yuv();
        let wanted = dim.0 * dim.1 * 4;

        // This needs some love, and better architecture.
        assert_eq!(self.info.iFormat, videoFormatI420 as i32);

        if target.len() != wanted as usize {
            return Err(Error::msg(&format!(
                "Target RGB8 array does not match image dimensions. Wanted: {} * {} * 4 = {}, got {}",
                dim.0,
                dim.1,
                wanted,
                target.len()
            )));
        }

        for y in 0..dim.1 {
            for x in 0..dim.0 {
                let base_tgt = (y * dim.0 + x) * 4;
                let base_y = y * strides.0 + x;
                let base_u = (y / 2 * strides.1) + (x / 2);
                let base_v = (y / 2 * strides.2) + (x / 2);

                let rgb_pixel = &mut target[base_tgt..base_tgt + 4];

                let y = self.y[base_y] as f32;
                let u = self.u[base_u] as f32;
                let v = self.v[base_v] as f32;

                rgb_pixel[0] = (y + 1.402 * (v - 128.0)) as u8;
                rgb_pixel[1] = (y - 0.344 * (u - 128.0) - 0.714 * (v - 128.0)) as u8;
                rgb_pixel[2] = (y + 1.772 * (u - 128.0)) as u8;
                rgb_pixel[3] = 255;
            }
        }

        Ok(())
    }
}

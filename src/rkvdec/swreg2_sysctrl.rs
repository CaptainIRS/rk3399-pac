#[doc = "Register `SWREG2_SYSCTRL` reader"]
pub type R = crate::R<Swreg2SysctrlSpec>;
#[doc = "Register `SWREG2_SYSCTRL` writer"]
pub type W = crate::W<Swreg2SysctrlSpec>;
#[doc = "Field `SW_IN_ENDIAN` reader - decoder input endian mode for other than stream and dpb data\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
pub type SwInEndianR = crate::BitReader;
#[doc = "Field `SW_IN_ENDIAN` writer - decoder input endian mode for other than stream and dpb data\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
pub type SwInEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IN_SWAP32_E` reader - input 32bit data swap for other than stream and dpb data\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
pub type SwInSwap32ER = crate::BitReader;
#[doc = "Field `SW_IN_SWAP32_E` writer - input 32bit data swap for other than stream and dpb data\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
pub type SwInSwap32EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IN_SWAP64_E` reader - input 64bit data swap for other than stream and dpb data\n\nmay be used for 128 bit environment\n\n0 = no swapping of 64 bit words\n\n1 = 64 bit data words are swapped"]
pub type SwInSwap64ER = crate::BitReader;
#[doc = "Field `SW_IN_SWAP64_E` writer - input 64bit data swap for other than stream and dpb data\n\nmay be used for 128 bit environment\n\n0 = no swapping of 64 bit words\n\n1 = 64 bit data words are swapped"]
pub type SwInSwap64EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_STR_ENDIAN` reader - stream data input endian mode\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
pub type SwStrEndianR = crate::BitReader;
#[doc = "Field `SW_STR_ENDIAN` writer - stream data input endian mode\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
pub type SwStrEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_STR_SWAP32_E` reader - stream 32bit data swap\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
pub type SwStrSwap32ER = crate::BitReader;
#[doc = "Field `SW_STR_SWAP32_E` writer - stream 32bit data swap\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
pub type SwStrSwap32EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_STR_SWAP64_E` reader - stream 64bit data swap\n\nmay be used for 128 bit environment\n\n0 = no swapping of 64 bit words\n\n1 = 64 bit data words are swapped"]
pub type SwStrSwap64ER = crate::BitReader;
#[doc = "Field `SW_STR_SWAP64_E` writer - stream 64bit data swap\n\nmay be used for 128 bit environment\n\n0 = no swapping of 64 bit words\n\n1 = 64 bit data words are swapped"]
pub type SwStrSwap64EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_OUT_ENDIAN` reader - dec output data and colmv , dpb data and colmv input endian\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
pub type SwOutEndianR = crate::BitReader;
#[doc = "Field `SW_OUT_ENDIAN` writer - dec output data and colmv , dpb data and colmv input endian\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
pub type SwOutEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_OUT_SWAP32_E` reader - decoder output data and dpb input data 32bit swap\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
pub type SwOutSwap32ER = crate::BitReader;
#[doc = "Field `SW_OUT_SWAP32_E` writer - decoder output data and dpb input data 32bit swap\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
pub type SwOutSwap32EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "output cbcr swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwOutCbcrSwap {
    #[doc = "0: cb(u) is in the lower address, cr(v) is in the higher address"]
    B0 = 0,
    #[doc = "1: cb(u) is in the higher address,cr(v) is in the lower address sw_in_cbcr_swap is the same with sw_out_cbcr_swap"]
    B1 = 1,
}
impl From<SwOutCbcrSwap> for bool {
    #[inline(always)]
    fn from(variant: SwOutCbcrSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_OUT_CBCR_SWAP` reader - output cbcr swap"]
pub type SwOutCbcrSwapR = crate::BitReader<SwOutCbcrSwap>;
impl SwOutCbcrSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwOutCbcrSwap {
        match self.bits {
            false => SwOutCbcrSwap::B0,
            true => SwOutCbcrSwap::B1,
        }
    }
    #[doc = "cb(u) is in the lower address, cr(v) is in the higher address"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwOutCbcrSwap::B0
    }
    #[doc = "cb(u) is in the higher address,cr(v) is in the lower address sw_in_cbcr_swap is the same with sw_out_cbcr_swap"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwOutCbcrSwap::B1
    }
}
#[doc = "Field `SW_OUT_CBCR_SWAP` writer - output cbcr swap"]
pub type SwOutCbcrSwapW<'a, REG> = crate::BitWriter<'a, REG, SwOutCbcrSwap>;
impl<'a, REG> SwOutCbcrSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cb(u) is in the lower address, cr(v) is in the higher address"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwOutCbcrSwap::B0)
    }
    #[doc = "cb(u) is in the higher address,cr(v) is in the lower address sw_in_cbcr_swap is the same with sw_out_cbcr_swap"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwOutCbcrSwap::B1)
    }
}
#[doc = "Field `SW_RLC_MODE_DIRECT_WRITE` reader - cabac decode output direct write\n\ncabac decode output direct write enable\n\nwhen this bit is enable , all the module other than cabac and busifd\n\nare not work"]
pub type SwRlcModeDirectWriteR = crate::BitReader;
#[doc = "Field `SW_RLC_MODE_DIRECT_WRITE` writer - cabac decode output direct write\n\ncabac decode output direct write enable\n\nwhen this bit is enable , all the module other than cabac and busifd\n\nare not work"]
pub type SwRlcModeDirectWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RLC_MODE` reader - rlc mode enable\n\n0 = HW decodes video from bit stream\n\n1 = HW decodes video from RLC input data"]
pub type SwRlcModeR = crate::BitReader;
#[doc = "Field `SW_RLC_MODE` writer - rlc mode enable\n\n0 = HW decodes video from bit stream\n\n1 = HW decodes video from RLC input data"]
pub type SwRlcModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_STRM_START_BIT` reader - exact bit of stream start\n\nexact bit of streamd start word where decoding can be started\n\n(asosiates with sw_str_rlc_base)"]
pub type SwStrmStartBitR = crate::FieldReader;
#[doc = "Field `SW_STRM_START_BIT` writer - exact bit of stream start\n\nexact bit of streamd start word where decoding can be started\n\n(asosiates with sw_str_rlc_base)"]
pub type SwStrmStartBitW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "dec mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDecMode {
    #[doc = "0: hevc"]
    D0 = 0,
    #[doc = "1: h264"]
    D1 = 1,
    #[doc = "2: vp9"]
    D2 = 2,
}
impl From<SwDecMode> for u8 {
    #[inline(always)]
    fn from(variant: SwDecMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDecMode {
    type Ux = u8;
}
#[doc = "Field `SW_DEC_MODE` reader - dec mode"]
pub type SwDecModeR = crate::FieldReader<SwDecMode>;
impl SwDecModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwDecMode> {
        match self.bits {
            0 => Some(SwDecMode::D0),
            1 => Some(SwDecMode::D1),
            2 => Some(SwDecMode::D2),
            _ => None,
        }
    }
    #[doc = "hevc"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SwDecMode::D0
    }
    #[doc = "h264"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SwDecMode::D1
    }
    #[doc = "vp9"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SwDecMode::D2
    }
}
#[doc = "Field `SW_DEC_MODE` writer - dec mode"]
pub type SwDecModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, SwDecMode>;
impl<'a, REG> SwDecModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "hevc"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecMode::D0)
    }
    #[doc = "h264"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecMode::D1)
    }
    #[doc = "vp9"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecMode::D2)
    }
}
#[doc = "h264 rps mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwH264RpsMode {
    #[doc = "0: hardware parse rps mode"]
    B0 = 0,
    #[doc = "1: software parse rps mode"]
    B1 = 1,
}
impl From<SwH264RpsMode> for bool {
    #[inline(always)]
    fn from(variant: SwH264RpsMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_H264_RPS_MODE` reader - h264 rps mode"]
pub type SwH264RpsModeR = crate::BitReader<SwH264RpsMode>;
impl SwH264RpsModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwH264RpsMode {
        match self.bits {
            false => SwH264RpsMode::B0,
            true => SwH264RpsMode::B1,
        }
    }
    #[doc = "hardware parse rps mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwH264RpsMode::B0
    }
    #[doc = "software parse rps mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwH264RpsMode::B1
    }
}
#[doc = "Field `SW_H264_RPS_MODE` writer - h264 rps mode"]
pub type SwH264RpsModeW<'a, REG> = crate::BitWriter<'a, REG, SwH264RpsMode>;
impl<'a, REG> SwH264RpsModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "hardware parse rps mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwH264RpsMode::B0)
    }
    #[doc = "software parse rps mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwH264RpsMode::B1)
    }
}
#[doc = "h264 stream mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwH264StreamMode {
    #[doc = "0: stream packet is slice by slice or frame by frame, should use sw_h264_frame_orslice"]
    B0 = 0,
    #[doc = "1: stream packet is random, should use sw_h264_stream_last"]
    B1 = 1,
}
impl From<SwH264StreamMode> for bool {
    #[inline(always)]
    fn from(variant: SwH264StreamMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_H264_STREAM_MODE` reader - h264 stream mode"]
pub type SwH264StreamModeR = crate::BitReader<SwH264StreamMode>;
impl SwH264StreamModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwH264StreamMode {
        match self.bits {
            false => SwH264StreamMode::B0,
            true => SwH264StreamMode::B1,
        }
    }
    #[doc = "stream packet is slice by slice or frame by frame, should use sw_h264_frame_orslice"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwH264StreamMode::B0
    }
    #[doc = "stream packet is random, should use sw_h264_stream_last"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwH264StreamMode::B1
    }
}
#[doc = "Field `SW_H264_STREAM_MODE` writer - h264 stream mode"]
pub type SwH264StreamModeW<'a, REG> = crate::BitWriter<'a, REG, SwH264StreamMode>;
impl<'a, REG> SwH264StreamModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stream packet is slice by slice or frame by frame, should use sw_h264_frame_orslice"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwH264StreamMode::B0)
    }
    #[doc = "stream packet is random, should use sw_h264_stream_last"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwH264StreamMode::B1)
    }
}
#[doc = "Field `SW_H264_STREAM_LASTPACKET` reader - stream last packet flag\n\nwhen sw_h264_stream_mode is 1'b1\n\nsw_h264_stream_lastpacket 1'b0: this packet is not the last packet\n\nof frame\n\n1'b1: the packet is the last packet of frame"]
pub type SwH264StreamLastpacketR = crate::BitReader;
#[doc = "Field `SW_H264_STREAM_LASTPACKET` writer - stream last packet flag\n\nwhen sw_h264_stream_mode is 1'b1\n\nsw_h264_stream_lastpacket 1'b0: this packet is not the last packet\n\nof frame\n\n1'b1: the packet is the last packet of frame"]
pub type SwH264StreamLastpacketW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_H264_FIRSTSLICE_FLAG` reader - firstslice flag\n\n1'b1: first packet in the frame, for h264 decode to read rps/pps\n\ndata\n\nbecause the first_mb_in_slice may be wrong, so need this syntax"]
pub type SwH264FirstsliceFlagR = crate::BitReader;
#[doc = "Field `SW_H264_FIRSTSLICE_FLAG` writer - firstslice flag\n\n1'b1: first packet in the frame, for h264 decode to read rps/pps\n\ndata\n\nbecause the first_mb_in_slice may be wrong, so need this syntax"]
pub type SwH264FirstsliceFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "h264 frame or slice\n\nfor H264 use\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwH264FrameOrslice {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: slice when sw_h264_streamd_mode is 1'b0, this register is valid"]
    B1 = 1,
}
impl From<SwH264FrameOrslice> for bool {
    #[inline(always)]
    fn from(variant: SwH264FrameOrslice) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_H264_FRAME_ORSLICE` reader - h264 frame or slice\n\nfor H264 use"]
pub type SwH264FrameOrsliceR = crate::BitReader<SwH264FrameOrslice>;
impl SwH264FrameOrsliceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwH264FrameOrslice {
        match self.bits {
            false => SwH264FrameOrslice::B0,
            true => SwH264FrameOrslice::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwH264FrameOrslice::B0
    }
    #[doc = "slice when sw_h264_streamd_mode is 1'b0, this register is valid"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwH264FrameOrslice::B1
    }
}
#[doc = "Field `SW_H264_FRAME_ORSLICE` writer - h264 frame or slice\n\nfor H264 use"]
pub type SwH264FrameOrsliceW<'a, REG> = crate::BitWriter<'a, REG, SwH264FrameOrslice>;
impl<'a, REG> SwH264FrameOrsliceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwH264FrameOrslice::B0)
    }
    #[doc = "slice when sw_h264_streamd_mode is 1'b0, this register is valid"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwH264FrameOrslice::B1)
    }
}
impl R {
    #[doc = "Bit 0 - decoder input endian mode for other than stream and dpb data\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
    #[inline(always)]
    pub fn sw_in_endian(&self) -> SwInEndianR {
        SwInEndianR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - input 32bit data swap for other than stream and dpb data\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
    #[inline(always)]
    pub fn sw_in_swap32_e(&self) -> SwInSwap32ER {
        SwInSwap32ER::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - input 64bit data swap for other than stream and dpb data\n\nmay be used for 128 bit environment\n\n0 = no swapping of 64 bit words\n\n1 = 64 bit data words are swapped"]
    #[inline(always)]
    pub fn sw_in_swap64_e(&self) -> SwInSwap64ER {
        SwInSwap64ER::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - stream data input endian mode\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
    #[inline(always)]
    pub fn sw_str_endian(&self) -> SwStrEndianR {
        SwStrEndianR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - stream 32bit data swap\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
    #[inline(always)]
    pub fn sw_str_swap32_e(&self) -> SwStrSwap32ER {
        SwStrSwap32ER::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - stream 64bit data swap\n\nmay be used for 128 bit environment\n\n0 = no swapping of 64 bit words\n\n1 = 64 bit data words are swapped"]
    #[inline(always)]
    pub fn sw_str_swap64_e(&self) -> SwStrSwap64ER {
        SwStrSwap64ER::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - dec output data and colmv , dpb data and colmv input endian\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
    #[inline(always)]
    pub fn sw_out_endian(&self) -> SwOutEndianR {
        SwOutEndianR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - decoder output data and dpb input data 32bit swap\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
    #[inline(always)]
    pub fn sw_out_swap32_e(&self) -> SwOutSwap32ER {
        SwOutSwap32ER::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - output cbcr swap"]
    #[inline(always)]
    pub fn sw_out_cbcr_swap(&self) -> SwOutCbcrSwapR {
        SwOutCbcrSwapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - cabac decode output direct write\n\ncabac decode output direct write enable\n\nwhen this bit is enable , all the module other than cabac and busifd\n\nare not work"]
    #[inline(always)]
    pub fn sw_rlc_mode_direct_write(&self) -> SwRlcModeDirectWriteR {
        SwRlcModeDirectWriteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - rlc mode enable\n\n0 = HW decodes video from bit stream\n\n1 = HW decodes video from RLC input data"]
    #[inline(always)]
    pub fn sw_rlc_mode(&self) -> SwRlcModeR {
        SwRlcModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:18 - exact bit of stream start\n\nexact bit of streamd start word where decoding can be started\n\n(asosiates with sw_str_rlc_base)"]
    #[inline(always)]
    pub fn sw_strm_start_bit(&self) -> SwStrmStartBitR {
        SwStrmStartBitR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - dec mode"]
    #[inline(always)]
    pub fn sw_dec_mode(&self) -> SwDecModeR {
        SwDecModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - h264 rps mode"]
    #[inline(always)]
    pub fn sw_h264_rps_mode(&self) -> SwH264RpsModeR {
        SwH264RpsModeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - h264 stream mode"]
    #[inline(always)]
    pub fn sw_h264_stream_mode(&self) -> SwH264StreamModeR {
        SwH264StreamModeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - stream last packet flag\n\nwhen sw_h264_stream_mode is 1'b1\n\nsw_h264_stream_lastpacket 1'b0: this packet is not the last packet\n\nof frame\n\n1'b1: the packet is the last packet of frame"]
    #[inline(always)]
    pub fn sw_h264_stream_lastpacket(&self) -> SwH264StreamLastpacketR {
        SwH264StreamLastpacketR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - firstslice flag\n\n1'b1: first packet in the frame, for h264 decode to read rps/pps\n\ndata\n\nbecause the first_mb_in_slice may be wrong, so need this syntax"]
    #[inline(always)]
    pub fn sw_h264_firstslice_flag(&self) -> SwH264FirstsliceFlagR {
        SwH264FirstsliceFlagR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - h264 frame or slice\n\nfor H264 use"]
    #[inline(always)]
    pub fn sw_h264_frame_orslice(&self) -> SwH264FrameOrsliceR {
        SwH264FrameOrsliceR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - decoder input endian mode for other than stream and dpb data\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_in_endian(&mut self) -> SwInEndianW<Swreg2SysctrlSpec> {
        SwInEndianW::new(self, 0)
    }
    #[doc = "Bit 1 - input 32bit data swap for other than stream and dpb data\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
    #[inline(always)]
    #[must_use]
    pub fn sw_in_swap32_e(&mut self) -> SwInSwap32EW<Swreg2SysctrlSpec> {
        SwInSwap32EW::new(self, 1)
    }
    #[doc = "Bit 2 - input 64bit data swap for other than stream and dpb data\n\nmay be used for 128 bit environment\n\n0 = no swapping of 64 bit words\n\n1 = 64 bit data words are swapped"]
    #[inline(always)]
    #[must_use]
    pub fn sw_in_swap64_e(&mut self) -> SwInSwap64EW<Swreg2SysctrlSpec> {
        SwInSwap64EW::new(self, 2)
    }
    #[doc = "Bit 3 - stream data input endian mode\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_str_endian(&mut self) -> SwStrEndianW<Swreg2SysctrlSpec> {
        SwStrEndianW::new(self, 3)
    }
    #[doc = "Bit 4 - stream 32bit data swap\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
    #[inline(always)]
    #[must_use]
    pub fn sw_str_swap32_e(&mut self) -> SwStrSwap32EW<Swreg2SysctrlSpec> {
        SwStrSwap32EW::new(self, 4)
    }
    #[doc = "Bit 5 - stream 64bit data swap\n\nmay be used for 128 bit environment\n\n0 = no swapping of 64 bit words\n\n1 = 64 bit data words are swapped"]
    #[inline(always)]
    #[must_use]
    pub fn sw_str_swap64_e(&mut self) -> SwStrSwap64EW<Swreg2SysctrlSpec> {
        SwStrSwap64EW::new(self, 5)
    }
    #[doc = "Bit 6 - dec output data and colmv , dpb data and colmv input endian\n\n0 = little endian\n\n1 = big endian\n\nfor litter enadian , a data 0x12345678, 0x78 is stored in lower\n\naddress, 0x12 is stored in higher address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_out_endian(&mut self) -> SwOutEndianW<Swreg2SysctrlSpec> {
        SwOutEndianW::new(self, 6)
    }
    #[doc = "Bit 7 - decoder output data and dpb input data 32bit swap\n\nmay be used for 64 or 128 bit environment\n\n0 = no swapping of 32 bit words\n\n1 = 32 bit data words are swapped"]
    #[inline(always)]
    #[must_use]
    pub fn sw_out_swap32_e(&mut self) -> SwOutSwap32EW<Swreg2SysctrlSpec> {
        SwOutSwap32EW::new(self, 7)
    }
    #[doc = "Bit 8 - output cbcr swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_out_cbcr_swap(&mut self) -> SwOutCbcrSwapW<Swreg2SysctrlSpec> {
        SwOutCbcrSwapW::new(self, 8)
    }
    #[doc = "Bit 10 - cabac decode output direct write\n\ncabac decode output direct write enable\n\nwhen this bit is enable , all the module other than cabac and busifd\n\nare not work"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rlc_mode_direct_write(&mut self) -> SwRlcModeDirectWriteW<Swreg2SysctrlSpec> {
        SwRlcModeDirectWriteW::new(self, 10)
    }
    #[doc = "Bit 11 - rlc mode enable\n\n0 = HW decodes video from bit stream\n\n1 = HW decodes video from RLC input data"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rlc_mode(&mut self) -> SwRlcModeW<Swreg2SysctrlSpec> {
        SwRlcModeW::new(self, 11)
    }
    #[doc = "Bits 12:18 - exact bit of stream start\n\nexact bit of streamd start word where decoding can be started\n\n(asosiates with sw_str_rlc_base)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_strm_start_bit(&mut self) -> SwStrmStartBitW<Swreg2SysctrlSpec> {
        SwStrmStartBitW::new(self, 12)
    }
    #[doc = "Bits 20:21 - dec mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_mode(&mut self) -> SwDecModeW<Swreg2SysctrlSpec> {
        SwDecModeW::new(self, 20)
    }
    #[doc = "Bit 24 - h264 rps mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_h264_rps_mode(&mut self) -> SwH264RpsModeW<Swreg2SysctrlSpec> {
        SwH264RpsModeW::new(self, 24)
    }
    #[doc = "Bit 25 - h264 stream mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_h264_stream_mode(&mut self) -> SwH264StreamModeW<Swreg2SysctrlSpec> {
        SwH264StreamModeW::new(self, 25)
    }
    #[doc = "Bit 26 - stream last packet flag\n\nwhen sw_h264_stream_mode is 1'b1\n\nsw_h264_stream_lastpacket 1'b0: this packet is not the last packet\n\nof frame\n\n1'b1: the packet is the last packet of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sw_h264_stream_lastpacket(&mut self) -> SwH264StreamLastpacketW<Swreg2SysctrlSpec> {
        SwH264StreamLastpacketW::new(self, 26)
    }
    #[doc = "Bit 27 - firstslice flag\n\n1'b1: first packet in the frame, for h264 decode to read rps/pps\n\ndata\n\nbecause the first_mb_in_slice may be wrong, so need this syntax"]
    #[inline(always)]
    #[must_use]
    pub fn sw_h264_firstslice_flag(&mut self) -> SwH264FirstsliceFlagW<Swreg2SysctrlSpec> {
        SwH264FirstsliceFlagW::new(self, 27)
    }
    #[doc = "Bit 28 - h264 frame or slice\n\nfor H264 use"]
    #[inline(always)]
    #[must_use]
    pub fn sw_h264_frame_orslice(&mut self) -> SwH264FrameOrsliceW<Swreg2SysctrlSpec> {
        SwH264FrameOrsliceW::new(self, 28)
    }
}
#[doc = "Data input and output endian setting and sys ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg2_sysctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg2_sysctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg2SysctrlSpec;
impl crate::RegisterSpec for Swreg2SysctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg2_sysctrl::R`](R) reader structure"]
impl crate::Readable for Swreg2SysctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg2_sysctrl::W`](W) writer structure"]
impl crate::Writable for Swreg2SysctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG2_SYSCTRL to value 0"]
impl crate::Resettable for Swreg2SysctrlSpec {
    const RESET_VALUE: u32 = 0;
}

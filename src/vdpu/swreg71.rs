#[doc = "Register `SWREG71` reader"]
pub type R = crate::R<Swreg71Spec>;
#[doc = "Field `SW_DEC_MAX_ALLOW_W` reader - the max width can be decoder"]
pub type SwDecMaxAllowWR = crate::FieldReader<u16>;
#[doc = "Decoding format support for Sorenson\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecSrsonAllow {
    #[doc = "0: no support"]
    B0 = 0,
    #[doc = "1: support"]
    B1 = 1,
}
impl From<DecSrsonAllow> for bool {
    #[inline(always)]
    fn from(variant: DecSrsonAllow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEC_SRSON_ALLOW` reader - Decoding format support for Sorenson"]
pub type DecSrsonAllowR = crate::BitReader<DecSrsonAllow>;
impl DecSrsonAllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DecSrsonAllow {
        match self.bits {
            false => DecSrsonAllow::B0,
            true => DecSrsonAllow::B1,
        }
    }
    #[doc = "no support"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DecSrsonAllow::B0
    }
    #[doc = "support"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DecSrsonAllow::B1
    }
}
#[doc = "\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusW {
    #[doc = "0: error"]
    D0 = 0,
    #[doc = "1: word bus"]
    D1 = 1,
    #[doc = "2: double word bus"]
    D2 = 2,
    #[doc = "3: quadruple word bus"]
    D3 = 3,
}
impl From<BusW> for u8 {
    #[inline(always)]
    fn from(variant: BusW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BusW {
    type Ux = u8;
}
#[doc = "Field `BUS_W` reader - "]
pub type BusWR = crate::FieldReader<BusW>;
impl BusWR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BusW {
        match self.bits {
            0 => BusW::D0,
            1 => BusW::D1,
            2 => BusW::D2,
            3 => BusW::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == BusW::D0
    }
    #[doc = "word bus"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == BusW::D1
    }
    #[doc = "double word bus"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == BusW::D2
    }
    #[doc = "quadruple word bus"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == BusW::D3
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RtlLanSel {
    #[doc = "0: no used"]
    D0 = 0,
    #[doc = "1: vhdl"]
    D1 = 1,
    #[doc = "2: verilog"]
    D2 = 2,
}
impl From<RtlLanSel> for u8 {
    #[inline(always)]
    fn from(variant: RtlLanSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RtlLanSel {
    type Ux = u8;
}
#[doc = "Field `RTL_LAN_SEL` reader - "]
pub type RtlLanSelR = crate::FieldReader<RtlLanSel>;
impl RtlLanSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RtlLanSel> {
        match self.bits {
            0 => Some(RtlLanSel::D0),
            1 => Some(RtlLanSel::D1),
            2 => Some(RtlLanSel::D2),
            _ => None,
        }
    }
    #[doc = "no used"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == RtlLanSel::D0
    }
    #[doc = "vhdl"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == RtlLanSel::D1
    }
    #[doc = "verilog"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == RtlLanSel::D2
    }
}
#[doc = "\n\nValue on reset: 5"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DecStdBus {
    #[doc = "0: error"]
    D0 = 0,
    #[doc = "1: AHB master, AHB slave"]
    D1 = 1,
    #[doc = "2: OCP master, OCP slave"]
    D2 = 2,
    #[doc = "3: AXI master, AXI slave"]
    D3 = 3,
    #[doc = "4: AXI master, APB slave"]
    D4 = 4,
    #[doc = "5: AXI master, AHB slave"]
    D5 = 5,
}
impl From<DecStdBus> for u8 {
    #[inline(always)]
    fn from(variant: DecStdBus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DecStdBus {
    type Ux = u8;
}
#[doc = "Field `DEC_STD_BUS` reader - "]
pub type DecStdBusR = crate::FieldReader<DecStdBus>;
impl DecStdBusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DecStdBus> {
        match self.bits {
            0 => Some(DecStdBus::D0),
            1 => Some(DecStdBus::D1),
            2 => Some(DecStdBus::D2),
            3 => Some(DecStdBus::D3),
            4 => Some(DecStdBus::D4),
            5 => Some(DecStdBus::D5),
            _ => None,
        }
    }
    #[doc = "error"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == DecStdBus::D0
    }
    #[doc = "AHB master, AHB slave"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == DecStdBus::D1
    }
    #[doc = "OCP master, OCP slave"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == DecStdBus::D2
    }
    #[doc = "AXI master, AXI slave"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == DecStdBus::D3
    }
    #[doc = "AXI master, APB slave"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == DecStdBus::D4
    }
    #[doc = "AXI master, AHB slave"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == DecStdBus::D5
    }
}
#[doc = "reference buffer support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefbufExist {
    #[doc = "0: not supported"]
    B0 = 0,
    #[doc = "1: support"]
    B1 = 1,
}
impl From<RefbufExist> for bool {
    #[inline(always)]
    fn from(variant: RefbufExist) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBUF_EXIST` reader - reference buffer support"]
pub type RefbufExistR = crate::BitReader<RefbufExist>;
impl RefbufExistR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefbufExist {
        match self.bits {
            false => RefbufExist::B0,
            true => RefbufExist::B1,
        }
    }
    #[doc = "not supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RefbufExist::B0
    }
    #[doc = "support"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RefbufExist::B1
    }
}
#[doc = "output buffer selected\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutbufSel {
    #[doc = "0: 1MB buffer be used"]
    B0 = 0,
    #[doc = "1: 4MB buffer be used"]
    B1 = 1,
}
impl From<OutbufSel> for bool {
    #[inline(always)]
    fn from(variant: OutbufSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTBUF_SEL` reader - output buffer selected"]
pub type OutbufSelR = crate::BitReader<OutbufSel>;
impl OutbufSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OutbufSel {
        match self.bits {
            false => OutbufSel::B0,
            true => OutbufSel::B1,
        }
    }
    #[doc = "1MB buffer be used"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == OutbufSel::B0
    }
    #[doc = "4MB buffer be used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == OutbufSel::B1
    }
}
#[doc = "support for progressive jpeg\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecProgJpegAllow {
    #[doc = "0: no support"]
    B0 = 0,
    #[doc = "1: support"]
    B1 = 1,
}
impl From<DecProgJpegAllow> for bool {
    #[inline(always)]
    fn from(variant: DecProgJpegAllow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEC_PROG_JPEG_ALLOW` reader - support for progressive jpeg"]
pub type DecProgJpegAllowR = crate::BitReader<DecProgJpegAllow>;
impl DecProgJpegAllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DecProgJpegAllow {
        match self.bits {
            false => DecProgJpegAllow::B0,
            true => DecProgJpegAllow::B1,
        }
    }
    #[doc = "no support"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DecProgJpegAllow::B0
    }
    #[doc = "support"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DecProgJpegAllow::B1
    }
}
#[doc = "Decoding format support for VP6\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecVp6Allow {
    #[doc = "0: no support"]
    B0 = 0,
    #[doc = "1: support"]
    B1 = 1,
}
impl From<DecVp6Allow> for bool {
    #[inline(always)]
    fn from(variant: DecVp6Allow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEC_VP6_ALLOW` reader - Decoding format support for VP6"]
pub type DecVp6AllowR = crate::BitReader<DecVp6Allow>;
impl DecVp6AllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DecVp6Allow {
        match self.bits {
            false => DecVp6Allow::B0,
            true => DecVp6Allow::B1,
        }
    }
    #[doc = "no support"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DecVp6Allow::B0
    }
    #[doc = "support"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DecVp6Allow::B1
    }
}
#[doc = "h264 be support\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DecH264Allow {
    #[doc = "0: no support"]
    D0 = 0,
    #[doc = "1: baseline profile be supported"]
    D1 = 1,
    #[doc = "2: high profile be supported"]
    D2 = 2,
}
impl From<DecH264Allow> for u8 {
    #[inline(always)]
    fn from(variant: DecH264Allow) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DecH264Allow {
    type Ux = u8;
}
#[doc = "Field `DEC_H264_ALLOW` reader - h264 be support"]
pub type DecH264AllowR = crate::FieldReader<DecH264Allow>;
impl DecH264AllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DecH264Allow> {
        match self.bits {
            0 => Some(DecH264Allow::D0),
            1 => Some(DecH264Allow::D1),
            2 => Some(DecH264Allow::D2),
            _ => None,
        }
    }
    #[doc = "no support"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == DecH264Allow::D0
    }
    #[doc = "baseline profile be supported"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == DecH264Allow::D1
    }
    #[doc = "high profile be supported"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == DecH264Allow::D2
    }
}
#[doc = "Decoding format support for MPEG-4 / H.263\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DecMpeg4Allow {
    #[doc = "0: not supported"]
    D0 = 0,
    #[doc = "1: simple profile be supported"]
    D1 = 1,
    #[doc = "2: advanced simple profile be supported"]
    D2 = 2,
}
impl From<DecMpeg4Allow> for u8 {
    #[inline(always)]
    fn from(variant: DecMpeg4Allow) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DecMpeg4Allow {
    type Ux = u8;
}
#[doc = "Field `DEC_MPEG4_ALLOW` reader - Decoding format support for MPEG-4 / H.263"]
pub type DecMpeg4AllowR = crate::FieldReader<DecMpeg4Allow>;
impl DecMpeg4AllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DecMpeg4Allow> {
        match self.bits {
            0 => Some(DecMpeg4Allow::D0),
            1 => Some(DecMpeg4Allow::D1),
            2 => Some(DecMpeg4Allow::D2),
            _ => None,
        }
    }
    #[doc = "not supported"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == DecMpeg4Allow::D0
    }
    #[doc = "simple profile be supported"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == DecMpeg4Allow::D1
    }
    #[doc = "advanced simple profile be supported"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == DecMpeg4Allow::D2
    }
}
#[doc = "Decoding format support for JPEG\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecJpegAllow {
    #[doc = "0: no support"]
    B0 = 0,
    #[doc = "1: support"]
    B1 = 1,
}
impl From<DecJpegAllow> for bool {
    #[inline(always)]
    fn from(variant: DecJpegAllow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEC_JPEG_ALLOW` reader - Decoding format support for JPEG"]
pub type DecJpegAllowR = crate::BitReader<DecJpegAllow>;
impl DecJpegAllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DecJpegAllow {
        match self.bits {
            false => DecJpegAllow::B0,
            true => DecJpegAllow::B1,
        }
    }
    #[doc = "no support"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DecJpegAllow::B0
    }
    #[doc = "support"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DecJpegAllow::B1
    }
}
#[doc = "Decoding format support, for MPEG-2 / MPEG-1\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecMpeg2Allow {
    #[doc = "0: no support"]
    B0 = 0,
    #[doc = "1: support"]
    B1 = 1,
}
impl From<DecMpeg2Allow> for bool {
    #[inline(always)]
    fn from(variant: DecMpeg2Allow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEC_MPEG2_ALLOW` reader - Decoding format support, for MPEG-2 / MPEG-1"]
pub type DecMpeg2AllowR = crate::BitReader<DecMpeg2Allow>;
impl DecMpeg2AllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DecMpeg2Allow {
        match self.bits {
            false => DecMpeg2Allow::B0,
            true => DecMpeg2Allow::B1,
        }
    }
    #[doc = "no support"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DecMpeg2Allow::B0
    }
    #[doc = "support"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DecMpeg2Allow::B1
    }
}
impl R {
    #[doc = "Bits 0:10 - the max width can be decoder"]
    #[inline(always)]
    pub fn sw_dec_max_allow_w(&self) -> SwDecMaxAllowWR {
        SwDecMaxAllowWR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Decoding format support for Sorenson"]
    #[inline(always)]
    pub fn dec_srson_allow(&self) -> DecSrsonAllowR {
        DecSrsonAllowR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn bus_w(&self) -> BusWR {
        BusWR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn rtl_lan_sel(&self) -> RtlLanSelR {
        RtlLanSelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dec_std_bus(&self) -> DecStdBusR {
        DecStdBusR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - reference buffer support"]
    #[inline(always)]
    pub fn refbuf_exist(&self) -> RefbufExistR {
        RefbufExistR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - output buffer selected"]
    #[inline(always)]
    pub fn outbuf_sel(&self) -> OutbufSelR {
        OutbufSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - support for progressive jpeg"]
    #[inline(always)]
    pub fn dec_prog_jpeg_allow(&self) -> DecProgJpegAllowR {
        DecProgJpegAllowR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Decoding format support for VP6"]
    #[inline(always)]
    pub fn dec_vp6_allow(&self) -> DecVp6AllowR {
        DecVp6AllowR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - h264 be support"]
    #[inline(always)]
    pub fn dec_h264_allow(&self) -> DecH264AllowR {
        DecH264AllowR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Decoding format support for MPEG-4 / H.263"]
    #[inline(always)]
    pub fn dec_mpeg4_allow(&self) -> DecMpeg4AllowR {
        DecMpeg4AllowR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Decoding format support for JPEG"]
    #[inline(always)]
    pub fn dec_jpeg_allow(&self) -> DecJpegAllowR {
        DecJpegAllowR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Decoding format support, for MPEG-2 / MPEG-1"]
    #[inline(always)]
    pub fn dec_mpeg2_allow(&self) -> DecMpeg2AllowR {
        DecMpeg2AllowR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "information for read only register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg71::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg71Spec;
impl crate::RegisterSpec for Swreg71Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg71::R`](R) reader structure"]
impl crate::Readable for Swreg71Spec {}
#[doc = "`reset()` method sets SWREG71 to value 0xfbb5_6f80"]
impl crate::Resettable for Swreg71Spec {
    const RESET_VALUE: u32 = 0xfbb5_6f80;
}

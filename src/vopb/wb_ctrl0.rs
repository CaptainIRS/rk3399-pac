#[doc = "Register `WB_CTRL0` reader"]
pub type R = crate::R<WbCtrl0Spec>;
#[doc = "Register `WB_CTRL0` writer"]
pub type W = crate::W<WbCtrl0Spec>;
#[doc = "write back enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WbEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<WbEn> for bool {
    #[inline(always)]
    fn from(variant: WbEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WB_EN` reader - write back enable"]
pub type WbEnR = crate::BitReader<WbEn>;
impl WbEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WbEn {
        match self.bits {
            false => WbEn::B0,
            true => WbEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WbEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WbEn::B1
    }
}
#[doc = "Field `WB_EN` writer - write back enable"]
pub type WbEnW<'a, REG> = crate::BitWriter<'a, REG, WbEn>;
impl<'a, REG> WbEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WbEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WbEn::B1)
    }
}
#[doc = "write back format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WbFmt {
    #[doc = "0: ARGB888"]
    B000 = 0,
    #[doc = "1: RGB888"]
    B001 = 1,
    #[doc = "2: RGB565"]
    B010 = 2,
    #[doc = "4: YcbCr420 other : reserved"]
    B100 = 4,
}
impl From<WbFmt> for u8 {
    #[inline(always)]
    fn from(variant: WbFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WbFmt {
    type Ux = u8;
}
#[doc = "Field `WB_FMT` reader - write back format"]
pub type WbFmtR = crate::FieldReader<WbFmt>;
impl WbFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WbFmt> {
        match self.bits {
            0 => Some(WbFmt::B000),
            1 => Some(WbFmt::B001),
            2 => Some(WbFmt::B010),
            4 => Some(WbFmt::B100),
            _ => None,
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == WbFmt::B000
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == WbFmt::B001
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == WbFmt::B010
    }
    #[doc = "YcbCr420 other : reserved"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == WbFmt::B100
    }
}
#[doc = "Field `WB_FMT` writer - write back format"]
pub type WbFmtW<'a, REG> = crate::FieldWriter<'a, REG, 3, WbFmt>;
impl<'a, REG> WbFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(WbFmt::B000)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(WbFmt::B001)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(WbFmt::B010)
    }
    #[doc = "YcbCr420 other : reserved"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(WbFmt::B100)
    }
}
#[doc = "write back dither enable\n\nwhen wb_fmt is RGB565.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WbDitherEn {
    #[doc = "0: no dither ,RGB888 clip to RGB565"]
    B0 = 0,
    #[doc = "1: with dither,RGB888 dither to RGB565"]
    B1 = 1,
}
impl From<WbDitherEn> for bool {
    #[inline(always)]
    fn from(variant: WbDitherEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WB_DITHER_EN` reader - write back dither enable\n\nwhen wb_fmt is RGB565."]
pub type WbDitherEnR = crate::BitReader<WbDitherEn>;
impl WbDitherEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WbDitherEn {
        match self.bits {
            false => WbDitherEn::B0,
            true => WbDitherEn::B1,
        }
    }
    #[doc = "no dither ,RGB888 clip to RGB565"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WbDitherEn::B0
    }
    #[doc = "with dither,RGB888 dither to RGB565"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WbDitherEn::B1
    }
}
#[doc = "Field `WB_DITHER_EN` writer - write back dither enable\n\nwhen wb_fmt is RGB565."]
pub type WbDitherEnW<'a, REG> = crate::BitWriter<'a, REG, WbDitherEn>;
impl<'a, REG> WbDitherEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no dither ,RGB888 clip to RGB565"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WbDitherEn::B0)
    }
    #[doc = "with dither,RGB888 dither to RGB565"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WbDitherEn::B1)
    }
}
#[doc = "write back rgb to yuv enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WbRgb2yuvEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<WbRgb2yuvEn> for bool {
    #[inline(always)]
    fn from(variant: WbRgb2yuvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WB_RGB2YUV_EN` reader - write back rgb to yuv enable"]
pub type WbRgb2yuvEnR = crate::BitReader<WbRgb2yuvEn>;
impl WbRgb2yuvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WbRgb2yuvEn {
        match self.bits {
            false => WbRgb2yuvEn::B0,
            true => WbRgb2yuvEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WbRgb2yuvEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WbRgb2yuvEn::B1
    }
}
#[doc = "Field `WB_RGB2YUV_EN` writer - write back rgb to yuv enable"]
pub type WbRgb2yuvEnW<'a, REG> = crate::BitWriter<'a, REG, WbRgb2yuvEn>;
impl<'a, REG> WbRgb2yuvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WbRgb2yuvEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WbRgb2yuvEn::B1)
    }
}
#[doc = "write back rgb to yuv mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WbRgb2yuvMode {
    #[doc = "0: BT601"]
    B0 = 0,
    #[doc = "1: BT709"]
    B1 = 1,
}
impl From<WbRgb2yuvMode> for bool {
    #[inline(always)]
    fn from(variant: WbRgb2yuvMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WB_RGB2YUV_MODE` reader - write back rgb to yuv mode"]
pub type WbRgb2yuvModeR = crate::BitReader<WbRgb2yuvMode>;
impl WbRgb2yuvModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WbRgb2yuvMode {
        match self.bits {
            false => WbRgb2yuvMode::B0,
            true => WbRgb2yuvMode::B1,
        }
    }
    #[doc = "BT601"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WbRgb2yuvMode::B0
    }
    #[doc = "BT709"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WbRgb2yuvMode::B1
    }
}
#[doc = "Field `WB_RGB2YUV_MODE` writer - write back rgb to yuv mode"]
pub type WbRgb2yuvModeW<'a, REG> = crate::BitWriter<'a, REG, WbRgb2yuvMode>;
impl<'a, REG> WbRgb2yuvModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BT601"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WbRgb2yuvMode::B0)
    }
    #[doc = "BT709"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WbRgb2yuvMode::B1)
    }
}
#[doc = "write back X direction bilinear scale enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WbXpsdBilEn {
    #[doc = "0: enable scale"]
    B0 = 0,
    #[doc = "1: disable scale"]
    B1 = 1,
}
impl From<WbXpsdBilEn> for bool {
    #[inline(always)]
    fn from(variant: WbXpsdBilEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WB_XPSD_BIL_EN` reader - write back X direction bilinear scale enable"]
pub type WbXpsdBilEnR = crate::BitReader<WbXpsdBilEn>;
impl WbXpsdBilEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WbXpsdBilEn {
        match self.bits {
            false => WbXpsdBilEn::B0,
            true => WbXpsdBilEn::B1,
        }
    }
    #[doc = "enable scale"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WbXpsdBilEn::B0
    }
    #[doc = "disable scale"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WbXpsdBilEn::B1
    }
}
#[doc = "Field `WB_XPSD_BIL_EN` writer - write back X direction bilinear scale enable"]
pub type WbXpsdBilEnW<'a, REG> = crate::BitWriter<'a, REG, WbXpsdBilEn>;
impl<'a, REG> WbXpsdBilEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable scale"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WbXpsdBilEn::B0)
    }
    #[doc = "disable scale"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WbXpsdBilEn::B1)
    }
}
#[doc = "write back y direction throw line enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WbYthrowEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<WbYthrowEn> for bool {
    #[inline(always)]
    fn from(variant: WbYthrowEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WB_YTHROW_EN` reader - write back y direction throw line enable"]
pub type WbYthrowEnR = crate::BitReader<WbYthrowEn>;
impl WbYthrowEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WbYthrowEn {
        match self.bits {
            false => WbYthrowEn::B0,
            true => WbYthrowEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WbYthrowEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WbYthrowEn::B1
    }
}
#[doc = "Field `WB_YTHROW_EN` writer - write back y direction throw line enable"]
pub type WbYthrowEnW<'a, REG> = crate::BitWriter<'a, REG, WbYthrowEn>;
impl<'a, REG> WbYthrowEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WbYthrowEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WbYthrowEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WbYthrowMode {
    #[doc = "0: throw odd line"]
    B0 = 0,
    #[doc = "1: throw even line"]
    B1 = 1,
}
impl From<WbYthrowMode> for bool {
    #[inline(always)]
    fn from(variant: WbYthrowMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WB_YTHROW_MODE` reader - "]
pub type WbYthrowModeR = crate::BitReader<WbYthrowMode>;
impl WbYthrowModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WbYthrowMode {
        match self.bits {
            false => WbYthrowMode::B0,
            true => WbYthrowMode::B1,
        }
    }
    #[doc = "throw odd line"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WbYthrowMode::B0
    }
    #[doc = "throw even line"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WbYthrowMode::B1
    }
}
#[doc = "Field `WB_YTHROW_MODE` writer - "]
pub type WbYthrowModeW<'a, REG> = crate::BitWriter<'a, REG, WbYthrowMode>;
impl<'a, REG> WbYthrowModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "throw odd line"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WbYthrowMode::B0)
    }
    #[doc = "throw even line"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WbYthrowMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WbHandshakeMode {
    #[doc = "0: full handshake"]
    B0 = 0,
    #[doc = "1: half handshake"]
    B1 = 1,
}
impl From<WbHandshakeMode> for bool {
    #[inline(always)]
    fn from(variant: WbHandshakeMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WB_HANDSHAKE_MODE` reader - "]
pub type WbHandshakeModeR = crate::BitReader<WbHandshakeMode>;
impl WbHandshakeModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WbHandshakeMode {
        match self.bits {
            false => WbHandshakeMode::B0,
            true => WbHandshakeMode::B1,
        }
    }
    #[doc = "full handshake"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WbHandshakeMode::B0
    }
    #[doc = "half handshake"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WbHandshakeMode::B1
    }
}
#[doc = "Field `WB_HANDSHAKE_MODE` writer - "]
pub type WbHandshakeModeW<'a, REG> = crate::BitWriter<'a, REG, WbHandshakeMode>;
impl<'a, REG> WbHandshakeModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "full handshake"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WbHandshakeMode::B0)
    }
    #[doc = "half handshake"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WbHandshakeMode::B1)
    }
}
#[doc = "Field `WB_YRGB_ID` reader - axi bus write back yrgb id\n\nuse default 0xd."]
pub type WbYrgbIdR = crate::FieldReader;
#[doc = "Field `WB_YRGB_ID` writer - axi bus write back yrgb id\n\nuse default 0xd."]
pub type WbYrgbIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WB_UV_ID` reader - axi bus write back yrgb id\n\nuse default 0xe."]
pub type WbUvIdR = crate::FieldReader;
#[doc = "Field `WB_UV_ID` writer - axi bus write back yrgb id\n\nuse default 0xe."]
pub type WbUvIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - write back enable"]
    #[inline(always)]
    pub fn wb_en(&self) -> WbEnR {
        WbEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - write back format"]
    #[inline(always)]
    pub fn wb_fmt(&self) -> WbFmtR {
        WbFmtR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - write back dither enable\n\nwhen wb_fmt is RGB565."]
    #[inline(always)]
    pub fn wb_dither_en(&self) -> WbDitherEnR {
        WbDitherEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write back rgb to yuv enable"]
    #[inline(always)]
    pub fn wb_rgb2yuv_en(&self) -> WbRgb2yuvEnR {
        WbRgb2yuvEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write back rgb to yuv mode"]
    #[inline(always)]
    pub fn wb_rgb2yuv_mode(&self) -> WbRgb2yuvModeR {
        WbRgb2yuvModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write back X direction bilinear scale enable"]
    #[inline(always)]
    pub fn wb_xpsd_bil_en(&self) -> WbXpsdBilEnR {
        WbXpsdBilEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write back y direction throw line enable"]
    #[inline(always)]
    pub fn wb_ythrow_en(&self) -> WbYthrowEnR {
        WbYthrowEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn wb_ythrow_mode(&self) -> WbYthrowModeR {
        WbYthrowModeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wb_handshake_mode(&self) -> WbHandshakeModeR {
        WbHandshakeModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 24:27 - axi bus write back yrgb id\n\nuse default 0xd."]
    #[inline(always)]
    pub fn wb_yrgb_id(&self) -> WbYrgbIdR {
        WbYrgbIdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - axi bus write back yrgb id\n\nuse default 0xe."]
    #[inline(always)]
    pub fn wb_uv_id(&self) -> WbUvIdR {
        WbUvIdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - write back enable"]
    #[inline(always)]
    #[must_use]
    pub fn wb_en(&mut self) -> WbEnW<WbCtrl0Spec> {
        WbEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - write back format"]
    #[inline(always)]
    #[must_use]
    pub fn wb_fmt(&mut self) -> WbFmtW<WbCtrl0Spec> {
        WbFmtW::new(self, 1)
    }
    #[doc = "Bit 4 - write back dither enable\n\nwhen wb_fmt is RGB565."]
    #[inline(always)]
    #[must_use]
    pub fn wb_dither_en(&mut self) -> WbDitherEnW<WbCtrl0Spec> {
        WbDitherEnW::new(self, 4)
    }
    #[doc = "Bit 5 - write back rgb to yuv enable"]
    #[inline(always)]
    #[must_use]
    pub fn wb_rgb2yuv_en(&mut self) -> WbRgb2yuvEnW<WbCtrl0Spec> {
        WbRgb2yuvEnW::new(self, 5)
    }
    #[doc = "Bit 6 - write back rgb to yuv mode"]
    #[inline(always)]
    #[must_use]
    pub fn wb_rgb2yuv_mode(&mut self) -> WbRgb2yuvModeW<WbCtrl0Spec> {
        WbRgb2yuvModeW::new(self, 6)
    }
    #[doc = "Bit 7 - write back X direction bilinear scale enable"]
    #[inline(always)]
    #[must_use]
    pub fn wb_xpsd_bil_en(&mut self) -> WbXpsdBilEnW<WbCtrl0Spec> {
        WbXpsdBilEnW::new(self, 7)
    }
    #[doc = "Bit 8 - write back y direction throw line enable"]
    #[inline(always)]
    #[must_use]
    pub fn wb_ythrow_en(&mut self) -> WbYthrowEnW<WbCtrl0Spec> {
        WbYthrowEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn wb_ythrow_mode(&mut self) -> WbYthrowModeW<WbCtrl0Spec> {
        WbYthrowModeW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn wb_handshake_mode(&mut self) -> WbHandshakeModeW<WbCtrl0Spec> {
        WbHandshakeModeW::new(self, 11)
    }
    #[doc = "Bits 24:27 - axi bus write back yrgb id\n\nuse default 0xd."]
    #[inline(always)]
    #[must_use]
    pub fn wb_yrgb_id(&mut self) -> WbYrgbIdW<WbCtrl0Spec> {
        WbYrgbIdW::new(self, 24)
    }
    #[doc = "Bits 28:31 - axi bus write back yrgb id\n\nuse default 0xe."]
    #[inline(always)]
    #[must_use]
    pub fn wb_uv_id(&mut self) -> WbUvIdW<WbCtrl0Spec> {
        WbUvIdW::new(self, 28)
    }
}
#[doc = "write back ctrl0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wb_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wb_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WbCtrl0Spec;
impl crate::RegisterSpec for WbCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wb_ctrl0::R`](R) reader structure"]
impl crate::Readable for WbCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`wb_ctrl0::W`](W) writer structure"]
impl crate::Writable for WbCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WB_CTRL0 to value 0xed00_0000"]
impl crate::Resettable for WbCtrl0Spec {
    const RESET_VALUE: u32 = 0xed00_0000;
}

#[doc = "Register `SWREG38` reader"]
pub type R = crate::R<Swreg38Spec>;
#[doc = "Register `SWREG38` writer"]
pub type W = crate::W<Swreg38Spec>;
#[doc = "Rotation mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwRotMode {
    #[doc = "0: rotation disabled"]
    D0 = 0,
    #[doc = "1: rotate + 90"]
    D1 = 1,
    #[doc = "2: rotate –90"]
    D2 = 2,
    #[doc = "3: horizontal flip (mirror)"]
    D3 = 3,
    #[doc = "4: vertical flip"]
    D4 = 4,
    #[doc = "5: rotate 180"]
    D5 = 5,
}
impl From<SwRotMode> for u8 {
    #[inline(always)]
    fn from(variant: SwRotMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwRotMode {
    type Ux = u8;
}
#[doc = "Field `SW_ROT_MODE` reader - Rotation mode"]
pub type SwRotModeR = crate::FieldReader<SwRotMode>;
impl SwRotModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwRotMode> {
        match self.bits {
            0 => Some(SwRotMode::D0),
            1 => Some(SwRotMode::D1),
            2 => Some(SwRotMode::D2),
            3 => Some(SwRotMode::D3),
            4 => Some(SwRotMode::D4),
            5 => Some(SwRotMode::D5),
            _ => None,
        }
    }
    #[doc = "rotation disabled"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SwRotMode::D0
    }
    #[doc = "rotate + 90"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SwRotMode::D1
    }
    #[doc = "rotate –90"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SwRotMode::D2
    }
    #[doc = "horizontal flip (mirror)"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == SwRotMode::D3
    }
    #[doc = "vertical flip"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == SwRotMode::D4
    }
    #[doc = "rotate 180"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == SwRotMode::D5
    }
}
#[doc = "Field `SW_ROT_MODE` writer - Rotation mode"]
pub type SwRotModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, SwRotMode>;
impl<'a, REG> SwRotModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rotation disabled"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRotMode::D0)
    }
    #[doc = "rotate + 90"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRotMode::D1)
    }
    #[doc = "rotate –90"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(SwRotMode::D2)
    }
    #[doc = "horizontal flip (mirror)"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(SwRotMode::D3)
    }
    #[doc = "vertical flip"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(SwRotMode::D4)
    }
    #[doc = "rotate 180"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(SwRotMode::D5)
    }
}
#[doc = "the input format of pp input data\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwPpInFmt {
    #[doc = "0: YUYV 4:2:2 ; interleaved and it only supported in external mode"]
    D0 = 0,
    #[doc = "1: YCbCr 4:2:0 ; the format of Semi-planar in linear raster-scan"]
    D1 = 1,
    #[doc = "2: YCbCr 4:2:0 ; planar and it only supported in external mode"]
    D2 = 2,
    #[doc = "3: YCbCr 4:0:0 ; it only supported in pipelined mode"]
    D3 = 3,
    #[doc = "4: YCbCr 4:2:2 ; Semi-planar and it only supported only in pipelined mode"]
    D4 = 4,
    #[doc = "5: YCbCr 4:2:0 ; Semi-planar in tiled format and it only supported in external mode"]
    D5 = 5,
    #[doc = "6: YCbCr 4:4:0 ; Semi-planar and it only supported for jpeg in pipelined mode"]
    D6 = 6,
    #[doc = "7: same as sw_pp_in_fmt_ecp"]
    D7 = 7,
}
impl From<SwPpInFmt> for u8 {
    #[inline(always)]
    fn from(variant: SwPpInFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwPpInFmt {
    type Ux = u8;
}
#[doc = "Field `SW_PP_IN_FMT` reader - the input format of pp input data"]
pub type SwPpInFmtR = crate::FieldReader<SwPpInFmt>;
impl SwPpInFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpInFmt {
        match self.bits {
            0 => SwPpInFmt::D0,
            1 => SwPpInFmt::D1,
            2 => SwPpInFmt::D2,
            3 => SwPpInFmt::D3,
            4 => SwPpInFmt::D4,
            5 => SwPpInFmt::D5,
            6 => SwPpInFmt::D6,
            7 => SwPpInFmt::D7,
            _ => unreachable!(),
        }
    }
    #[doc = "YUYV 4:2:2 ; interleaved and it only supported in external mode"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SwPpInFmt::D0
    }
    #[doc = "YCbCr 4:2:0 ; the format of Semi-planar in linear raster-scan"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SwPpInFmt::D1
    }
    #[doc = "YCbCr 4:2:0 ; planar and it only supported in external mode"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SwPpInFmt::D2
    }
    #[doc = "YCbCr 4:0:0 ; it only supported in pipelined mode"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == SwPpInFmt::D3
    }
    #[doc = "YCbCr 4:2:2 ; Semi-planar and it only supported only in pipelined mode"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == SwPpInFmt::D4
    }
    #[doc = "YCbCr 4:2:0 ; Semi-planar in tiled format and it only supported in external mode"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == SwPpInFmt::D5
    }
    #[doc = "YCbCr 4:4:0 ; Semi-planar and it only supported for jpeg in pipelined mode"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == SwPpInFmt::D6
    }
    #[doc = "same as sw_pp_in_fmt_ecp"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == SwPpInFmt::D7
    }
}
#[doc = "Field `SW_PP_IN_FMT` writer - the input format of pp input data"]
pub type SwPpInFmtW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SwPpInFmt>;
impl<'a, REG> SwPpInFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "YUYV 4:2:2 ; interleaved and it only supported in external mode"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmt::D0)
    }
    #[doc = "YCbCr 4:2:0 ; the format of Semi-planar in linear raster-scan"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmt::D1)
    }
    #[doc = "YCbCr 4:2:0 ; planar and it only supported in external mode"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmt::D2)
    }
    #[doc = "YCbCr 4:0:0 ; it only supported in pipelined mode"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmt::D3)
    }
    #[doc = "YCbCr 4:2:2 ; Semi-planar and it only supported only in pipelined mode"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmt::D4)
    }
    #[doc = "YCbCr 4:2:0 ; Semi-planar in tiled format and it only supported in external mode"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmt::D5)
    }
    #[doc = "YCbCr 4:4:0 ; Semi-planar and it only supported for jpeg in pipelined mode"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmt::D6)
    }
    #[doc = "same as sw_pp_in_fmt_ecp"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmt::D7)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwPpOutFmt {
    #[doc = "0: RGB"]
    D0 = 0,
    #[doc = "1: YCbCr 4:2:0 ; planar (Not supported)"]
    D1 = 1,
    #[doc = "2: YCbCr 4:2:2 ; planar (Not supported)"]
    D2 = 2,
    #[doc = "3: YUYV 4:2:2 ; interleaved"]
    D3 = 3,
    #[doc = "4: YCbCr 4:4:4 ; planar (Not supported)"]
    D4 = 4,
    #[doc = "5: YCh 4:2:0 ; chrominance interleaved"]
    D5 = 5,
    #[doc = "6: YCh 4:2:2 ; (Not supported)"]
    D6 = 6,
    #[doc = "7: YCh 4:4:4 (Not supported)"]
    D7 = 7,
}
impl From<SwPpOutFmt> for u8 {
    #[inline(always)]
    fn from(variant: SwPpOutFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwPpOutFmt {
    type Ux = u8;
}
#[doc = "Field `SW_PP_OUT_FMT` reader - "]
pub type SwPpOutFmtR = crate::FieldReader<SwPpOutFmt>;
impl SwPpOutFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpOutFmt {
        match self.bits {
            0 => SwPpOutFmt::D0,
            1 => SwPpOutFmt::D1,
            2 => SwPpOutFmt::D2,
            3 => SwPpOutFmt::D3,
            4 => SwPpOutFmt::D4,
            5 => SwPpOutFmt::D5,
            6 => SwPpOutFmt::D6,
            7 => SwPpOutFmt::D7,
            _ => unreachable!(),
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SwPpOutFmt::D0
    }
    #[doc = "YCbCr 4:2:0 ; planar (Not supported)"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SwPpOutFmt::D1
    }
    #[doc = "YCbCr 4:2:2 ; planar (Not supported)"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SwPpOutFmt::D2
    }
    #[doc = "YUYV 4:2:2 ; interleaved"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == SwPpOutFmt::D3
    }
    #[doc = "YCbCr 4:4:4 ; planar (Not supported)"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == SwPpOutFmt::D4
    }
    #[doc = "YCh 4:2:0 ; chrominance interleaved"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == SwPpOutFmt::D5
    }
    #[doc = "YCh 4:2:2 ; (Not supported)"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == SwPpOutFmt::D6
    }
    #[doc = "YCh 4:4:4 (Not supported)"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == SwPpOutFmt::D7
    }
}
#[doc = "Field `SW_PP_OUT_FMT` writer - "]
pub type SwPpOutFmtW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SwPpOutFmt>;
impl<'a, REG> SwPpOutFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutFmt::D0)
    }
    #[doc = "YCbCr 4:2:0 ; planar (Not supported)"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutFmt::D1)
    }
    #[doc = "YCbCr 4:2:2 ; planar (Not supported)"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutFmt::D2)
    }
    #[doc = "YUYV 4:2:2 ; interleaved"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutFmt::D3)
    }
    #[doc = "YCbCr 4:4:4 ; planar (Not supported)"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutFmt::D4)
    }
    #[doc = "YCh 4:2:0 ; chrominance interleaved"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutFmt::D5)
    }
    #[doc = "YCh 4:2:2 ; (Not supported)"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutFmt::D6)
    }
    #[doc = "YCh 4:4:4 (Not supported)"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutFmt::D7)
    }
}
#[doc = "Escape PP in format\n\nbe actived when sw_pp_in_fmt = 3'b111\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwPpInFmtEcp {
    #[doc = "0: YCbCr 4:4:4"]
    B0 = 0,
    #[doc = "1: YCbCr 4:1:1"]
    B1 = 1,
}
impl From<SwPpInFmtEcp> for u8 {
    #[inline(always)]
    fn from(variant: SwPpInFmtEcp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwPpInFmtEcp {
    type Ux = u8;
}
#[doc = "Field `SW_PP_IN_FMT_ECP` reader - Escape PP in format\n\nbe actived when sw_pp_in_fmt = 3'b111"]
pub type SwPpInFmtEcpR = crate::FieldReader<SwPpInFmtEcp>;
impl SwPpInFmtEcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwPpInFmtEcp> {
        match self.bits {
            0 => Some(SwPpInFmtEcp::B0),
            1 => Some(SwPpInFmtEcp::B1),
            _ => None,
        }
    }
    #[doc = "YCbCr 4:4:4"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpInFmtEcp::B0
    }
    #[doc = "YCbCr 4:1:1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpInFmtEcp::B1
    }
}
#[doc = "Field `SW_PP_IN_FMT_ECP` writer - Escape PP in format\n\nbe actived when sw_pp_in_fmt = 3'b111"]
pub type SwPpInFmtEcpW<'a, REG> = crate::FieldWriter<'a, REG, 3, SwPpInFmtEcp>;
impl<'a, REG> SwPpInFmtEcpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "YCbCr 4:4:4"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmtEcp::B0)
    }
    #[doc = "YCbCr 4:1:1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInFmtEcp::B1)
    }
}
#[doc = "the tiled mode of pp input data\n\nonly support yuv420 input data,can be as pipeline or external mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwPpInTilmod {
    #[doc = "0: Tiled mode not be actived"]
    B0 = 0,
    #[doc = "1: 8x4 sized tiles be used 2,3 : reserved"]
    B1 = 1,
}
impl From<SwPpInTilmod> for u8 {
    #[inline(always)]
    fn from(variant: SwPpInTilmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwPpInTilmod {
    type Ux = u8;
}
#[doc = "Field `SW_PP_IN_TILMOD` reader - the tiled mode of pp input data\n\nonly support yuv420 input data,can be as pipeline or external mode"]
pub type SwPpInTilmodR = crate::FieldReader<SwPpInTilmod>;
impl SwPpInTilmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwPpInTilmod> {
        match self.bits {
            0 => Some(SwPpInTilmod::B0),
            1 => Some(SwPpInTilmod::B1),
            _ => None,
        }
    }
    #[doc = "Tiled mode not be actived"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpInTilmod::B0
    }
    #[doc = "8x4 sized tiles be used 2,3 : reserved"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpInTilmod::B1
    }
}
#[doc = "Field `SW_PP_IN_TILMOD` writer - the tiled mode of pp input data\n\nonly support yuv420 input data,can be as pipeline or external mode"]
pub type SwPpInTilmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, SwPpInTilmod>;
impl<'a, REG> SwPpInTilmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Tiled mode not be actived"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInTilmod::B0)
    }
    #[doc = "8x4 sized tiles be used 2,3 : reserved"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInTilmod::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Rotation mode"]
    #[inline(always)]
    pub fn sw_rot_mode(&self) -> SwRotModeR {
        SwRotModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - the input format of pp input data"]
    #[inline(always)]
    pub fn sw_pp_in_fmt(&self) -> SwPpInFmtR {
        SwPpInFmtR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn sw_pp_out_fmt(&self) -> SwPpOutFmtR {
        SwPpOutFmtR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Escape PP in format\n\nbe actived when sw_pp_in_fmt = 3'b111"]
    #[inline(always)]
    pub fn sw_pp_in_fmt_ecp(&self) -> SwPpInFmtEcpR {
        SwPpInFmtEcpR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - the tiled mode of pp input data\n\nonly support yuv420 input data,can be as pipeline or external mode"]
    #[inline(always)]
    pub fn sw_pp_in_tilmod(&self) -> SwPpInTilmodR {
        SwPpInTilmodR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rotation mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rot_mode(&mut self) -> SwRotModeW<Swreg38Spec> {
        SwRotModeW::new(self, 0)
    }
    #[doc = "Bits 8:10 - the input format of pp input data"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_in_fmt(&mut self) -> SwPpInFmtW<Swreg38Spec> {
        SwPpInFmtW::new(self, 8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_out_fmt(&mut self) -> SwPpOutFmtW<Swreg38Spec> {
        SwPpOutFmtW::new(self, 11)
    }
    #[doc = "Bits 16:18 - Escape PP in format\n\nbe actived when sw_pp_in_fmt = 3'b111"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_in_fmt_ecp(&mut self) -> SwPpInFmtEcpW<Swreg38Spec> {
        SwPpInFmtEcpW::new(self, 16)
    }
    #[doc = "Bits 20:21 - the tiled mode of pp input data\n\nonly support yuv420 input data,can be as pipeline or external mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_in_tilmod(&mut self) -> SwPpInTilmodW<Swreg38Spec> {
        SwPpInTilmodW::new(self, 20)
    }
}
#[doc = "PP input/output data format\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg38Spec;
impl crate::RegisterSpec for Swreg38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg38::R`](R) reader structure"]
impl crate::Readable for Swreg38Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg38::W`](W) writer structure"]
impl crate::Writable for Swreg38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG38 to value 0"]
impl crate::Resettable for Swreg38Spec {
    const RESET_VALUE: u32 = 0;
}

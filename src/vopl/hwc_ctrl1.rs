#[doc = "Register `HWC_CTRL1` reader"]
pub type R = crate::R<HwcCtrl1Spec>;
#[doc = "Register `HWC_CTRL1` writer"]
pub type W = crate::W<HwcCtrl1Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcAxiGatherEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<HwcAxiGatherEn> for bool {
    #[inline(always)]
    fn from(variant: HwcAxiGatherEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_AXI_GATHER_EN` reader - "]
pub type HwcAxiGatherEnR = crate::BitReader<HwcAxiGatherEn>;
impl HwcAxiGatherEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcAxiGatherEn {
        match self.bits {
            false => HwcAxiGatherEn::B0,
            true => HwcAxiGatherEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcAxiGatherEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcAxiGatherEn::B1
    }
}
#[doc = "Field `HWC_AXI_GATHER_EN` writer - "]
pub type HwcAxiGatherEnW<'a, REG> = crate::BitWriter<'a, REG, HwcAxiGatherEn>;
impl<'a, REG> HwcAxiGatherEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcAxiGatherEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcAxiGatherEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcAxiMaxOutstandingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<HwcAxiMaxOutstandingEn> for bool {
    #[inline(always)]
    fn from(variant: HwcAxiMaxOutstandingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_AXI_MAX_OUTSTANDING_EN` reader - "]
pub type HwcAxiMaxOutstandingEnR = crate::BitReader<HwcAxiMaxOutstandingEn>;
impl HwcAxiMaxOutstandingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcAxiMaxOutstandingEn {
        match self.bits {
            false => HwcAxiMaxOutstandingEn::B0,
            true => HwcAxiMaxOutstandingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcAxiMaxOutstandingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcAxiMaxOutstandingEn::B1
    }
}
#[doc = "Field `HWC_AXI_MAX_OUTSTANDING_EN` writer - "]
pub type HwcAxiMaxOutstandingEnW<'a, REG> = crate::BitWriter<'a, REG, HwcAxiMaxOutstandingEn>;
impl<'a, REG> HwcAxiMaxOutstandingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcAxiMaxOutstandingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcAxiMaxOutstandingEn::B1)
    }
}
#[doc = "HWC DMA read Burst length\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HwcDmaBurstLength {
    #[doc = "0: burst16 (burst 15 in rgb888 pack mode)"]
    B00 = 0,
    #[doc = "1: burst8 (burst 12 in rgb888 pack mode)"]
    B01 = 1,
    #[doc = "2: burst4 (burst 6 in rgb888 pack mode)"]
    B10 = 2,
}
impl From<HwcDmaBurstLength> for u8 {
    #[inline(always)]
    fn from(variant: HwcDmaBurstLength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HwcDmaBurstLength {
    type Ux = u8;
}
#[doc = "Field `HWC_DMA_BURST_LENGTH` reader - HWC DMA read Burst length"]
pub type HwcDmaBurstLengthR = crate::FieldReader<HwcDmaBurstLength>;
impl HwcDmaBurstLengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HwcDmaBurstLength> {
        match self.bits {
            0 => Some(HwcDmaBurstLength::B00),
            1 => Some(HwcDmaBurstLength::B01),
            2 => Some(HwcDmaBurstLength::B10),
            _ => None,
        }
    }
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == HwcDmaBurstLength::B00
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == HwcDmaBurstLength::B01
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == HwcDmaBurstLength::B10
    }
}
#[doc = "Field `HWC_DMA_BURST_LENGTH` writer - HWC DMA read Burst length"]
pub type HwcDmaBurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2, HwcDmaBurstLength>;
impl<'a, REG> HwcDmaBurstLengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDmaBurstLength::B00)
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDmaBurstLength::B01)
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDmaBurstLength::B10)
    }
}
#[doc = "Field `HWC_AXI_GATHER_NUM` reader - hwc axi gather transfer number"]
pub type HwcAxiGatherNumR = crate::FieldReader;
#[doc = "Field `HWC_AXI_GATHER_NUM` writer - hwc axi gather transfer number"]
pub type HwcAxiGatherNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HWC_AXI_MAX_OUTSTANDING_NUM` reader - hwc axi bus max outstanding number"]
pub type HwcAxiMaxOutstandingNumR = crate::FieldReader;
#[doc = "Field `HWC_AXI_MAX_OUTSTANDING_NUM` writer - hwc axi bus max outstanding number"]
pub type HwcAxiMaxOutstandingNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcRgb2yuvEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: disable"]
    B1 = 1,
}
impl From<HwcRgb2yuvEn> for bool {
    #[inline(always)]
    fn from(variant: HwcRgb2yuvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_RGB2YUV_EN` reader - "]
pub type HwcRgb2yuvEnR = crate::BitReader<HwcRgb2yuvEn>;
impl HwcRgb2yuvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcRgb2yuvEn {
        match self.bits {
            false => HwcRgb2yuvEn::B0,
            true => HwcRgb2yuvEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcRgb2yuvEn::B0
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcRgb2yuvEn::B1
    }
}
#[doc = "Field `HWC_RGB2YUV_EN` writer - "]
pub type HwcRgb2yuvEnW<'a, REG> = crate::BitWriter<'a, REG, HwcRgb2yuvEn>;
impl<'a, REG> HwcRgb2yuvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcRgb2yuvEn::B0)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcRgb2yuvEn::B1)
    }
}
#[doc = "hwc AXI master read outstanding\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcNoOutstanding {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: disable"]
    B1 = 1,
}
impl From<HwcNoOutstanding> for bool {
    #[inline(always)]
    fn from(variant: HwcNoOutstanding) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_NO_OUTSTANDING` reader - hwc AXI master read outstanding"]
pub type HwcNoOutstandingR = crate::BitReader<HwcNoOutstanding>;
impl HwcNoOutstandingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcNoOutstanding {
        match self.bits {
            false => HwcNoOutstanding::B0,
            true => HwcNoOutstanding::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcNoOutstanding::B0
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcNoOutstanding::B1
    }
}
#[doc = "Field `HWC_NO_OUTSTANDING` writer - hwc AXI master read outstanding"]
pub type HwcNoOutstandingW<'a, REG> = crate::BitWriter<'a, REG, HwcNoOutstanding>;
impl<'a, REG> HwcNoOutstandingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcNoOutstanding::B0)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcNoOutstanding::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcYMirEn {
    #[doc = "0: no y_mirror"]
    B0 = 0,
    #[doc = "1: y_mirror"]
    B1 = 1,
}
impl From<HwcYMirEn> for bool {
    #[inline(always)]
    fn from(variant: HwcYMirEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_Y_MIR_EN` reader - "]
pub type HwcYMirEnR = crate::BitReader<HwcYMirEn>;
impl HwcYMirEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcYMirEn {
        match self.bits {
            false => HwcYMirEn::B0,
            true => HwcYMirEn::B1,
        }
    }
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcYMirEn::B0
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcYMirEn::B1
    }
}
#[doc = "Field `HWC_Y_MIR_EN` writer - "]
pub type HwcYMirEnW<'a, REG> = crate::BitWriter<'a, REG, HwcYMirEn>;
impl<'a, REG> HwcYMirEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcYMirEn::B0)
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcYMirEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcLutEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<HwcLutEn> for bool {
    #[inline(always)]
    fn from(variant: HwcLutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_LUT_EN` reader - "]
pub type HwcLutEnR = crate::BitReader<HwcLutEn>;
impl HwcLutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcLutEn {
        match self.bits {
            false => HwcLutEn::B0,
            true => HwcLutEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcLutEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcLutEn::B1
    }
}
#[doc = "Field `HWC_LUT_EN` writer - "]
pub type HwcLutEnW<'a, REG> = crate::BitWriter<'a, REG, HwcLutEn>;
impl<'a, REG> HwcLutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcLutEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcLutEn::B1)
    }
}
#[doc = "Field `WIN_RID_HWC` reader - axi read id of hwc channel"]
pub type WinRidHwcR = crate::FieldReader;
#[doc = "Field `WIN_RID_HWC` writer - axi read id of hwc channel"]
pub type WinRidHwcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hwc_axi_gather_en(&self) -> HwcAxiGatherEnR {
        HwcAxiGatherEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hwc_axi_max_outstanding_en(&self) -> HwcAxiMaxOutstandingEnR {
        HwcAxiMaxOutstandingEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - HWC DMA read Burst length"]
    #[inline(always)]
    pub fn hwc_dma_burst_length(&self) -> HwcDmaBurstLengthR {
        HwcDmaBurstLengthR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - hwc axi gather transfer number"]
    #[inline(always)]
    pub fn hwc_axi_gather_num(&self) -> HwcAxiGatherNumR {
        HwcAxiGatherNumR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:12 - hwc axi bus max outstanding number"]
    #[inline(always)]
    pub fn hwc_axi_max_outstanding_num(&self) -> HwcAxiMaxOutstandingNumR {
        HwcAxiMaxOutstandingNumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hwc_rgb2yuv_en(&self) -> HwcRgb2yuvEnR {
        HwcRgb2yuvEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - hwc AXI master read outstanding"]
    #[inline(always)]
    pub fn hwc_no_outstanding(&self) -> HwcNoOutstandingR {
        HwcNoOutstandingR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn hwc_y_mir_en(&self) -> HwcYMirEnR {
        HwcYMirEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hwc_lut_en(&self) -> HwcLutEnR {
        HwcLutEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - axi read id of hwc channel"]
    #[inline(always)]
    pub fn win_rid_hwc(&self) -> WinRidHwcR {
        WinRidHwcR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_axi_gather_en(&mut self) -> HwcAxiGatherEnW<HwcCtrl1Spec> {
        HwcAxiGatherEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_axi_max_outstanding_en(&mut self) -> HwcAxiMaxOutstandingEnW<HwcCtrl1Spec> {
        HwcAxiMaxOutstandingEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - HWC DMA read Burst length"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_dma_burst_length(&mut self) -> HwcDmaBurstLengthW<HwcCtrl1Spec> {
        HwcDmaBurstLengthW::new(self, 2)
    }
    #[doc = "Bits 4:6 - hwc axi gather transfer number"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_axi_gather_num(&mut self) -> HwcAxiGatherNumW<HwcCtrl1Spec> {
        HwcAxiGatherNumW::new(self, 4)
    }
    #[doc = "Bits 8:12 - hwc axi bus max outstanding number"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_axi_max_outstanding_num(&mut self) -> HwcAxiMaxOutstandingNumW<HwcCtrl1Spec> {
        HwcAxiMaxOutstandingNumW::new(self, 8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_rgb2yuv_en(&mut self) -> HwcRgb2yuvEnW<HwcCtrl1Spec> {
        HwcRgb2yuvEnW::new(self, 13)
    }
    #[doc = "Bit 14 - hwc AXI master read outstanding"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_no_outstanding(&mut self) -> HwcNoOutstandingW<HwcCtrl1Spec> {
        HwcNoOutstandingW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_y_mir_en(&mut self) -> HwcYMirEnW<HwcCtrl1Spec> {
        HwcYMirEnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_lut_en(&mut self) -> HwcLutEnW<HwcCtrl1Spec> {
        HwcLutEnW::new(self, 16)
    }
    #[doc = "Bits 20:23 - axi read id of hwc channel"]
    #[inline(always)]
    #[must_use]
    pub fn win_rid_hwc(&mut self) -> WinRidHwcW<HwcCtrl1Spec> {
        WinRidHwcW::new(self, 20)
    }
}
#[doc = "Hwc ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwcCtrl1Spec;
impl crate::RegisterSpec for HwcCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwc_ctrl1::R`](R) reader structure"]
impl crate::Readable for HwcCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`hwc_ctrl1::W`](W) writer structure"]
impl crate::Writable for HwcCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWC_CTRL1 to value 0x0070_1d00"]
impl crate::Resettable for HwcCtrl1Spec {
    const RESET_VALUE: u32 = 0x0070_1d00;
}

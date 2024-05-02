#[doc = "Register `DPF_MODE` reader"]
pub type R = crate::R<DpfModeSpec>;
#[doc = "Register `DPF_MODE` writer"]
pub type W = crate::W<DpfModeSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpfEnable {
    #[doc = "1: enable dpf"]
    B1 = 1,
    #[doc = "0: bypass dpf *Default*"]
    B0 = 0,
}
impl From<DpfEnable> for bool {
    #[inline(always)]
    fn from(variant: DpfEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPF_ENABLE` reader - "]
pub type DpfEnableR = crate::BitReader<DpfEnable>;
impl DpfEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpfEnable {
        match self.bits {
            true => DpfEnable::B1,
            false => DpfEnable::B0,
        }
    }
    #[doc = "enable dpf"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpfEnable::B1
    }
    #[doc = "bypass dpf *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpfEnable::B0
    }
}
#[doc = "Field `DPF_ENABLE` writer - "]
pub type DpfEnableW<'a, REG> = crate::BitWriter<'a, REG, DpfEnable>;
impl<'a, REG> DpfEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable dpf"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpfEnable::B1)
    }
    #[doc = "bypass dpf *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpfEnable::B0)
    }
}
#[doc = "Field `B_FILTER_OFF` reader - 1: disable filter processing for blue pixels (B) 0: filter B\n\npixels *Default*"]
pub type BFilterOffR = crate::BitReader;
#[doc = "Field `B_FILTER_OFF` writer - 1: disable filter processing for blue pixels (B) 0: filter B\n\npixels *Default*"]
pub type BFilterOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GbFilterOff {
    #[doc = "1: disable filter processing for green pixels in green/blue lines (GB)"]
    B1 = 1,
    #[doc = "0: filter GB pixels *Default*"]
    B0 = 0,
}
impl From<GbFilterOff> for bool {
    #[inline(always)]
    fn from(variant: GbFilterOff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GB_FILTER_OFF` reader - "]
pub type GbFilterOffR = crate::BitReader<GbFilterOff>;
impl GbFilterOffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GbFilterOff {
        match self.bits {
            true => GbFilterOff::B1,
            false => GbFilterOff::B0,
        }
    }
    #[doc = "disable filter processing for green pixels in green/blue lines (GB)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GbFilterOff::B1
    }
    #[doc = "filter GB pixels *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GbFilterOff::B0
    }
}
#[doc = "Field `GB_FILTER_OFF` writer - "]
pub type GbFilterOffW<'a, REG> = crate::BitWriter<'a, REG, GbFilterOff>;
impl<'a, REG> GbFilterOffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable filter processing for green pixels in green/blue lines (GB)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GbFilterOff::B1)
    }
    #[doc = "filter GB pixels *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GbFilterOff::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrFilterOff {
    #[doc = "1: disable filter processing for green pixels in green/red lines (GR)"]
    B1 = 1,
    #[doc = "0: filter GR pixels *Default*"]
    B0 = 0,
}
impl From<GrFilterOff> for bool {
    #[inline(always)]
    fn from(variant: GrFilterOff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GR_FILTER_OFF` reader - "]
pub type GrFilterOffR = crate::BitReader<GrFilterOff>;
impl GrFilterOffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GrFilterOff {
        match self.bits {
            true => GrFilterOff::B1,
            false => GrFilterOff::B0,
        }
    }
    #[doc = "disable filter processing for green pixels in green/red lines (GR)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GrFilterOff::B1
    }
    #[doc = "filter GR pixels *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GrFilterOff::B0
    }
}
#[doc = "Field `GR_FILTER_OFF` writer - "]
pub type GrFilterOffW<'a, REG> = crate::BitWriter<'a, REG, GrFilterOff>;
impl<'a, REG> GrFilterOffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable filter processing for green pixels in green/red lines (GR)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GrFilterOff::B1)
    }
    #[doc = "filter GR pixels *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GrFilterOff::B0)
    }
}
#[doc = "Field `R_FILTER_OFF` reader - 1: disable filter processing for red pixels (R) 0: filter R\n\npixels *Default*"]
pub type RFilterOffR = crate::BitReader;
#[doc = "Field `R_FILTER_OFF` writer - 1: disable filter processing for red pixels (R) 0: filter R\n\npixels *Default*"]
pub type RFilterOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_FILTER_SIZE` reader - 1: Red and Blue filter kernel size of 9x9 (5x5 active)\n\npixels 0: Wide Red and Blue filter kernel size of 13x9 (7x5\n\nactive) pixels *Default*"]
pub type RbFilterSizeR = crate::BitReader;
#[doc = "Field `RB_FILTER_SIZE` writer - 1: Red and Blue filter kernel size of 9x9 (5x5 active)\n\npixels 0: Wide Red and Blue filter kernel size of 13x9 (7x5\n\nactive) pixels *Default*"]
pub type RbFilterSizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NllSegmentation {
    #[doc = "1: optimized logarithmic like segmentation for Noise Level Lookup (NLL)"]
    B1 = 1,
    #[doc = "0: equidistant segmentation for NLL *Default*"]
    B0 = 0,
}
impl From<NllSegmentation> for bool {
    #[inline(always)]
    fn from(variant: NllSegmentation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NLL_SEGMENTATION` reader - "]
pub type NllSegmentationR = crate::BitReader<NllSegmentation>;
impl NllSegmentationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NllSegmentation {
        match self.bits {
            true => NllSegmentation::B1,
            false => NllSegmentation::B0,
        }
    }
    #[doc = "optimized logarithmic like segmentation for Noise Level Lookup (NLL)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NllSegmentation::B1
    }
    #[doc = "equidistant segmentation for NLL *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NllSegmentation::B0
    }
}
#[doc = "Field `NLL_SEGMENTATION` writer - "]
pub type NllSegmentationW<'a, REG> = crate::BitWriter<'a, REG, NllSegmentation>;
impl<'a, REG> NllSegmentationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "optimized logarithmic like segmentation for Noise Level Lookup (NLL)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(NllSegmentation::B1)
    }
    #[doc = "equidistant segmentation for NLL *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(NllSegmentation::B0)
    }
}
#[doc = "Only relevant when use_nf_gain == 0 &amp;&amp; ISP_CTRL::ISP_AWB_ENABLE ==1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AwbGainComp {
    #[doc = "1: ISP_AWB gains will be processed"]
    B1 = 1,
    #[doc = "0: ISP_AWB gains will not be processed. Use AWB gain factor of 1. *Default*"]
    B0 = 0,
}
impl From<AwbGainComp> for bool {
    #[inline(always)]
    fn from(variant: AwbGainComp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWB_GAIN_COMP` reader - Only relevant when use_nf_gain == 0 &amp;&amp; ISP_CTRL::ISP_AWB_ENABLE ==1"]
pub type AwbGainCompR = crate::BitReader<AwbGainComp>;
impl AwbGainCompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AwbGainComp {
        match self.bits {
            true => AwbGainComp::B1,
            false => AwbGainComp::B0,
        }
    }
    #[doc = "ISP_AWB gains will be processed"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AwbGainComp::B1
    }
    #[doc = "ISP_AWB gains will not be processed. Use AWB gain factor of 1. *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AwbGainComp::B0
    }
}
#[doc = "Field `AWB_GAIN_COMP` writer - Only relevant when use_nf_gain == 0 &amp;&amp; ISP_CTRL::ISP_AWB_ENABLE ==1"]
pub type AwbGainCompW<'a, REG> = crate::BitWriter<'a, REG, AwbGainComp>;
impl<'a, REG> AwbGainCompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ISP_AWB gains will be processed"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AwbGainComp::B1)
    }
    #[doc = "ISP_AWB gains will not be processed. Use AWB gain factor of 1. *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AwbGainComp::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LscGainComp {
    #[doc = "1: LSC gain will be processed"]
    B1 = 1,
    #[doc = "0: LSC gain will not be processed. Use LSC gain factor of 1. *Default*"]
    B0 = 0,
}
impl From<LscGainComp> for bool {
    #[inline(always)]
    fn from(variant: LscGainComp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSC_GAIN_COMP` reader - "]
pub type LscGainCompR = crate::BitReader<LscGainComp>;
impl LscGainCompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LscGainComp {
        match self.bits {
            true => LscGainComp::B1,
            false => LscGainComp::B0,
        }
    }
    #[doc = "LSC gain will be processed"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LscGainComp::B1
    }
    #[doc = "LSC gain will not be processed. Use LSC gain factor of 1. *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LscGainComp::B0
    }
}
#[doc = "Field `LSC_GAIN_COMP` writer - "]
pub type LscGainCompW<'a, REG> = crate::BitWriter<'a, REG, LscGainComp>;
impl<'a, REG> LscGainCompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSC gain will be processed"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LscGainComp::B1)
    }
    #[doc = "LSC gain will not be processed. Use LSC gain factor of 1. *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LscGainComp::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseNfGain {
    #[doc = "1: DPF_NF_GAINs will be used."]
    B1 = 1,
    #[doc = "0: DPF_NF_GAINs will not be used. *Default*"]
    B0 = 0,
}
impl From<UseNfGain> for bool {
    #[inline(always)]
    fn from(variant: UseNfGain) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE_NF_GAIN` reader - "]
pub type UseNfGainR = crate::BitReader<UseNfGain>;
impl UseNfGainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UseNfGain {
        match self.bits {
            true => UseNfGain::B1,
            false => UseNfGain::B0,
        }
    }
    #[doc = "DPF_NF_GAINs will be used."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UseNfGain::B1
    }
    #[doc = "DPF_NF_GAINs will not be used. *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UseNfGain::B0
    }
}
#[doc = "Field `USE_NF_GAIN` writer - "]
pub type UseNfGainW<'a, REG> = crate::BitWriter<'a, REG, UseNfGain>;
impl<'a, REG> UseNfGainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DPF_NF_GAINs will be used."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UseNfGain::B1)
    }
    #[doc = "DPF_NF_GAINs will not be used. *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UseNfGain::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dpf_enable(&self) -> DpfEnableR {
        DpfEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: disable filter processing for blue pixels (B) 0: filter B\n\npixels *Default*"]
    #[inline(always)]
    pub fn b_filter_off(&self) -> BFilterOffR {
        BFilterOffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gb_filter_off(&self) -> GbFilterOffR {
        GbFilterOffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gr_filter_off(&self) -> GrFilterOffR {
        GrFilterOffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: disable filter processing for red pixels (R) 0: filter R\n\npixels *Default*"]
    #[inline(always)]
    pub fn r_filter_off(&self) -> RFilterOffR {
        RFilterOffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Red and Blue filter kernel size of 9x9 (5x5 active)\n\npixels 0: Wide Red and Blue filter kernel size of 13x9 (7x5\n\nactive) pixels *Default*"]
    #[inline(always)]
    pub fn rb_filter_size(&self) -> RbFilterSizeR {
        RbFilterSizeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn nll_segmentation(&self) -> NllSegmentationR {
        NllSegmentationR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Only relevant when use_nf_gain == 0 &amp;&amp; ISP_CTRL::ISP_AWB_ENABLE ==1"]
    #[inline(always)]
    pub fn awb_gain_comp(&self) -> AwbGainCompR {
        AwbGainCompR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lsc_gain_comp(&self) -> LscGainCompR {
        LscGainCompR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn use_nf_gain(&self) -> UseNfGainR {
        UseNfGainR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dpf_enable(&mut self) -> DpfEnableW<DpfModeSpec> {
        DpfEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: disable filter processing for blue pixels (B) 0: filter B\n\npixels *Default*"]
    #[inline(always)]
    #[must_use]
    pub fn b_filter_off(&mut self) -> BFilterOffW<DpfModeSpec> {
        BFilterOffW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gb_filter_off(&mut self) -> GbFilterOffW<DpfModeSpec> {
        GbFilterOffW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gr_filter_off(&mut self) -> GrFilterOffW<DpfModeSpec> {
        GrFilterOffW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: disable filter processing for red pixels (R) 0: filter R\n\npixels *Default*"]
    #[inline(always)]
    #[must_use]
    pub fn r_filter_off(&mut self) -> RFilterOffW<DpfModeSpec> {
        RFilterOffW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Red and Blue filter kernel size of 9x9 (5x5 active)\n\npixels 0: Wide Red and Blue filter kernel size of 13x9 (7x5\n\nactive) pixels *Default*"]
    #[inline(always)]
    #[must_use]
    pub fn rb_filter_size(&mut self) -> RbFilterSizeW<DpfModeSpec> {
        RbFilterSizeW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn nll_segmentation(&mut self) -> NllSegmentationW<DpfModeSpec> {
        NllSegmentationW::new(self, 6)
    }
    #[doc = "Bit 7 - Only relevant when use_nf_gain == 0 &amp;&amp; ISP_CTRL::ISP_AWB_ENABLE ==1"]
    #[inline(always)]
    #[must_use]
    pub fn awb_gain_comp(&mut self) -> AwbGainCompW<DpfModeSpec> {
        AwbGainCompW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lsc_gain_comp(&mut self) -> LscGainCompW<DpfModeSpec> {
        LscGainCompW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn use_nf_gain(&mut self) -> UseNfGainW<DpfModeSpec> {
        UseNfGainW::new(self, 9)
    }
}
#[doc = "Mode control for Denoising Pre-Filter block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpf_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpf_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpfModeSpec;
impl crate::RegisterSpec for DpfModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpf_mode::R`](R) reader structure"]
impl crate::Readable for DpfModeSpec {}
#[doc = "`write(|w| ..)` method takes [`dpf_mode::W`](W) writer structure"]
impl crate::Writable for DpfModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPF_MODE to value 0"]
impl crate::Resettable for DpfModeSpec {
    const RESET_VALUE: u32 = 0;
}

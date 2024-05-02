#[doc = "Register `ACQ_PROP` reader"]
pub type R = crate::R<AcqPropSpec>;
#[doc = "Register `ACQ_PROP` writer"]
pub type W = crate::W<AcqPropSpec>;
#[doc = "Field `SAMPLE_EDGE` reader - 0- negative edge sampling\n\n1- positive edge sampling"]
pub type SampleEdgeR = crate::BitReader;
#[doc = "Field `SAMPLE_EDGE` writer - 0- negative edge sampling\n\n1- positive edge sampling"]
pub type SampleEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "horizontal sync polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsyncPol {
    #[doc = "0: high active"]
    B0 = 0,
    #[doc = "1: low active"]
    B1 = 1,
}
impl From<HsyncPol> for bool {
    #[inline(always)]
    fn from(variant: HsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSYNC_POL` reader - horizontal sync polarity"]
pub type HsyncPolR = crate::BitReader<HsyncPol>;
impl HsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsyncPol {
        match self.bits {
            false => HsyncPol::B0,
            true => HsyncPol::B1,
        }
    }
    #[doc = "high active"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsyncPol::B0
    }
    #[doc = "low active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsyncPol::B1
    }
}
#[doc = "Field `HSYNC_POL` writer - horizontal sync polarity"]
pub type HsyncPolW<'a, REG> = crate::BitWriter<'a, REG, HsyncPol>;
impl<'a, REG> HsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "high active"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsyncPol::B0)
    }
    #[doc = "low active"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsyncPol::B1)
    }
}
#[doc = "vertical sync polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VsyncPol {
    #[doc = "0: high active"]
    B0 = 0,
    #[doc = "1: low active"]
    B1 = 1,
}
impl From<VsyncPol> for bool {
    #[inline(always)]
    fn from(variant: VsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNC_POL` reader - vertical sync polarity"]
pub type VsyncPolR = crate::BitReader<VsyncPol>;
impl VsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VsyncPol {
        match self.bits {
            false => VsyncPol::B0,
            true => VsyncPol::B1,
        }
    }
    #[doc = "high active"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VsyncPol::B0
    }
    #[doc = "low active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VsyncPol::B1
    }
}
#[doc = "Field `VSYNC_POL` writer - vertical sync polarity"]
pub type VsyncPolW<'a, REG> = crate::BitWriter<'a, REG, VsyncPol>;
impl<'a, REG> VsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "high active"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VsyncPol::B0)
    }
    #[doc = "low active"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VsyncPol::B1)
    }
}
#[doc = "Field `BAYER_PAT` reader - color components from sensor, starting with top left\n\nposition in sampled frame (reprogram with ISP_ACQ_H_OFFS, ISP_ACQ_V_OFFS)\n\n00- first line: RGRG..., second line: GBGB..., etc.\n\n01- first line: GRGR..., second line: BGBG..., etc.\n\n10- first line: GBGB..., second line: RGRG..., etc.\n\n11- first line: BGBG..., second line: GRGR..., etc.\n\nThis configuration applies for the black level area after\n\ncropping by the input formatter."]
pub type BayerPatR = crate::FieldReader;
#[doc = "Field `BAYER_PAT` writer - color components from sensor, starting with top left\n\nposition in sampled frame (reprogram with ISP_ACQ_H_OFFS, ISP_ACQ_V_OFFS)\n\n00- first line: RGRG..., second line: GBGB..., etc.\n\n01- first line: GRGR..., second line: BGBG..., etc.\n\n10- first line: GBGB..., second line: RGRG..., etc.\n\n11- first line: BGBG..., second line: GRGR..., etc.\n\nThis configuration applies for the black level area after\n\ncropping by the input formatter."]
pub type BayerPatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONV_422` reader - co-sited color subsampling Y0Cb0Cr0 – Y1\n\ninterleaved color subsampling Y0Cb0 – Y1Cr1 (not\n\nrecommended)\n\n10- non-cosited color subsampling Y0Cb(0+1)/2 –\n\nY1Cr(0+1)/2 11- reserved"]
pub type Conv422R = crate::FieldReader;
#[doc = "Field `CONV_422` writer - co-sited color subsampling Y0Cb0Cr0 – Y1\n\ninterleaved color subsampling Y0Cb0 – Y1Cr1 (not\n\nrecommended)\n\n10- non-cosited color subsampling Y0Cb(0+1)/2 –\n\nY1Cr(0+1)/2 11- reserved"]
pub type Conv422W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CCIR_SEQ` reader - 00- YCbYCr\n\n01- YCrYCb\n\n10- CbYCrY\n\n11- CrYCbY"]
pub type CcirSeqR = crate::FieldReader;
#[doc = "Field `CCIR_SEQ` writer - 00- YCbYCr\n\n01- YCrYCb\n\n10- CbYCrY\n\n11- CrYCbY"]
pub type CcirSeqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD_SELECTION` reader - 00- sample all fields (don‟t care about fields)\n\n01- sample only even fields\n\n10- sample only odd fields\n\n11- reserved"]
pub type FieldSelectionR = crate::FieldReader;
#[doc = "Field `FIELD_SELECTION` writer - 00- sample all fields (don‟t care about fields)\n\n01- sample only even fields\n\n10- sample only odd fields\n\n11- reserved"]
pub type FieldSelectionW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldInv {
    #[doc = "1: swap odd and even fields"]
    B1 = 1,
    #[doc = "0: do not swap fields"]
    B0 = 0,
}
impl From<FieldInv> for bool {
    #[inline(always)]
    fn from(variant: FieldInv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELD_INV` reader - "]
pub type FieldInvR = crate::BitReader<FieldInv>;
impl FieldInvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FieldInv {
        match self.bits {
            true => FieldInv::B1,
            false => FieldInv::B0,
        }
    }
    #[doc = "swap odd and even fields"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FieldInv::B1
    }
    #[doc = "do not swap fields"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FieldInv::B0
    }
}
#[doc = "Field `FIELD_INV` writer - "]
pub type FieldInvW<'a, REG> = crate::BitWriter<'a, REG, FieldInv>;
impl<'a, REG> FieldInvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "swap odd and even fields"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FieldInv::B1)
    }
    #[doc = "do not swap fields"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FieldInv::B0)
    }
}
#[doc = "Field `INPUT_SELECTION` reader - 000 000- 12Bit external Interface\n\n001- 10Bit Interface, append 2 zeroes as LSBs\n\n010- 10Bit Interface, append 2 MSBs as LSBs\n\n011- 8Bit Interface, append 4 zeroes as LSBs\n\n100- 8Bit Interface, append 4 MSBs as LSBs\n\n101...111 reserved"]
pub type InputSelectionR = crate::FieldReader;
#[doc = "Field `INPUT_SELECTION` writer - 000 000- 12Bit external Interface\n\n001- 10Bit Interface, append 2 zeroes as LSBs\n\n010- 10Bit Interface, append 2 MSBs as LSBs\n\n011- 8Bit Interface, append 4 zeroes as LSBs\n\n100- 8Bit Interface, append 4 MSBs as LSBs\n\n101...111 reserved"]
pub type InputSelectionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRgbSelection {
    #[doc = "0: use input formatter data for latency fifo."]
    B0 = 0,
    #[doc = "1: use dma rgb read data for latency fifo."]
    B1 = 1,
}
impl From<DmaRgbSelection> for bool {
    #[inline(always)]
    fn from(variant: DmaRgbSelection) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RGB_SELECTION` reader - "]
pub type DmaRgbSelectionR = crate::BitReader<DmaRgbSelection>;
impl DmaRgbSelectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRgbSelection {
        match self.bits {
            false => DmaRgbSelection::B0,
            true => DmaRgbSelection::B1,
        }
    }
    #[doc = "use input formatter data for latency fifo."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DmaRgbSelection::B0
    }
    #[doc = "use dma rgb read data for latency fifo."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DmaRgbSelection::B1
    }
}
#[doc = "Field `DMA_RGB_SELECTION` writer - "]
pub type DmaRgbSelectionW<'a, REG> = crate::BitWriter<'a, REG, DmaRgbSelection>;
impl<'a, REG> DmaRgbSelectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use input formatter data for latency fifo."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRgbSelection::B0)
    }
    #[doc = "use dma rgb read data for latency fifo."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRgbSelection::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaYuvSelection {
    #[doc = "0: use align or conversion data for isp_is input."]
    B0 = 0,
    #[doc = "1: use dma yuv read data for isp_is input."]
    B1 = 1,
}
impl From<DmaYuvSelection> for bool {
    #[inline(always)]
    fn from(variant: DmaYuvSelection) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_YUV_SELECTION` reader - "]
pub type DmaYuvSelectionR = crate::BitReader<DmaYuvSelection>;
impl DmaYuvSelectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaYuvSelection {
        match self.bits {
            false => DmaYuvSelection::B0,
            true => DmaYuvSelection::B1,
        }
    }
    #[doc = "use align or conversion data for isp_is input."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DmaYuvSelection::B0
    }
    #[doc = "use dma yuv read data for isp_is input."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DmaYuvSelection::B1
    }
}
#[doc = "Field `DMA_YUV_SELECTION` writer - "]
pub type DmaYuvSelectionW<'a, REG> = crate::BitWriter<'a, REG, DmaYuvSelection>;
impl<'a, REG> DmaYuvSelectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use align or conversion data for isp_is input."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DmaYuvSelection::B0)
    }
    #[doc = "use dma yuv read data for isp_is input."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DmaYuvSelection::B1)
    }
}
impl R {
    #[doc = "Bit 0 - 0- negative edge sampling\n\n1- positive edge sampling"]
    #[inline(always)]
    pub fn sample_edge(&self) -> SampleEdgeR {
        SampleEdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - horizontal sync polarity"]
    #[inline(always)]
    pub fn hsync_pol(&self) -> HsyncPolR {
        HsyncPolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - vertical sync polarity"]
    #[inline(always)]
    pub fn vsync_pol(&self) -> VsyncPolR {
        VsyncPolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - color components from sensor, starting with top left\n\nposition in sampled frame (reprogram with ISP_ACQ_H_OFFS, ISP_ACQ_V_OFFS)\n\n00- first line: RGRG..., second line: GBGB..., etc.\n\n01- first line: GRGR..., second line: BGBG..., etc.\n\n10- first line: GBGB..., second line: RGRG..., etc.\n\n11- first line: BGBG..., second line: GRGR..., etc.\n\nThis configuration applies for the black level area after\n\ncropping by the input formatter."]
    #[inline(always)]
    pub fn bayer_pat(&self) -> BayerPatR {
        BayerPatR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - co-sited color subsampling Y0Cb0Cr0 – Y1\n\ninterleaved color subsampling Y0Cb0 – Y1Cr1 (not\n\nrecommended)\n\n10- non-cosited color subsampling Y0Cb(0+1)/2 –\n\nY1Cr(0+1)/2 11- reserved"]
    #[inline(always)]
    pub fn conv_422(&self) -> Conv422R {
        Conv422R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - 00- YCbYCr\n\n01- YCrYCb\n\n10- CbYCrY\n\n11- CrYCbY"]
    #[inline(always)]
    pub fn ccir_seq(&self) -> CcirSeqR {
        CcirSeqR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - 00- sample all fields (don‟t care about fields)\n\n01- sample only even fields\n\n10- sample only odd fields\n\n11- reserved"]
    #[inline(always)]
    pub fn field_selection(&self) -> FieldSelectionR {
        FieldSelectionR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn field_inv(&self) -> FieldInvR {
        FieldInvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 000 000- 12Bit external Interface\n\n001- 10Bit Interface, append 2 zeroes as LSBs\n\n010- 10Bit Interface, append 2 MSBs as LSBs\n\n011- 8Bit Interface, append 4 zeroes as LSBs\n\n100- 8Bit Interface, append 4 MSBs as LSBs\n\n101...111 reserved"]
    #[inline(always)]
    pub fn input_selection(&self) -> InputSelectionR {
        InputSelectionR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dma_rgb_selection(&self) -> DmaRgbSelectionR {
        DmaRgbSelectionR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_yuv_selection(&self) -> DmaYuvSelectionR {
        DmaYuvSelectionR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0- negative edge sampling\n\n1- positive edge sampling"]
    #[inline(always)]
    #[must_use]
    pub fn sample_edge(&mut self) -> SampleEdgeW<AcqPropSpec> {
        SampleEdgeW::new(self, 0)
    }
    #[doc = "Bit 1 - horizontal sync polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hsync_pol(&mut self) -> HsyncPolW<AcqPropSpec> {
        HsyncPolW::new(self, 1)
    }
    #[doc = "Bit 2 - vertical sync polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_pol(&mut self) -> VsyncPolW<AcqPropSpec> {
        VsyncPolW::new(self, 2)
    }
    #[doc = "Bits 3:4 - color components from sensor, starting with top left\n\nposition in sampled frame (reprogram with ISP_ACQ_H_OFFS, ISP_ACQ_V_OFFS)\n\n00- first line: RGRG..., second line: GBGB..., etc.\n\n01- first line: GRGR..., second line: BGBG..., etc.\n\n10- first line: GBGB..., second line: RGRG..., etc.\n\n11- first line: BGBG..., second line: GRGR..., etc.\n\nThis configuration applies for the black level area after\n\ncropping by the input formatter."]
    #[inline(always)]
    #[must_use]
    pub fn bayer_pat(&mut self) -> BayerPatW<AcqPropSpec> {
        BayerPatW::new(self, 3)
    }
    #[doc = "Bits 5:6 - co-sited color subsampling Y0Cb0Cr0 – Y1\n\ninterleaved color subsampling Y0Cb0 – Y1Cr1 (not\n\nrecommended)\n\n10- non-cosited color subsampling Y0Cb(0+1)/2 –\n\nY1Cr(0+1)/2 11- reserved"]
    #[inline(always)]
    #[must_use]
    pub fn conv_422(&mut self) -> Conv422W<AcqPropSpec> {
        Conv422W::new(self, 5)
    }
    #[doc = "Bits 7:8 - 00- YCbYCr\n\n01- YCrYCb\n\n10- CbYCrY\n\n11- CrYCbY"]
    #[inline(always)]
    #[must_use]
    pub fn ccir_seq(&mut self) -> CcirSeqW<AcqPropSpec> {
        CcirSeqW::new(self, 7)
    }
    #[doc = "Bits 9:10 - 00- sample all fields (don‟t care about fields)\n\n01- sample only even fields\n\n10- sample only odd fields\n\n11- reserved"]
    #[inline(always)]
    #[must_use]
    pub fn field_selection(&mut self) -> FieldSelectionW<AcqPropSpec> {
        FieldSelectionW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn field_inv(&mut self) -> FieldInvW<AcqPropSpec> {
        FieldInvW::new(self, 11)
    }
    #[doc = "Bits 12:14 - 000 000- 12Bit external Interface\n\n001- 10Bit Interface, append 2 zeroes as LSBs\n\n010- 10Bit Interface, append 2 MSBs as LSBs\n\n011- 8Bit Interface, append 4 zeroes as LSBs\n\n100- 8Bit Interface, append 4 MSBs as LSBs\n\n101...111 reserved"]
    #[inline(always)]
    #[must_use]
    pub fn input_selection(&mut self) -> InputSelectionW<AcqPropSpec> {
        InputSelectionW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rgb_selection(&mut self) -> DmaRgbSelectionW<AcqPropSpec> {
        DmaRgbSelectionW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dma_yuv_selection(&mut self) -> DmaYuvSelectionW<AcqPropSpec> {
        DmaYuvSelectionW::new(self, 16)
    }
}
#[doc = "ISP acquisition properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_prop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_prop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcqPropSpec;
impl crate::RegisterSpec for AcqPropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acq_prop::R`](R) reader structure"]
impl crate::Readable for AcqPropSpec {}
#[doc = "`write(|w| ..)` method takes [`acq_prop::W`](W) writer structure"]
impl crate::Writable for AcqPropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACQ_PROP to value 0"]
impl crate::Resettable for AcqPropSpec {
    const RESET_VALUE: u32 = 0;
}

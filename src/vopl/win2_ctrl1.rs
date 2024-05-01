#[doc = "Register `WIN2_CTRL1` reader"]
pub type R = crate::R<Win2Ctrl1Spec>;
#[doc = "Register `WIN2_CTRL1` writer"]
pub type W = crate::W<Win2Ctrl1Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2AxiGatherEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2AxiGatherEn> for bool {
    #[inline(always)]
    fn from(variant: Win2AxiGatherEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_AXI_GATHER_EN` reader - "]
pub type Win2AxiGatherEnR = crate::BitReader<Win2AxiGatherEn>;
impl Win2AxiGatherEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2AxiGatherEn {
        match self.bits {
            false => Win2AxiGatherEn::B0,
            true => Win2AxiGatherEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2AxiGatherEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2AxiGatherEn::B1
    }
}
#[doc = "Field `WIN2_AXI_GATHER_EN` writer - "]
pub type Win2AxiGatherEnW<'a, REG> = crate::BitWriter<'a, REG, Win2AxiGatherEn>;
impl<'a, REG> Win2AxiGatherEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AxiGatherEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AxiGatherEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2AxiMaxOutstandingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2AxiMaxOutstandingEn> for bool {
    #[inline(always)]
    fn from(variant: Win2AxiMaxOutstandingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_AXI_MAX_OUTSTANDING_EN` reader - "]
pub type Win2AxiMaxOutstandingEnR = crate::BitReader<Win2AxiMaxOutstandingEn>;
impl Win2AxiMaxOutstandingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2AxiMaxOutstandingEn {
        match self.bits {
            false => Win2AxiMaxOutstandingEn::B0,
            true => Win2AxiMaxOutstandingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2AxiMaxOutstandingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2AxiMaxOutstandingEn::B1
    }
}
#[doc = "Field `WIN2_AXI_MAX_OUTSTANDING_EN` writer - "]
pub type Win2AxiMaxOutstandingEnW<'a, REG> = crate::BitWriter<'a, REG, Win2AxiMaxOutstandingEn>;
impl<'a, REG> Win2AxiMaxOutstandingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AxiMaxOutstandingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AxiMaxOutstandingEn::B1)
    }
}
#[doc = "WIN2 DMA read Burst length\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win2DmaBurstLength {
    #[doc = "0: burst16 (burst 15 in rgb888 pack mode)"]
    B00 = 0,
    #[doc = "1: burst8 (burst 12 in rgb888 pack mode)"]
    B01 = 1,
    #[doc = "2: burst4 (burst 6 in rgb888 pack mode)"]
    B10 = 2,
}
impl From<Win2DmaBurstLength> for u8 {
    #[inline(always)]
    fn from(variant: Win2DmaBurstLength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win2DmaBurstLength {
    type Ux = u8;
}
#[doc = "Field `WIN2_DMA_BURST_LENGTH` reader - WIN2 DMA read Burst length"]
pub type Win2DmaBurstLengthR = crate::FieldReader<Win2DmaBurstLength>;
impl Win2DmaBurstLengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win2DmaBurstLength> {
        match self.bits {
            0 => Some(Win2DmaBurstLength::B00),
            1 => Some(Win2DmaBurstLength::B01),
            2 => Some(Win2DmaBurstLength::B10),
            _ => None,
        }
    }
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win2DmaBurstLength::B00
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win2DmaBurstLength::B01
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win2DmaBurstLength::B10
    }
}
#[doc = "Field `WIN2_DMA_BURST_LENGTH` writer - WIN2 DMA read Burst length"]
pub type Win2DmaBurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win2DmaBurstLength>;
impl<'a, REG> Win2DmaBurstLengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DmaBurstLength::B00)
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DmaBurstLength::B01)
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DmaBurstLength::B10)
    }
}
#[doc = "Field `WIN2_AXI_GATHER_NUM` reader - win2 axi gather transfer number"]
pub type Win2AxiGatherNumR = crate::FieldReader;
#[doc = "Field `WIN2_AXI_GATHER_NUM` writer - win2 axi gather transfer number"]
pub type Win2AxiGatherNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WIN2_AXI_MAX_OUTSTANDING_NUM` reader - win2 axi max outstanding number"]
pub type Win2AxiMaxOutstandingNumR = crate::FieldReader;
#[doc = "Field `WIN2_AXI_MAX_OUTSTANDING_NUM` writer - win2 axi max outstanding number"]
pub type Win2AxiMaxOutstandingNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Win2 AXI master read outstanding\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2NoOutstanding {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: disable"]
    B1 = 1,
}
impl From<Win2NoOutstanding> for bool {
    #[inline(always)]
    fn from(variant: Win2NoOutstanding) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_NO_OUTSTANDING` reader - Win2 AXI master read outstanding"]
pub type Win2NoOutstandingR = crate::BitReader<Win2NoOutstanding>;
impl Win2NoOutstandingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2NoOutstanding {
        match self.bits {
            false => Win2NoOutstanding::B0,
            true => Win2NoOutstanding::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2NoOutstanding::B0
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2NoOutstanding::B1
    }
}
#[doc = "Field `WIN2_NO_OUTSTANDING` writer - Win2 AXI master read outstanding"]
pub type Win2NoOutstandingW<'a, REG> = crate::BitWriter<'a, REG, Win2NoOutstanding>;
impl<'a, REG> Win2NoOutstandingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2NoOutstanding::B0)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2NoOutstanding::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2YMirEn {
    #[doc = "0: no y_mirror"]
    B0 = 0,
    #[doc = "1: y_mirror"]
    B1 = 1,
}
impl From<Win2YMirEn> for bool {
    #[inline(always)]
    fn from(variant: Win2YMirEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_Y_MIR_EN` reader - "]
pub type Win2YMirEnR = crate::BitReader<Win2YMirEn>;
impl Win2YMirEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2YMirEn {
        match self.bits {
            false => Win2YMirEn::B0,
            true => Win2YMirEn::B1,
        }
    }
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2YMirEn::B0
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2YMirEn::B1
    }
}
#[doc = "Field `WIN2_Y_MIR_EN` writer - "]
pub type Win2YMirEnW<'a, REG> = crate::BitWriter<'a, REG, Win2YMirEn>;
impl<'a, REG> Win2YMirEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2YMirEn::B0)
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2YMirEn::B1)
    }
}
#[doc = "Win2 LUT ram enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2LutEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable *This bit should be '0' when CPU updates the LUT, and should be '1' when Win2 LUT mode enable."]
    B1 = 1,
}
impl From<Win2LutEn> for bool {
    #[inline(always)]
    fn from(variant: Win2LutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_LUT_EN` reader - Win2 LUT ram enable"]
pub type Win2LutEnR = crate::BitReader<Win2LutEn>;
impl Win2LutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2LutEn {
        match self.bits {
            false => Win2LutEn::B0,
            true => Win2LutEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2LutEn::B0
    }
    #[doc = "enable *This bit should be '0' when CPU updates the LUT, and should be '1' when Win2 LUT mode enable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2LutEn::B1
    }
}
#[doc = "Field `WIN2_LUT_EN` writer - Win2 LUT ram enable"]
pub type Win2LutEnW<'a, REG> = crate::BitWriter<'a, REG, Win2LutEn>;
impl<'a, REG> Win2LutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2LutEn::B0)
    }
    #[doc = "enable *This bit should be '0' when CPU updates the LUT, and should be '1' when Win2 LUT mode enable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2LutEn::B1)
    }
}
#[doc = "Field `WIN_RID_WIN2` reader - axi read id of win2 channel"]
pub type WinRidWin2R = crate::FieldReader;
#[doc = "Field `WIN_RID_WIN2` writer - axi read id of win2 channel"]
pub type WinRidWin2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn win2_axi_gather_en(&self) -> Win2AxiGatherEnR {
        Win2AxiGatherEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn win2_axi_max_outstanding_en(&self) -> Win2AxiMaxOutstandingEnR {
        Win2AxiMaxOutstandingEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - WIN2 DMA read Burst length"]
    #[inline(always)]
    pub fn win2_dma_burst_length(&self) -> Win2DmaBurstLengthR {
        Win2DmaBurstLengthR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - win2 axi gather transfer number"]
    #[inline(always)]
    pub fn win2_axi_gather_num(&self) -> Win2AxiGatherNumR {
        Win2AxiGatherNumR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - win2 axi max outstanding number"]
    #[inline(always)]
    pub fn win2_axi_max_outstanding_num(&self) -> Win2AxiMaxOutstandingNumR {
        Win2AxiMaxOutstandingNumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Win2 AXI master read outstanding"]
    #[inline(always)]
    pub fn win2_no_outstanding(&self) -> Win2NoOutstandingR {
        Win2NoOutstandingR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn win2_y_mir_en(&self) -> Win2YMirEnR {
        Win2YMirEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Win2 LUT ram enable"]
    #[inline(always)]
    pub fn win2_lut_en(&self) -> Win2LutEnR {
        Win2LutEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - axi read id of win2 channel"]
    #[inline(always)]
    pub fn win_rid_win2(&self) -> WinRidWin2R {
        WinRidWin2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn win2_axi_gather_en(&mut self) -> Win2AxiGatherEnW<Win2Ctrl1Spec> {
        Win2AxiGatherEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn win2_axi_max_outstanding_en(&mut self) -> Win2AxiMaxOutstandingEnW<Win2Ctrl1Spec> {
        Win2AxiMaxOutstandingEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - WIN2 DMA read Burst length"]
    #[inline(always)]
    #[must_use]
    pub fn win2_dma_burst_length(&mut self) -> Win2DmaBurstLengthW<Win2Ctrl1Spec> {
        Win2DmaBurstLengthW::new(self, 2)
    }
    #[doc = "Bits 4:7 - win2 axi gather transfer number"]
    #[inline(always)]
    #[must_use]
    pub fn win2_axi_gather_num(&mut self) -> Win2AxiGatherNumW<Win2Ctrl1Spec> {
        Win2AxiGatherNumW::new(self, 4)
    }
    #[doc = "Bits 8:12 - win2 axi max outstanding number"]
    #[inline(always)]
    #[must_use]
    pub fn win2_axi_max_outstanding_num(&mut self) -> Win2AxiMaxOutstandingNumW<Win2Ctrl1Spec> {
        Win2AxiMaxOutstandingNumW::new(self, 8)
    }
    #[doc = "Bit 14 - Win2 AXI master read outstanding"]
    #[inline(always)]
    #[must_use]
    pub fn win2_no_outstanding(&mut self) -> Win2NoOutstandingW<Win2Ctrl1Spec> {
        Win2NoOutstandingW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn win2_y_mir_en(&mut self) -> Win2YMirEnW<Win2Ctrl1Spec> {
        Win2YMirEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Win2 LUT ram enable"]
    #[inline(always)]
    #[must_use]
    pub fn win2_lut_en(&mut self) -> Win2LutEnW<Win2Ctrl1Spec> {
        Win2LutEnW::new(self, 16)
    }
    #[doc = "Bits 20:23 - axi read id of win2 channel"]
    #[inline(always)]
    #[must_use]
    pub fn win_rid_win2(&mut self) -> WinRidWin2W<Win2Ctrl1Spec> {
        WinRidWin2W::new(self, 20)
    }
}
#[doc = "win2 ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2Ctrl1Spec;
impl crate::RegisterSpec for Win2Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_ctrl1::R`](R) reader structure"]
impl crate::Readable for Win2Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_ctrl1::W`](W) writer structure"]
impl crate::Writable for Win2Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_CTRL1 to value 0x0050_1d00"]
impl crate::Resettable for Win2Ctrl1Spec {
    const RESET_VALUE: u32 = 0x0050_1d00;
}

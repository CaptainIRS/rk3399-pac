#[doc = "Register `WIN3_CTRL1` reader"]
pub type R = crate::R<Win3Ctrl1Spec>;
#[doc = "Register `WIN3_CTRL1` writer"]
pub type W = crate::W<Win3Ctrl1Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3AxiGatherEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3AxiGatherEn> for bool {
    #[inline(always)]
    fn from(variant: Win3AxiGatherEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_AXI_GATHER_EN` reader - "]
pub type Win3AxiGatherEnR = crate::BitReader<Win3AxiGatherEn>;
impl Win3AxiGatherEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3AxiGatherEn {
        match self.bits {
            false => Win3AxiGatherEn::B0,
            true => Win3AxiGatherEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3AxiGatherEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3AxiGatherEn::B1
    }
}
#[doc = "Field `WIN3_AXI_GATHER_EN` writer - "]
pub type Win3AxiGatherEnW<'a, REG> = crate::BitWriter<'a, REG, Win3AxiGatherEn>;
impl<'a, REG> Win3AxiGatherEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AxiGatherEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AxiGatherEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3AxiMaxOutstandingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3AxiMaxOutstandingEn> for bool {
    #[inline(always)]
    fn from(variant: Win3AxiMaxOutstandingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_AXI_MAX_OUTSTANDING_EN` reader - "]
pub type Win3AxiMaxOutstandingEnR = crate::BitReader<Win3AxiMaxOutstandingEn>;
impl Win3AxiMaxOutstandingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3AxiMaxOutstandingEn {
        match self.bits {
            false => Win3AxiMaxOutstandingEn::B0,
            true => Win3AxiMaxOutstandingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3AxiMaxOutstandingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3AxiMaxOutstandingEn::B1
    }
}
#[doc = "Field `WIN3_AXI_MAX_OUTSTANDING_EN` writer - "]
pub type Win3AxiMaxOutstandingEnW<'a, REG> = crate::BitWriter<'a, REG, Win3AxiMaxOutstandingEn>;
impl<'a, REG> Win3AxiMaxOutstandingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AxiMaxOutstandingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AxiMaxOutstandingEn::B1)
    }
}
#[doc = "WIN3 DMA read Burst length\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win3DmaBurstLength {
    #[doc = "0: burst16 (burst 15 in rgb888 pack mode)"]
    B00 = 0,
    #[doc = "1: burst8 (burst 12 in rgb888 pack mode)"]
    B01 = 1,
    #[doc = "2: burst4 (burst 6 in rgb888 pack mode)"]
    B10 = 2,
}
impl From<Win3DmaBurstLength> for u8 {
    #[inline(always)]
    fn from(variant: Win3DmaBurstLength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win3DmaBurstLength {
    type Ux = u8;
}
#[doc = "Field `WIN3_DMA_BURST_LENGTH` reader - WIN3 DMA read Burst length"]
pub type Win3DmaBurstLengthR = crate::FieldReader<Win3DmaBurstLength>;
impl Win3DmaBurstLengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win3DmaBurstLength> {
        match self.bits {
            0 => Some(Win3DmaBurstLength::B00),
            1 => Some(Win3DmaBurstLength::B01),
            2 => Some(Win3DmaBurstLength::B10),
            _ => None,
        }
    }
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win3DmaBurstLength::B00
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win3DmaBurstLength::B01
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win3DmaBurstLength::B10
    }
}
#[doc = "Field `WIN3_DMA_BURST_LENGTH` writer - WIN3 DMA read Burst length"]
pub type Win3DmaBurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win3DmaBurstLength>;
impl<'a, REG> Win3DmaBurstLengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DmaBurstLength::B00)
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DmaBurstLength::B01)
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DmaBurstLength::B10)
    }
}
#[doc = "Field `WIN3_AXI_GATHER_NUM` reader - win3 axi gather transfer number"]
pub type Win3AxiGatherNumR = crate::FieldReader;
#[doc = "Field `WIN3_AXI_GATHER_NUM` writer - win3 axi gather transfer number"]
pub type Win3AxiGatherNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WIN3_AXI_MAX_OUTSTANDING_NUM` reader - win3 axi bus max outstanding number"]
pub type Win3AxiMaxOutstandingNumR = crate::FieldReader;
#[doc = "Field `WIN3_AXI_MAX_OUTSTANDING_NUM` writer - win3 axi bus max outstanding number"]
pub type Win3AxiMaxOutstandingNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Win3 AXI master read outstanding\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3NoOutstanding {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: disable"]
    B1 = 1,
}
impl From<Win3NoOutstanding> for bool {
    #[inline(always)]
    fn from(variant: Win3NoOutstanding) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_NO_OUTSTANDING` reader - Win3 AXI master read outstanding"]
pub type Win3NoOutstandingR = crate::BitReader<Win3NoOutstanding>;
impl Win3NoOutstandingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3NoOutstanding {
        match self.bits {
            false => Win3NoOutstanding::B0,
            true => Win3NoOutstanding::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3NoOutstanding::B0
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3NoOutstanding::B1
    }
}
#[doc = "Field `WIN3_NO_OUTSTANDING` writer - Win3 AXI master read outstanding"]
pub type Win3NoOutstandingW<'a, REG> = crate::BitWriter<'a, REG, Win3NoOutstanding>;
impl<'a, REG> Win3NoOutstandingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3NoOutstanding::B0)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3NoOutstanding::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3YMirEn {
    #[doc = "0: no y_mirror"]
    B0 = 0,
    #[doc = "1: y_mirror"]
    B1 = 1,
}
impl From<Win3YMirEn> for bool {
    #[inline(always)]
    fn from(variant: Win3YMirEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_Y_MIR_EN` reader - "]
pub type Win3YMirEnR = crate::BitReader<Win3YMirEn>;
impl Win3YMirEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3YMirEn {
        match self.bits {
            false => Win3YMirEn::B0,
            true => Win3YMirEn::B1,
        }
    }
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3YMirEn::B0
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3YMirEn::B1
    }
}
#[doc = "Field `WIN3_Y_MIR_EN` writer - "]
pub type Win3YMirEnW<'a, REG> = crate::BitWriter<'a, REG, Win3YMirEn>;
impl<'a, REG> Win3YMirEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3YMirEn::B0)
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3YMirEn::B1)
    }
}
#[doc = "Win3 LUT ram enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3LutEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable *This bit should be '0' when CPU updates the LUT, and should be '1' when Win1 LUT mode enable."]
    B1 = 1,
}
impl From<Win3LutEn> for bool {
    #[inline(always)]
    fn from(variant: Win3LutEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_LUT_EN` reader - Win3 LUT ram enable"]
pub type Win3LutEnR = crate::BitReader<Win3LutEn>;
impl Win3LutEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3LutEn {
        match self.bits {
            false => Win3LutEn::B0,
            true => Win3LutEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3LutEn::B0
    }
    #[doc = "enable *This bit should be '0' when CPU updates the LUT, and should be '1' when Win1 LUT mode enable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3LutEn::B1
    }
}
#[doc = "Field `WIN3_LUT_EN` writer - Win3 LUT ram enable"]
pub type Win3LutEnW<'a, REG> = crate::BitWriter<'a, REG, Win3LutEn>;
impl<'a, REG> Win3LutEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3LutEn::B0)
    }
    #[doc = "enable *This bit should be '0' when CPU updates the LUT, and should be '1' when Win1 LUT mode enable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3LutEn::B1)
    }
}
#[doc = "Field `WIN_RID_WIN3` reader - axi read id of win3 channel"]
pub type WinRidWin3R = crate::FieldReader;
#[doc = "Field `WIN_RID_WIN3` writer - axi read id of win3 channel"]
pub type WinRidWin3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn win3_axi_gather_en(&self) -> Win3AxiGatherEnR {
        Win3AxiGatherEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn win3_axi_max_outstanding_en(&self) -> Win3AxiMaxOutstandingEnR {
        Win3AxiMaxOutstandingEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - WIN3 DMA read Burst length"]
    #[inline(always)]
    pub fn win3_dma_burst_length(&self) -> Win3DmaBurstLengthR {
        Win3DmaBurstLengthR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - win3 axi gather transfer number"]
    #[inline(always)]
    pub fn win3_axi_gather_num(&self) -> Win3AxiGatherNumR {
        Win3AxiGatherNumR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - win3 axi bus max outstanding number"]
    #[inline(always)]
    pub fn win3_axi_max_outstanding_num(&self) -> Win3AxiMaxOutstandingNumR {
        Win3AxiMaxOutstandingNumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Win3 AXI master read outstanding"]
    #[inline(always)]
    pub fn win3_no_outstanding(&self) -> Win3NoOutstandingR {
        Win3NoOutstandingR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn win3_y_mir_en(&self) -> Win3YMirEnR {
        Win3YMirEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Win3 LUT ram enable"]
    #[inline(always)]
    pub fn win3_lut_en(&self) -> Win3LutEnR {
        Win3LutEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - axi read id of win3 channel"]
    #[inline(always)]
    pub fn win_rid_win3(&self) -> WinRidWin3R {
        WinRidWin3R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn win3_axi_gather_en(&mut self) -> Win3AxiGatherEnW<Win3Ctrl1Spec> {
        Win3AxiGatherEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn win3_axi_max_outstanding_en(&mut self) -> Win3AxiMaxOutstandingEnW<Win3Ctrl1Spec> {
        Win3AxiMaxOutstandingEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - WIN3 DMA read Burst length"]
    #[inline(always)]
    #[must_use]
    pub fn win3_dma_burst_length(&mut self) -> Win3DmaBurstLengthW<Win3Ctrl1Spec> {
        Win3DmaBurstLengthW::new(self, 2)
    }
    #[doc = "Bits 4:7 - win3 axi gather transfer number"]
    #[inline(always)]
    #[must_use]
    pub fn win3_axi_gather_num(&mut self) -> Win3AxiGatherNumW<Win3Ctrl1Spec> {
        Win3AxiGatherNumW::new(self, 4)
    }
    #[doc = "Bits 8:12 - win3 axi bus max outstanding number"]
    #[inline(always)]
    #[must_use]
    pub fn win3_axi_max_outstanding_num(&mut self) -> Win3AxiMaxOutstandingNumW<Win3Ctrl1Spec> {
        Win3AxiMaxOutstandingNumW::new(self, 8)
    }
    #[doc = "Bit 14 - Win3 AXI master read outstanding"]
    #[inline(always)]
    #[must_use]
    pub fn win3_no_outstanding(&mut self) -> Win3NoOutstandingW<Win3Ctrl1Spec> {
        Win3NoOutstandingW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn win3_y_mir_en(&mut self) -> Win3YMirEnW<Win3Ctrl1Spec> {
        Win3YMirEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Win3 LUT ram enable"]
    #[inline(always)]
    #[must_use]
    pub fn win3_lut_en(&mut self) -> Win3LutEnW<Win3Ctrl1Spec> {
        Win3LutEnW::new(self, 16)
    }
    #[doc = "Bits 20:23 - axi read id of win3 channel"]
    #[inline(always)]
    #[must_use]
    pub fn win_rid_win3(&mut self) -> WinRidWin3W<Win3Ctrl1Spec> {
        WinRidWin3W::new(self, 20)
    }
}
#[doc = "Win3 ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3Ctrl1Spec;
impl crate::RegisterSpec for Win3Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_ctrl1::R`](R) reader structure"]
impl crate::Readable for Win3Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_ctrl1::W`](W) writer structure"]
impl crate::Writable for Win3Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_CTRL1 to value 0x0060_1d00"]
impl crate::Resettable for Win3Ctrl1Spec {
    const RESET_VALUE: u32 = 0x0060_1d00;
}

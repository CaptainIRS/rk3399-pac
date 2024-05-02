#[doc = "Register `FLASH_CONFIG` reader"]
pub type R = crate::R<FlashConfigSpec>;
#[doc = "Register `FLASH_CONFIG` writer"]
pub type W = crate::W<FlashConfigSpec>;
#[doc = "prelight mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrelightMode {
    #[doc = "0: prelight is switched off at begin of flash"]
    B0 = 0,
    #[doc = "1: prelight is switched off at end of flash"]
    B1 = 1,
}
impl From<PrelightMode> for bool {
    #[inline(always)]
    fn from(variant: PrelightMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `prelight_mode` reader - prelight mode"]
pub type PrelightModeR = crate::BitReader<PrelightMode>;
impl PrelightModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrelightMode {
        match self.bits {
            false => PrelightMode::B0,
            true => PrelightMode::B1,
        }
    }
    #[doc = "prelight is switched off at begin of flash"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PrelightMode::B0
    }
    #[doc = "prelight is switched off at end of flash"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PrelightMode::B1
    }
}
#[doc = "Field `prelight_mode` writer - prelight mode"]
pub type PrelightModeW<'a, REG> = crate::BitWriter<'a, REG, PrelightMode>;
impl<'a, REG> PrelightModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "prelight is switched off at begin of flash"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PrelightMode::B0)
    }
    #[doc = "prelight is switched off at end of flash"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PrelightMode::B1)
    }
}
#[doc = "VSYNC edge\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VsInEdge {
    #[doc = "0: use negative edge of “vds_vsync” if generating a trigger event"]
    B0 = 0,
    #[doc = "1: use positive edge of “vds_vsync” if generating a trigger event"]
    B1 = 1,
}
impl From<VsInEdge> for bool {
    #[inline(always)]
    fn from(variant: VsInEdge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vs_in_edge` reader - VSYNC edge"]
pub type VsInEdgeR = crate::BitReader<VsInEdge>;
impl VsInEdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VsInEdge {
        match self.bits {
            false => VsInEdge::B0,
            true => VsInEdge::B1,
        }
    }
    #[doc = "use negative edge of “vds_vsync” if generating a trigger event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VsInEdge::B0
    }
    #[doc = "use positive edge of “vds_vsync” if generating a trigger event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VsInEdge::B1
    }
}
#[doc = "Field `vs_in_edge` writer - VSYNC edge"]
pub type VsInEdgeW<'a, REG> = crate::BitWriter<'a, REG, VsInEdge>;
impl<'a, REG> VsInEdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use negative edge of “vds_vsync” if generating a trigger event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VsInEdge::B0)
    }
    #[doc = "use positive edge of “vds_vsync” if generating a trigger event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VsInEdge::B1)
    }
}
#[doc = "polarity of flash related signals\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlPol {
    #[doc = "0: flash_trig, prelight_trig are high active"]
    B0 = 0,
    #[doc = "1: flash_trig, prelight_trig are low active"]
    B1 = 1,
}
impl From<FlPol> for bool {
    #[inline(always)]
    fn from(variant: FlPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `fl_pol` reader - polarity of flash related signals"]
pub type FlPolR = crate::BitReader<FlPol>;
impl FlPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlPol {
        match self.bits {
            false => FlPol::B0,
            true => FlPol::B1,
        }
    }
    #[doc = "flash_trig, prelight_trig are high active"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FlPol::B0
    }
    #[doc = "flash_trig, prelight_trig are low active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FlPol::B1
    }
}
#[doc = "Field `fl_pol` writer - polarity of flash related signals"]
pub type FlPolW<'a, REG> = crate::BitWriter<'a, REG, FlPol>;
impl<'a, REG> FlPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "flash_trig, prelight_trig are high active"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FlPol::B0)
    }
    #[doc = "flash_trig, prelight_trig are low active"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FlPol::B1)
    }
}
#[doc = "trigger source for flash and prelight\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlTrigSrc {
    #[doc = "0: use “vds_vsync” for trigger event (with evaluation of vs_in_edge)"]
    B0 = 0,
    #[doc = "1: use “fl_trig” for trigger event (positive edge)"]
    B1 = 1,
}
impl From<FlTrigSrc> for bool {
    #[inline(always)]
    fn from(variant: FlTrigSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `fl_trig_src` reader - trigger source for flash and prelight"]
pub type FlTrigSrcR = crate::BitReader<FlTrigSrc>;
impl FlTrigSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlTrigSrc {
        match self.bits {
            false => FlTrigSrc::B0,
            true => FlTrigSrc::B1,
        }
    }
    #[doc = "use “vds_vsync” for trigger event (with evaluation of vs_in_edge)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FlTrigSrc::B0
    }
    #[doc = "use “fl_trig” for trigger event (positive edge)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FlTrigSrc::B1
    }
}
#[doc = "Field `fl_trig_src` writer - trigger source for flash and prelight"]
pub type FlTrigSrcW<'a, REG> = crate::BitWriter<'a, REG, FlTrigSrc>;
impl<'a, REG> FlTrigSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use “vds_vsync” for trigger event (with evaluation of vs_in_edge)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FlTrigSrc::B0)
    }
    #[doc = "use “fl_trig” for trigger event (positive edge)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FlTrigSrc::B1)
    }
}
#[doc = "Field `fl_cap_del` reader - capture delay\n\nframe number (0 to 15) to be captured after trigger\n\nevent"]
pub type FlCapDelR = crate::FieldReader;
#[doc = "Field `fl_cap_del` writer - capture delay\n\nframe number (0 to 15) to be captured after trigger\n\nevent"]
pub type FlCapDelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - prelight mode"]
    #[inline(always)]
    pub fn prelight_mode(&self) -> PrelightModeR {
        PrelightModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VSYNC edge"]
    #[inline(always)]
    pub fn vs_in_edge(&self) -> VsInEdgeR {
        VsInEdgeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - polarity of flash related signals"]
    #[inline(always)]
    pub fn fl_pol(&self) -> FlPolR {
        FlPolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - trigger source for flash and prelight"]
    #[inline(always)]
    pub fn fl_trig_src(&self) -> FlTrigSrcR {
        FlTrigSrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - capture delay\n\nframe number (0 to 15) to be captured after trigger\n\nevent"]
    #[inline(always)]
    pub fn fl_cap_del(&self) -> FlCapDelR {
        FlCapDelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - prelight mode"]
    #[inline(always)]
    #[must_use]
    pub fn prelight_mode(&mut self) -> PrelightModeW<FlashConfigSpec> {
        PrelightModeW::new(self, 0)
    }
    #[doc = "Bit 1 - VSYNC edge"]
    #[inline(always)]
    #[must_use]
    pub fn vs_in_edge(&mut self) -> VsInEdgeW<FlashConfigSpec> {
        VsInEdgeW::new(self, 1)
    }
    #[doc = "Bit 2 - polarity of flash related signals"]
    #[inline(always)]
    #[must_use]
    pub fn fl_pol(&mut self) -> FlPolW<FlashConfigSpec> {
        FlPolW::new(self, 2)
    }
    #[doc = "Bit 3 - trigger source for flash and prelight"]
    #[inline(always)]
    #[must_use]
    pub fn fl_trig_src(&mut self) -> FlTrigSrcW<FlashConfigSpec> {
        FlTrigSrcW::new(self, 3)
    }
    #[doc = "Bits 4:7 - capture delay\n\nframe number (0 to 15) to be captured after trigger\n\nevent"]
    #[inline(always)]
    #[must_use]
    pub fn fl_cap_del(&mut self) -> FlCapDelW<FlashConfigSpec> {
        FlCapDelW::new(self, 4)
    }
}
#[doc = "Flash config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashConfigSpec;
impl crate::RegisterSpec for FlashConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_config::R`](R) reader structure"]
impl crate::Readable for FlashConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_config::W`](W) writer structure"]
impl crate::Writable for FlashConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_CONFIG to value 0"]
impl crate::Resettable for FlashConfigSpec {
    const RESET_VALUE: u32 = 0;
}

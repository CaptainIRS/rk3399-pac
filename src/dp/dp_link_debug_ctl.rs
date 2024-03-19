#[doc = "Register `DP_LINK_DEBUG_CTL` reader"]
pub type R = crate::R<DpLinkDebugCtlSpec>;
#[doc = "Register `DP_LINK_DEBUG_CTL` writer"]
pub type W = crate::W<DpLinkDebugCtlSpec>;
#[doc = "Enable DisplayPort PRBS 31.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prbs31En {
    #[doc = "1: Enabled,"]
    B1 = 1,
    #[doc = "0: Normal mode."]
    B0 = 0,
}
impl From<Prbs31En> for bool {
    #[inline(always)]
    fn from(variant: Prbs31En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRBS31_EN` reader - Enable DisplayPort PRBS 31."]
pub type Prbs31EnR = crate::BitReader<Prbs31En>;
impl Prbs31EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prbs31En {
        match self.bits {
            true => Prbs31En::B1,
            false => Prbs31En::B0,
        }
    }
    #[doc = "Enabled,"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Prbs31En::B1
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Prbs31En::B0
    }
}
#[doc = "Field `PRBS31_EN` writer - Enable DisplayPort PRBS 31."]
pub type Prbs31EnW<'a, REG> = crate::BitWriter<'a, REG, Prbs31En>;
impl<'a, REG> Prbs31EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled,"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Prbs31En::B1)
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Prbs31En::B0)
    }
}
#[doc = "Disable 8b/10 encoder auto reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableAutoResetEncoder {
    #[doc = "1: Disabled auto reset 8b/10 encode before sending Link Training Pattern 2"]
    B1 = 1,
    #[doc = "0: Auto reset 8b/10 encode before sending Link Training Pattern 2"]
    B0 = 0,
}
impl From<DisableAutoResetEncoder> for bool {
    #[inline(always)]
    fn from(variant: DisableAutoResetEncoder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLE_AUTO_RESET_ENCODER` reader - Disable 8b/10 encoder auto reset"]
pub type DisableAutoResetEncoderR = crate::BitReader<DisableAutoResetEncoder>;
impl DisableAutoResetEncoderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisableAutoResetEncoder {
        match self.bits {
            true => DisableAutoResetEncoder::B1,
            false => DisableAutoResetEncoder::B0,
        }
    }
    #[doc = "Disabled auto reset 8b/10 encode before sending Link Training Pattern 2"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DisableAutoResetEncoder::B1
    }
    #[doc = "Auto reset 8b/10 encode before sending Link Training Pattern 2"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DisableAutoResetEncoder::B0
    }
}
#[doc = "Field `DISABLE_AUTO_RESET_ENCODER` writer - Disable 8b/10 encoder auto reset"]
pub type DisableAutoResetEncoderW<'a, REG> = crate::BitWriter1C<'a, REG, DisableAutoResetEncoder>;
impl<'a, REG> DisableAutoResetEncoderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled auto reset 8b/10 encode before sending Link Training Pattern 2"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DisableAutoResetEncoder::B1)
    }
    #[doc = "Auto reset 8b/10 encode before sending Link Training Pattern 2"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DisableAutoResetEncoder::B0)
    }
}
#[doc = "Disable video FIFO reset every line\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisFifoRst {
    #[doc = "1: Disable,"]
    B1 = 1,
    #[doc = "0: Reset video FIFO every line."]
    B0 = 0,
}
impl From<DisFifoRst> for bool {
    #[inline(always)]
    fn from(variant: DisFifoRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_FIFO_RST` reader - Disable video FIFO reset every line"]
pub type DisFifoRstR = crate::BitReader<DisFifoRst>;
impl DisFifoRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisFifoRst {
        match self.bits {
            true => DisFifoRst::B1,
            false => DisFifoRst::B0,
        }
    }
    #[doc = "Disable,"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DisFifoRst::B1
    }
    #[doc = "Reset video FIFO every line."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DisFifoRst::B0
    }
}
#[doc = "Field `DIS_FIFO_RST` writer - Disable video FIFO reset every line"]
pub type DisFifoRstW<'a, REG> = crate::BitWriter<'a, REG, DisFifoRst>;
impl<'a, REG> DisFifoRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable,"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DisFifoRst::B1)
    }
    #[doc = "Reset video FIFO every line."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DisFifoRst::B0)
    }
}
#[doc = "Control the PRBS 7 formula.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NewPrbs7 {
    #[doc = "1: Use new PRBS7 formula in DP 1.1 version"]
    B1 = 1,
    #[doc = "0: Use old PRBS7 formula in DP 1.0 version"]
    B0 = 0,
}
impl From<NewPrbs7> for bool {
    #[inline(always)]
    fn from(variant: NewPrbs7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEW_PRBS7` reader - Control the PRBS 7 formula."]
pub type NewPrbs7R = crate::BitReader<NewPrbs7>;
impl NewPrbs7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NewPrbs7 {
        match self.bits {
            true => NewPrbs7::B1,
            false => NewPrbs7::B0,
        }
    }
    #[doc = "Use new PRBS7 formula in DP 1.1 version"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NewPrbs7::B1
    }
    #[doc = "Use old PRBS7 formula in DP 1.0 version"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NewPrbs7::B0
    }
}
#[doc = "Field `NEW_PRBS7` writer - Control the PRBS 7 formula."]
pub type NewPrbs7W<'a, REG> = crate::BitWriter<'a, REG, NewPrbs7>;
impl<'a, REG> NewPrbs7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use new PRBS7 formula in DP 1.1 version"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(NewPrbs7::B1)
    }
    #[doc = "Use old PRBS7 formula in DP 1.0 version"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(NewPrbs7::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Enable DisplayPort PRBS 31."]
    #[inline(always)]
    pub fn prbs31_en(&self) -> Prbs31EnR {
        Prbs31EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Disable 8b/10 encoder auto reset"]
    #[inline(always)]
    pub fn disable_auto_reset_encoder(&self) -> DisableAutoResetEncoderR {
        DisableAutoResetEncoderR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable video FIFO reset every line"]
    #[inline(always)]
    pub fn dis_fifo_rst(&self) -> DisFifoRstR {
        DisFifoRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control the PRBS 7 formula."]
    #[inline(always)]
    pub fn new_prbs7(&self) -> NewPrbs7R {
        NewPrbs7R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DisplayPort PRBS 31."]
    #[inline(always)]
    #[must_use]
    pub fn prbs31_en(&mut self) -> Prbs31EnW<DpLinkDebugCtlSpec> {
        Prbs31EnW::new(self, 0)
    }
    #[doc = "Bit 2 - Disable 8b/10 encoder auto reset"]
    #[inline(always)]
    #[must_use]
    pub fn disable_auto_reset_encoder(&mut self) -> DisableAutoResetEncoderW<DpLinkDebugCtlSpec> {
        DisableAutoResetEncoderW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable video FIFO reset every line"]
    #[inline(always)]
    #[must_use]
    pub fn dis_fifo_rst(&mut self) -> DisFifoRstW<DpLinkDebugCtlSpec> {
        DisFifoRstW::new(self, 3)
    }
    #[doc = "Bit 4 - Control the PRBS 7 formula."]
    #[inline(always)]
    #[must_use]
    pub fn new_prbs7(&mut self) -> NewPrbs7W<DpLinkDebugCtlSpec> {
        NewPrbs7W::new(self, 4)
    }
}
#[doc = "DP Link Debug Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_link_debug_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_link_debug_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpLinkDebugCtlSpec;
impl crate::RegisterSpec for DpLinkDebugCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_link_debug_ctl::R`](R) reader structure"]
impl crate::Readable for DpLinkDebugCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_link_debug_ctl::W`](W) writer structure"]
impl crate::Writable for DpLinkDebugCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x04;
}
#[doc = "`reset()` method sets DP_LINK_DEBUG_CTL to value 0x10"]
impl crate::Resettable for DpLinkDebugCtlSpec {
    const RESET_VALUE: u32 = 0x10;
}

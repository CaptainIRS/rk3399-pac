#[doc = "Register `SWREG_54` reader"]
pub type R = crate::R<Swreg54Spec>;
#[doc = "Register `SWREG_54` writer"]
pub type W = crate::W<Swreg54Spec>;
#[doc = "disable burst mode for AXI\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BurstDisable {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: disable"]
    B1 = 1,
}
impl From<BurstDisable> for bool {
    #[inline(always)]
    fn from(variant: BurstDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST_DISABLE` reader - disable burst mode for AXI"]
pub type BurstDisableR = crate::BitReader<BurstDisable>;
impl BurstDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BurstDisable {
        match self.bits {
            false => BurstDisable::B0,
            true => BurstDisable::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BurstDisable::B0
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BurstDisable::B1
    }
}
#[doc = "Field `BURST_DISABLE` writer - disable burst mode for AXI"]
pub type BurstDisableW<'a, REG> = crate::BitWriter<'a, REG, BurstDisable>;
impl<'a, REG> BurstDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BurstDisable::B0)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BurstDisable::B1)
    }
}
#[doc = "on-off burst data dicard\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BurstDiscard {
    #[doc = "0: disable ,off"]
    B0 = 0,
    #[doc = "1: enable,on"]
    B1 = 1,
}
impl From<BurstDiscard> for bool {
    #[inline(always)]
    fn from(variant: BurstDiscard) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST_DISCARD` reader - on-off burst data dicard"]
pub type BurstDiscardR = crate::BitReader<BurstDiscard>;
impl BurstDiscardR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BurstDiscard {
        match self.bits {
            false => BurstDiscard::B0,
            true => BurstDiscard::B1,
        }
    }
    #[doc = "disable ,off"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BurstDiscard::B0
    }
    #[doc = "enable,on"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BurstDiscard::B1
    }
}
#[doc = "Field `BURST_DISCARD` writer - on-off burst data dicard"]
pub type BurstDiscardW<'a, REG> = crate::BitWriter<'a, REG, BurstDiscard>;
impl<'a, REG> BurstDiscardW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable ,off"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BurstDiscard::B0)
    }
    #[doc = "enable,on"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BurstDiscard::B1)
    }
}
#[doc = "burst increment mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BurstIncrModSel {
    #[doc = "0: single burst selected"]
    B0 = 0,
    #[doc = "1: incr burst selected"]
    B1 = 1,
}
impl From<BurstIncrModSel> for bool {
    #[inline(always)]
    fn from(variant: BurstIncrModSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST_INCR_MOD_SEL` reader - burst increment mode select"]
pub type BurstIncrModSelR = crate::BitReader<BurstIncrModSel>;
impl BurstIncrModSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BurstIncrModSel {
        match self.bits {
            false => BurstIncrModSel::B0,
            true => BurstIncrModSel::B1,
        }
    }
    #[doc = "single burst selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BurstIncrModSel::B0
    }
    #[doc = "incr burst selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BurstIncrModSel::B1
    }
}
#[doc = "Field `BURST_INCR_MOD_SEL` writer - burst increment mode select"]
pub type BurstIncrModSelW<'a, REG> = crate::BitWriter<'a, REG, BurstIncrModSel>;
impl<'a, REG> BurstIncrModSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "single burst selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BurstIncrModSel::B0)
    }
    #[doc = "incr burst selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BurstIncrModSel::B1)
    }
}
#[doc = "Field `BURST_LEN` reader - burst length\n\nburst length"]
pub type BurstLenR = crate::FieldReader;
#[doc = "Field `BURST_LEN` writer - burst length\n\nburst length"]
pub type BurstLenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AXI_WR_ID` reader - axi write id\n\nif config 0,it will be modify as 1 by HW auto"]
pub type AxiWrIdR = crate::FieldReader;
#[doc = "Field `AXI_WR_ID` writer - axi write id\n\nif config 0,it will be modify as 1 by HW auto"]
pub type AxiWrIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AXI_RD_ID` reader - axi read id\n\nif config 0,it will be modify as 1 by HW auto"]
pub type AxiRdIdR = crate::FieldReader;
#[doc = "Field `AXI_RD_ID` writer - axi read id\n\nif config 0,it will be modify as 1 by HW auto"]
pub type AxiRdIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - disable burst mode for AXI"]
    #[inline(always)]
    pub fn burst_disable(&self) -> BurstDisableR {
        BurstDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - on-off burst data dicard"]
    #[inline(always)]
    pub fn burst_discard(&self) -> BurstDiscardR {
        BurstDiscardR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - burst increment mode select"]
    #[inline(always)]
    pub fn burst_incr_mod_sel(&self) -> BurstIncrModSelR {
        BurstIncrModSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:13 - burst length\n\nburst length"]
    #[inline(always)]
    pub fn burst_len(&self) -> BurstLenR {
        BurstLenR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - axi write id\n\nif config 0,it will be modify as 1 by HW auto"]
    #[inline(always)]
    pub fn axi_wr_id(&self) -> AxiWrIdR {
        AxiWrIdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - axi read id\n\nif config 0,it will be modify as 1 by HW auto"]
    #[inline(always)]
    pub fn axi_rd_id(&self) -> AxiRdIdR {
        AxiRdIdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - disable burst mode for AXI"]
    #[inline(always)]
    #[must_use]
    pub fn burst_disable(&mut self) -> BurstDisableW<Swreg54Spec> {
        BurstDisableW::new(self, 0)
    }
    #[doc = "Bit 1 - on-off burst data dicard"]
    #[inline(always)]
    #[must_use]
    pub fn burst_discard(&mut self) -> BurstDiscardW<Swreg54Spec> {
        BurstDiscardW::new(self, 1)
    }
    #[doc = "Bit 2 - burst increment mode select"]
    #[inline(always)]
    #[must_use]
    pub fn burst_incr_mod_sel(&mut self) -> BurstIncrModSelW<Swreg54Spec> {
        BurstIncrModSelW::new(self, 2)
    }
    #[doc = "Bits 8:13 - burst length\n\nburst length"]
    #[inline(always)]
    #[must_use]
    pub fn burst_len(&mut self) -> BurstLenW<Swreg54Spec> {
        BurstLenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - axi write id\n\nif config 0,it will be modify as 1 by HW auto"]
    #[inline(always)]
    #[must_use]
    pub fn axi_wr_id(&mut self) -> AxiWrIdW<Swreg54Spec> {
        AxiWrIdW::new(self, 16)
    }
    #[doc = "Bits 24:31 - axi read id\n\nif config 0,it will be modify as 1 by HW auto"]
    #[inline(always)]
    #[must_use]
    pub fn axi_rd_id(&mut self) -> AxiRdIdW<Swreg54Spec> {
        AxiRdIdW::new(self, 24)
    }
}
#[doc = "axi control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_54::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_54::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg54Spec;
impl crate::RegisterSpec for Swreg54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_54::R`](R) reader structure"]
impl crate::Readable for Swreg54Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_54::W`](W) writer structure"]
impl crate::Writable for Swreg54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_54 to value 0x0101_0000"]
impl crate::Resettable for Swreg54Spec {
    const RESET_VALUE: u32 = 0x0101_0000;
}

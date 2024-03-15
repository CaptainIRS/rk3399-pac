#[doc = "Register `DP_LINK_STATUS1` reader"]
pub type R = crate::R<DpLinkStatus1Spec>;
#[doc = "Register `DP_LINK_STATUS1` writer"]
pub type W = crate::W<DpLinkStatus1Spec>;
#[doc = "Field `LN2_CR_DONE` reader - Lane2 CR done"]
pub type Ln2CrDoneR = crate::BitReader;
#[doc = "Field `LN2_CR_DONE` writer - Lane2 CR done"]
pub type Ln2CrDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LN2_EQ_DONE` reader - Lane2 EQ done"]
pub type Ln2EqDoneR = crate::BitReader;
#[doc = "Field `LN2_EQ_DONE` writer - Lane2 EQ done"]
pub type Ln2EqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN2_SYMBOL_LOCK` reader - Lane2 symbol lock"]
pub type Ln2SymbolLockR = crate::BitReader;
#[doc = "Field `LN2_SYMBOL_LOCK` writer - Lane2 symbol lock"]
pub type Ln2SymbolLockW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LN3_CR_DONE` reader - Lane3 CR done"]
pub type Ln3CrDoneR = crate::BitReader;
#[doc = "Field `LN3_CR_DONE` writer - Lane3 CR done"]
pub type Ln3CrDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LN3_EQ_DONE` reader - Lane3 EQ done"]
pub type Ln3EqDoneR = crate::BitReader;
#[doc = "Field `LN3_EQ_DONE` writer - Lane3 EQ done"]
pub type Ln3EqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN3_SYMBOL_LOCK` reader - Lane3 symbol lock"]
pub type Ln3SymbolLockR = crate::BitReader;
#[doc = "Field `LN3_SYMBOL_LOCK` writer - Lane3 symbol lock"]
pub type Ln3SymbolLockW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INTER_LN_ALIGN` reader - Interlace align"]
pub type InterLnAlignR = crate::BitReader;
#[doc = "Field `INTER_LN_ALIGN` writer - Interlace align"]
pub type InterLnAlignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lane2 CR done"]
    #[inline(always)]
    pub fn ln2_cr_done(&self) -> Ln2CrDoneR {
        Ln2CrDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lane2 EQ done"]
    #[inline(always)]
    pub fn ln2_eq_done(&self) -> Ln2EqDoneR {
        Ln2EqDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lane2 symbol lock"]
    #[inline(always)]
    pub fn ln2_symbol_lock(&self) -> Ln2SymbolLockR {
        Ln2SymbolLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Lane3 CR done"]
    #[inline(always)]
    pub fn ln3_cr_done(&self) -> Ln3CrDoneR {
        Ln3CrDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lane3 EQ done"]
    #[inline(always)]
    pub fn ln3_eq_done(&self) -> Ln3EqDoneR {
        Ln3EqDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lane3 symbol lock"]
    #[inline(always)]
    pub fn ln3_symbol_lock(&self) -> Ln3SymbolLockR {
        Ln3SymbolLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interlace align"]
    #[inline(always)]
    pub fn inter_ln_align(&self) -> InterLnAlignR {
        InterLnAlignR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lane2 CR done"]
    #[inline(always)]
    #[must_use]
    pub fn ln2_cr_done(&mut self) -> Ln2CrDoneW<DpLinkStatus1Spec> {
        Ln2CrDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Lane2 EQ done"]
    #[inline(always)]
    #[must_use]
    pub fn ln2_eq_done(&mut self) -> Ln2EqDoneW<DpLinkStatus1Spec> {
        Ln2EqDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Lane2 symbol lock"]
    #[inline(always)]
    #[must_use]
    pub fn ln2_symbol_lock(&mut self) -> Ln2SymbolLockW<DpLinkStatus1Spec> {
        Ln2SymbolLockW::new(self, 2)
    }
    #[doc = "Bit 4 - Lane3 CR done"]
    #[inline(always)]
    #[must_use]
    pub fn ln3_cr_done(&mut self) -> Ln3CrDoneW<DpLinkStatus1Spec> {
        Ln3CrDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Lane3 EQ done"]
    #[inline(always)]
    #[must_use]
    pub fn ln3_eq_done(&mut self) -> Ln3EqDoneW<DpLinkStatus1Spec> {
        Ln3EqDoneW::new(self, 5)
    }
    #[doc = "Bit 6 - Lane3 symbol lock"]
    #[inline(always)]
    #[must_use]
    pub fn ln3_symbol_lock(&mut self) -> Ln3SymbolLockW<DpLinkStatus1Spec> {
        Ln3SymbolLockW::new(self, 6)
    }
    #[doc = "Bit 7 - Interlace align"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ln_align(&mut self) -> InterLnAlignW<DpLinkStatus1Spec> {
        InterLnAlignW::new(self, 7)
    }
}
#[doc = "DP Lane2 and Lane3 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_link_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_link_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpLinkStatus1Spec;
impl crate::RegisterSpec for DpLinkStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_link_status1::R`](R) reader structure"]
impl crate::Readable for DpLinkStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`dp_link_status1::W`](W) writer structure"]
impl crate::Writable for DpLinkStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x55;
}
#[doc = "`reset()` method sets DP_LINK_STATUS1 to value 0"]
impl crate::Resettable for DpLinkStatus1Spec {
    const RESET_VALUE: u32 = 0;
}

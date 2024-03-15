#[doc = "Register `DP_LINK_STATUS0` reader"]
pub type R = crate::R<DpLinkStatus0Spec>;
#[doc = "Register `DP_LINK_STATUS0` writer"]
pub type W = crate::W<DpLinkStatus0Spec>;
#[doc = "Field `LN0_CR_DONE` reader - Lane0 CR done"]
pub type Ln0CrDoneR = crate::BitReader;
#[doc = "Field `LN0_CR_DONE` writer - Lane0 CR done"]
pub type Ln0CrDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LN0_EQ_DONE` reader - Lane0 EQ done"]
pub type Ln0EqDoneR = crate::BitReader;
#[doc = "Field `LN0_EQ_DONE` writer - Lane0 EQ done"]
pub type Ln0EqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN0_SYBOL_LOCK` reader - Lane0 symbol lock"]
pub type Ln0SybolLockR = crate::BitReader;
#[doc = "Field `LN0_SYBOL_LOCK` writer - Lane0 symbol lock"]
pub type Ln0SybolLockW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LN_CR_DONE` reader - Lane1 CR done"]
pub type LnCrDoneR = crate::BitReader;
#[doc = "Field `LN_CR_DONE` writer - Lane1 CR done"]
pub type LnCrDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LN_EQ_DONE` reader - Lane1 EQ done"]
pub type LnEqDoneR = crate::BitReader;
#[doc = "Field `LN_EQ_DONE` writer - Lane1 EQ done"]
pub type LnEqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN1_SYBOL_LOCK` reader - Lane1 symbol lock"]
pub type Ln1SybolLockR = crate::BitReader;
#[doc = "Field `LN1_SYBOL_LOCK` writer - Lane1 symbol lock"]
pub type Ln1SybolLockW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lane0 CR done"]
    #[inline(always)]
    pub fn ln0_cr_done(&self) -> Ln0CrDoneR {
        Ln0CrDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lane0 EQ done"]
    #[inline(always)]
    pub fn ln0_eq_done(&self) -> Ln0EqDoneR {
        Ln0EqDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lane0 symbol lock"]
    #[inline(always)]
    pub fn ln0_sybol_lock(&self) -> Ln0SybolLockR {
        Ln0SybolLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Lane1 CR done"]
    #[inline(always)]
    pub fn ln_cr_done(&self) -> LnCrDoneR {
        LnCrDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lane1 EQ done"]
    #[inline(always)]
    pub fn ln_eq_done(&self) -> LnEqDoneR {
        LnEqDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lane1 symbol lock"]
    #[inline(always)]
    pub fn ln1_sybol_lock(&self) -> Ln1SybolLockR {
        Ln1SybolLockR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lane0 CR done"]
    #[inline(always)]
    #[must_use]
    pub fn ln0_cr_done(&mut self) -> Ln0CrDoneW<DpLinkStatus0Spec> {
        Ln0CrDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Lane0 EQ done"]
    #[inline(always)]
    #[must_use]
    pub fn ln0_eq_done(&mut self) -> Ln0EqDoneW<DpLinkStatus0Spec> {
        Ln0EqDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Lane0 symbol lock"]
    #[inline(always)]
    #[must_use]
    pub fn ln0_sybol_lock(&mut self) -> Ln0SybolLockW<DpLinkStatus0Spec> {
        Ln0SybolLockW::new(self, 2)
    }
    #[doc = "Bit 4 - Lane1 CR done"]
    #[inline(always)]
    #[must_use]
    pub fn ln_cr_done(&mut self) -> LnCrDoneW<DpLinkStatus0Spec> {
        LnCrDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Lane1 EQ done"]
    #[inline(always)]
    #[must_use]
    pub fn ln_eq_done(&mut self) -> LnEqDoneW<DpLinkStatus0Spec> {
        LnEqDoneW::new(self, 5)
    }
    #[doc = "Bit 6 - Lane1 symbol lock"]
    #[inline(always)]
    #[must_use]
    pub fn ln1_sybol_lock(&mut self) -> Ln1SybolLockW<DpLinkStatus0Spec> {
        Ln1SybolLockW::new(self, 6)
    }
}
#[doc = "DP Lane0 and Lane1 Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_link_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_link_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpLinkStatus0Spec;
impl crate::RegisterSpec for DpLinkStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_link_status0::R`](R) reader structure"]
impl crate::Readable for DpLinkStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`dp_link_status0::W`](W) writer structure"]
impl crate::Writable for DpLinkStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x55;
}
#[doc = "`reset()` method sets DP_LINK_STATUS0 to value 0"]
impl crate::Resettable for DpLinkStatus0Spec {
    const RESET_VALUE: u32 = 0;
}

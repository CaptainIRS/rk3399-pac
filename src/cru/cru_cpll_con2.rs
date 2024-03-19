#[doc = "Register `CRU_CPLL_CON2` reader"]
pub type R = crate::R<CruCpllCon2Spec>;
#[doc = "Register `CRU_CPLL_CON2` writer"]
pub type W = crate::W<CruCpllCon2Spec>;
#[doc = "Field `FRACDIV` reader - Fractional part of feedback divide\n\n(fraction = FRAC/2^24)"]
pub type FracdivR = crate::FieldReader<u32>;
#[doc = "Field `FRACDIV` writer - Fractional part of feedback divide\n\n(fraction = FRAC/2^24)"]
pub type FracdivW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "PLL lock status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllLock {
    #[doc = "0: unlock"]
    B0 = 0,
    #[doc = "1: lock"]
    B1 = 1,
}
impl From<PllLock> for bool {
    #[inline(always)]
    fn from(variant: PllLock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK` reader - PLL lock status"]
pub type PllLockR = crate::BitReader<PllLock>;
impl PllLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PllLock {
        match self.bits {
            false => PllLock::B0,
            true => PllLock::B1,
        }
    }
    #[doc = "unlock"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PllLock::B0
    }
    #[doc = "lock"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PllLock::B1
    }
}
impl R {
    #[doc = "Bits 0:23 - Fractional part of feedback divide\n\n(fraction = FRAC/2^24)"]
    #[inline(always)]
    pub fn fracdiv(&self) -> FracdivR {
        FracdivR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - PLL lock status"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PllLockR {
        PllLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Fractional part of feedback divide\n\n(fraction = FRAC/2^24)"]
    #[inline(always)]
    #[must_use]
    pub fn fracdiv(&mut self) -> FracdivW<CruCpllCon2Spec> {
        FracdivW::new(self, 0)
    }
}
#[doc = "CPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_cpll_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_cpll_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruCpllCon2Spec;
impl crate::RegisterSpec for CruCpllCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_cpll_con2::R`](R) reader structure"]
impl crate::Readable for CruCpllCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_cpll_con2::W`](W) writer structure"]
impl crate::Writable for CruCpllCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CPLL_CON2 to value 0x031f"]
impl crate::Resettable for CruCpllCon2Spec {
    const RESET_VALUE: u32 = 0x031f;
}

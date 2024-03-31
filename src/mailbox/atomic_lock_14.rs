#[doc = "Register `ATOMIC_LOCK_14` reader"]
pub type R = crate::R<AtomicLock14Spec>;
#[doc = "Register `ATOMIC_LOCK_14` writer"]
pub type W = crate::W<AtomicLock14Spec>;
#[doc = "Field `ATOMIC_LOCK_14` reader - lock flag bit 14"]
pub type AtomicLock14R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_14` writer - lock flag bit 14"]
pub type AtomicLock14W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 14"]
    #[inline(always)]
    pub fn atomic_lock_14(&self) -> AtomicLock14R {
        AtomicLock14R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_14(&mut self) -> AtomicLock14W<AtomicLock14Spec> {
        AtomicLock14W::new(self, 0)
    }
}
#[doc = "Lock flag register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock14Spec;
impl crate::RegisterSpec for AtomicLock14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_14::R`](R) reader structure"]
impl crate::Readable for AtomicLock14Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_14::W`](W) writer structure"]
impl crate::Writable for AtomicLock14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_14 to value 0"]
impl crate::Resettable for AtomicLock14Spec {
    const RESET_VALUE: u32 = 0;
}

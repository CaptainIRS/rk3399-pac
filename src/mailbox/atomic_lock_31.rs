#[doc = "Register `ATOMIC_LOCK_31` reader"]
pub type R = crate::R<AtomicLock31Spec>;
#[doc = "Register `ATOMIC_LOCK_31` writer"]
pub type W = crate::W<AtomicLock31Spec>;
#[doc = "Field `ATOMIC_LOCK_31` reader - lock flag bit 31"]
pub type AtomicLock31R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_31` writer - lock flag bit 31"]
pub type AtomicLock31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 31"]
    #[inline(always)]
    pub fn atomic_lock_31(&self) -> AtomicLock31R {
        AtomicLock31R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_31(&mut self) -> AtomicLock31W<AtomicLock31Spec> {
        AtomicLock31W::new(self, 0)
    }
}
#[doc = "Lock flag register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock31Spec;
impl crate::RegisterSpec for AtomicLock31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_31::R`](R) reader structure"]
impl crate::Readable for AtomicLock31Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_31::W`](W) writer structure"]
impl crate::Writable for AtomicLock31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_31 to value 0"]
impl crate::Resettable for AtomicLock31Spec {
    const RESET_VALUE: u32 = 0;
}

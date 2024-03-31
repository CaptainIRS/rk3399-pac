#[doc = "Register `ATOMIC_LOCK_22` reader"]
pub type R = crate::R<AtomicLock22Spec>;
#[doc = "Register `ATOMIC_LOCK_22` writer"]
pub type W = crate::W<AtomicLock22Spec>;
#[doc = "Field `ATOMIC_LOCK_22` reader - lock flag bit 22"]
pub type AtomicLock22R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_22` writer - lock flag bit 22"]
pub type AtomicLock22W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 22"]
    #[inline(always)]
    pub fn atomic_lock_22(&self) -> AtomicLock22R {
        AtomicLock22R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_22(&mut self) -> AtomicLock22W<AtomicLock22Spec> {
        AtomicLock22W::new(self, 0)
    }
}
#[doc = "Lock flag register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock22Spec;
impl crate::RegisterSpec for AtomicLock22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_22::R`](R) reader structure"]
impl crate::Readable for AtomicLock22Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_22::W`](W) writer structure"]
impl crate::Writable for AtomicLock22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_22 to value 0"]
impl crate::Resettable for AtomicLock22Spec {
    const RESET_VALUE: u32 = 0;
}

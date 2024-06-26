#[doc = "Register `ATOMIC_LOCK_11` reader"]
pub type R = crate::R<AtomicLock11Spec>;
#[doc = "Register `ATOMIC_LOCK_11` writer"]
pub type W = crate::W<AtomicLock11Spec>;
#[doc = "Field `ATOMIC_LOCK_11` reader - lock flag bit 11"]
pub type AtomicLock11R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_11` writer - lock flag bit 11"]
pub type AtomicLock11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 11"]
    #[inline(always)]
    pub fn atomic_lock_11(&self) -> AtomicLock11R {
        AtomicLock11R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_11(&mut self) -> AtomicLock11W<AtomicLock11Spec> {
        AtomicLock11W::new(self, 0)
    }
}
#[doc = "Lock flag register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock11Spec;
impl crate::RegisterSpec for AtomicLock11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_11::R`](R) reader structure"]
impl crate::Readable for AtomicLock11Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_11::W`](W) writer structure"]
impl crate::Writable for AtomicLock11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_11 to value 0"]
impl crate::Resettable for AtomicLock11Spec {
    const RESET_VALUE: u32 = 0;
}

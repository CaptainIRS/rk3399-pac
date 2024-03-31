#[doc = "Register `ATOMIC_LOCK_18` reader"]
pub type R = crate::R<AtomicLock18Spec>;
#[doc = "Register `ATOMIC_LOCK_18` writer"]
pub type W = crate::W<AtomicLock18Spec>;
#[doc = "Field `ATOMIC_LOCK_18` reader - lock flag bit 18"]
pub type AtomicLock18R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_18` writer - lock flag bit 18"]
pub type AtomicLock18W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 18"]
    #[inline(always)]
    pub fn atomic_lock_18(&self) -> AtomicLock18R {
        AtomicLock18R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_18(&mut self) -> AtomicLock18W<AtomicLock18Spec> {
        AtomicLock18W::new(self, 0)
    }
}
#[doc = "Lock flag register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock18Spec;
impl crate::RegisterSpec for AtomicLock18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_18::R`](R) reader structure"]
impl crate::Readable for AtomicLock18Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_18::W`](W) writer structure"]
impl crate::Writable for AtomicLock18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_18 to value 0"]
impl crate::Resettable for AtomicLock18Spec {
    const RESET_VALUE: u32 = 0;
}

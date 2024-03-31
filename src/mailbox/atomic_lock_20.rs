#[doc = "Register `ATOMIC_LOCK_20` reader"]
pub type R = crate::R<AtomicLock20Spec>;
#[doc = "Register `ATOMIC_LOCK_20` writer"]
pub type W = crate::W<AtomicLock20Spec>;
#[doc = "Field `ATOMIC_LOCK_20` reader - lock flag bit 20"]
pub type AtomicLock20R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_20` writer - lock flag bit 20"]
pub type AtomicLock20W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 20"]
    #[inline(always)]
    pub fn atomic_lock_20(&self) -> AtomicLock20R {
        AtomicLock20R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_20(&mut self) -> AtomicLock20W<AtomicLock20Spec> {
        AtomicLock20W::new(self, 0)
    }
}
#[doc = "Lock flag register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock20Spec;
impl crate::RegisterSpec for AtomicLock20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_20::R`](R) reader structure"]
impl crate::Readable for AtomicLock20Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_20::W`](W) writer structure"]
impl crate::Writable for AtomicLock20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_20 to value 0"]
impl crate::Resettable for AtomicLock20Spec {
    const RESET_VALUE: u32 = 0;
}

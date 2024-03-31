#[doc = "Register `ATOMIC_LOCK_17` reader"]
pub type R = crate::R<AtomicLock17Spec>;
#[doc = "Register `ATOMIC_LOCK_17` writer"]
pub type W = crate::W<AtomicLock17Spec>;
#[doc = "Field `ATOMIC_LOCK_17` reader - lock flag bit 17"]
pub type AtomicLock17R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_17` writer - lock flag bit 17"]
pub type AtomicLock17W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 17"]
    #[inline(always)]
    pub fn atomic_lock_17(&self) -> AtomicLock17R {
        AtomicLock17R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_17(&mut self) -> AtomicLock17W<AtomicLock17Spec> {
        AtomicLock17W::new(self, 0)
    }
}
#[doc = "Lock flag register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock17Spec;
impl crate::RegisterSpec for AtomicLock17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_17::R`](R) reader structure"]
impl crate::Readable for AtomicLock17Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_17::W`](W) writer structure"]
impl crate::Writable for AtomicLock17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_17 to value 0"]
impl crate::Resettable for AtomicLock17Spec {
    const RESET_VALUE: u32 = 0;
}

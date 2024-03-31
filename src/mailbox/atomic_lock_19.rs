#[doc = "Register `ATOMIC_LOCK_19` reader"]
pub type R = crate::R<AtomicLock19Spec>;
#[doc = "Register `ATOMIC_LOCK_19` writer"]
pub type W = crate::W<AtomicLock19Spec>;
#[doc = "Field `ATOMIC_LOCK_19` reader - lock flag bit 19"]
pub type AtomicLock19R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_19` writer - lock flag bit 19"]
pub type AtomicLock19W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 19"]
    #[inline(always)]
    pub fn atomic_lock_19(&self) -> AtomicLock19R {
        AtomicLock19R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_19(&mut self) -> AtomicLock19W<AtomicLock19Spec> {
        AtomicLock19W::new(self, 0)
    }
}
#[doc = "Lock flag register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock19Spec;
impl crate::RegisterSpec for AtomicLock19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_19::R`](R) reader structure"]
impl crate::Readable for AtomicLock19Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_19::W`](W) writer structure"]
impl crate::Writable for AtomicLock19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_19 to value 0"]
impl crate::Resettable for AtomicLock19Spec {
    const RESET_VALUE: u32 = 0;
}

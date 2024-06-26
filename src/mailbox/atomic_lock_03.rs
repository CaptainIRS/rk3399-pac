#[doc = "Register `ATOMIC_LOCK_03` reader"]
pub type R = crate::R<AtomicLock03Spec>;
#[doc = "Register `ATOMIC_LOCK_03` writer"]
pub type W = crate::W<AtomicLock03Spec>;
#[doc = "Field `ATOMIC_LOCK_03` reader - lock flag bit 03"]
pub type AtomicLock03R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_03` writer - lock flag bit 03"]
pub type AtomicLock03W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 03"]
    #[inline(always)]
    pub fn atomic_lock_03(&self) -> AtomicLock03R {
        AtomicLock03R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 03"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_03(&mut self) -> AtomicLock03W<AtomicLock03Spec> {
        AtomicLock03W::new(self, 0)
    }
}
#[doc = "Lock flag register 03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_03::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_03::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock03Spec;
impl crate::RegisterSpec for AtomicLock03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_03::R`](R) reader structure"]
impl crate::Readable for AtomicLock03Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_03::W`](W) writer structure"]
impl crate::Writable for AtomicLock03Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_03 to value 0"]
impl crate::Resettable for AtomicLock03Spec {
    const RESET_VALUE: u32 = 0;
}

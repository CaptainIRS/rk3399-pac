#[doc = "Register `ATOMIC_LOCK_01` reader"]
pub type R = crate::R<AtomicLock01Spec>;
#[doc = "Register `ATOMIC_LOCK_01` writer"]
pub type W = crate::W<AtomicLock01Spec>;
#[doc = "Field `ATOMIC_LOCK_01` reader - lock flag bit 01"]
pub type AtomicLock01R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_01` writer - lock flag bit 01"]
pub type AtomicLock01W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 01"]
    #[inline(always)]
    pub fn atomic_lock_01(&self) -> AtomicLock01R {
        AtomicLock01R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 01"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_01(&mut self) -> AtomicLock01W<AtomicLock01Spec> {
        AtomicLock01W::new(self, 0)
    }
}
#[doc = "Lock flag register 01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock01Spec;
impl crate::RegisterSpec for AtomicLock01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_01::R`](R) reader structure"]
impl crate::Readable for AtomicLock01Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_01::W`](W) writer structure"]
impl crate::Writable for AtomicLock01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_01 to value 0"]
impl crate::Resettable for AtomicLock01Spec {
    const RESET_VALUE: u32 = 0;
}

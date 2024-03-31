#[doc = "Register `ATOMIC_LOCK_07` reader"]
pub type R = crate::R<AtomicLock07Spec>;
#[doc = "Register `ATOMIC_LOCK_07` writer"]
pub type W = crate::W<AtomicLock07Spec>;
#[doc = "Field `ATOMIC_LOCK_07` reader - lock flag bit 07"]
pub type AtomicLock07R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_07` writer - lock flag bit 07"]
pub type AtomicLock07W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 07"]
    #[inline(always)]
    pub fn atomic_lock_07(&self) -> AtomicLock07R {
        AtomicLock07R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 07"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_07(&mut self) -> AtomicLock07W<AtomicLock07Spec> {
        AtomicLock07W::new(self, 0)
    }
}
#[doc = "Lock flag register 07\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_07::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_07::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock07Spec;
impl crate::RegisterSpec for AtomicLock07Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_07::R`](R) reader structure"]
impl crate::Readable for AtomicLock07Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_07::W`](W) writer structure"]
impl crate::Writable for AtomicLock07Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_07 to value 0"]
impl crate::Resettable for AtomicLock07Spec {
    const RESET_VALUE: u32 = 0;
}

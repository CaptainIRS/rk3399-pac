#[doc = "Register `ATOMIC_LOCK_08` reader"]
pub type R = crate::R<AtomicLock08Spec>;
#[doc = "Register `ATOMIC_LOCK_08` writer"]
pub type W = crate::W<AtomicLock08Spec>;
#[doc = "Field `ATOMIC_LOCK_08` reader - lock flag bit 08"]
pub type AtomicLock08R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_08` writer - lock flag bit 08"]
pub type AtomicLock08W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 08"]
    #[inline(always)]
    pub fn atomic_lock_08(&self) -> AtomicLock08R {
        AtomicLock08R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 08"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_08(&mut self) -> AtomicLock08W<AtomicLock08Spec> {
        AtomicLock08W::new(self, 0)
    }
}
#[doc = "Lock flag register 08\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_08::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_08::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock08Spec;
impl crate::RegisterSpec for AtomicLock08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_08::R`](R) reader structure"]
impl crate::Readable for AtomicLock08Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_08::W`](W) writer structure"]
impl crate::Writable for AtomicLock08Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_08 to value 0"]
impl crate::Resettable for AtomicLock08Spec {
    const RESET_VALUE: u32 = 0;
}

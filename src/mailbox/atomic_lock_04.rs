#[doc = "Register `ATOMIC_LOCK_04` reader"]
pub type R = crate::R<AtomicLock04Spec>;
#[doc = "Register `ATOMIC_LOCK_04` writer"]
pub type W = crate::W<AtomicLock04Spec>;
#[doc = "Field `ATOMIC_LOCK_04` reader - lock flag bit 04"]
pub type AtomicLock04R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_04` writer - lock flag bit 04"]
pub type AtomicLock04W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 04"]
    #[inline(always)]
    pub fn atomic_lock_04(&self) -> AtomicLock04R {
        AtomicLock04R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 04"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_04(&mut self) -> AtomicLock04W<AtomicLock04Spec> {
        AtomicLock04W::new(self, 0)
    }
}
#[doc = "Lock flag register 04\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_04::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_04::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock04Spec;
impl crate::RegisterSpec for AtomicLock04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_04::R`](R) reader structure"]
impl crate::Readable for AtomicLock04Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_04::W`](W) writer structure"]
impl crate::Writable for AtomicLock04Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_04 to value 0"]
impl crate::Resettable for AtomicLock04Spec {
    const RESET_VALUE: u32 = 0;
}

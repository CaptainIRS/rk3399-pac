#[doc = "Register `ATOMIC_LOCK_05` reader"]
pub type R = crate::R<AtomicLock05Spec>;
#[doc = "Register `ATOMIC_LOCK_05` writer"]
pub type W = crate::W<AtomicLock05Spec>;
#[doc = "Field `ATOMIC_LOCK_05` reader - lock flag bit 05"]
pub type AtomicLock05R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_05` writer - lock flag bit 05"]
pub type AtomicLock05W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 05"]
    #[inline(always)]
    pub fn atomic_lock_05(&self) -> AtomicLock05R {
        AtomicLock05R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 05"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_05(&mut self) -> AtomicLock05W<AtomicLock05Spec> {
        AtomicLock05W::new(self, 0)
    }
}
#[doc = "Lock flag register 05\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_05::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_05::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock05Spec;
impl crate::RegisterSpec for AtomicLock05Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_05::R`](R) reader structure"]
impl crate::Readable for AtomicLock05Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_05::W`](W) writer structure"]
impl crate::Writable for AtomicLock05Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_05 to value 0"]
impl crate::Resettable for AtomicLock05Spec {
    const RESET_VALUE: u32 = 0;
}

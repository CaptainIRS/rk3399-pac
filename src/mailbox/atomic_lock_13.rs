#[doc = "Register `ATOMIC_LOCK_13` reader"]
pub type R = crate::R<AtomicLock13Spec>;
#[doc = "Register `ATOMIC_LOCK_13` writer"]
pub type W = crate::W<AtomicLock13Spec>;
#[doc = "Field `ATOMIC_LOCK_13` reader - lock flag bit 13"]
pub type AtomicLock13R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_13` writer - lock flag bit 13"]
pub type AtomicLock13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 13"]
    #[inline(always)]
    pub fn atomic_lock_13(&self) -> AtomicLock13R {
        AtomicLock13R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_13(&mut self) -> AtomicLock13W<AtomicLock13Spec> {
        AtomicLock13W::new(self, 0)
    }
}
#[doc = "Lock flag register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock13Spec;
impl crate::RegisterSpec for AtomicLock13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_13::R`](R) reader structure"]
impl crate::Readable for AtomicLock13Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_13::W`](W) writer structure"]
impl crate::Writable for AtomicLock13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_13 to value 0"]
impl crate::Resettable for AtomicLock13Spec {
    const RESET_VALUE: u32 = 0;
}

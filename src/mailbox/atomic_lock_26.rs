#[doc = "Register `ATOMIC_LOCK_26` reader"]
pub type R = crate::R<AtomicLock26Spec>;
#[doc = "Register `ATOMIC_LOCK_26` writer"]
pub type W = crate::W<AtomicLock26Spec>;
#[doc = "Field `ATOMIC_LOCK_26` reader - lock flag bit 26"]
pub type AtomicLock26R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_26` writer - lock flag bit 26"]
pub type AtomicLock26W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 26"]
    #[inline(always)]
    pub fn atomic_lock_26(&self) -> AtomicLock26R {
        AtomicLock26R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_26(&mut self) -> AtomicLock26W<AtomicLock26Spec> {
        AtomicLock26W::new(self, 0)
    }
}
#[doc = "Lock flag register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock26Spec;
impl crate::RegisterSpec for AtomicLock26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_26::R`](R) reader structure"]
impl crate::Readable for AtomicLock26Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_26::W`](W) writer structure"]
impl crate::Writable for AtomicLock26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_26 to value 0"]
impl crate::Resettable for AtomicLock26Spec {
    const RESET_VALUE: u32 = 0;
}

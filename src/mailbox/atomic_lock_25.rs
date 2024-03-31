#[doc = "Register `ATOMIC_LOCK_25` reader"]
pub type R = crate::R<AtomicLock25Spec>;
#[doc = "Register `ATOMIC_LOCK_25` writer"]
pub type W = crate::W<AtomicLock25Spec>;
#[doc = "Field `ATOMIC_LOCK_25` reader - lock flag bit 25"]
pub type AtomicLock25R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_25` writer - lock flag bit 25"]
pub type AtomicLock25W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 25"]
    #[inline(always)]
    pub fn atomic_lock_25(&self) -> AtomicLock25R {
        AtomicLock25R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_25(&mut self) -> AtomicLock25W<AtomicLock25Spec> {
        AtomicLock25W::new(self, 0)
    }
}
#[doc = "Lock flag register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock25Spec;
impl crate::RegisterSpec for AtomicLock25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_25::R`](R) reader structure"]
impl crate::Readable for AtomicLock25Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_25::W`](W) writer structure"]
impl crate::Writable for AtomicLock25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_25 to value 0"]
impl crate::Resettable for AtomicLock25Spec {
    const RESET_VALUE: u32 = 0;
}

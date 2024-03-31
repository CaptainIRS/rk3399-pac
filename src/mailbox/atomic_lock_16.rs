#[doc = "Register `ATOMIC_LOCK_16` reader"]
pub type R = crate::R<AtomicLock16Spec>;
#[doc = "Register `ATOMIC_LOCK_16` writer"]
pub type W = crate::W<AtomicLock16Spec>;
#[doc = "Field `ATOMIC_LOCK_16` reader - lock flag bit 16"]
pub type AtomicLock16R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_16` writer - lock flag bit 16"]
pub type AtomicLock16W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 16"]
    #[inline(always)]
    pub fn atomic_lock_16(&self) -> AtomicLock16R {
        AtomicLock16R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_16(&mut self) -> AtomicLock16W<AtomicLock16Spec> {
        AtomicLock16W::new(self, 0)
    }
}
#[doc = "Lock flag register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock16Spec;
impl crate::RegisterSpec for AtomicLock16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_16::R`](R) reader structure"]
impl crate::Readable for AtomicLock16Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_16::W`](W) writer structure"]
impl crate::Writable for AtomicLock16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_16 to value 0"]
impl crate::Resettable for AtomicLock16Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `ATOMIC_LOCK_06` reader"]
pub type R = crate::R<AtomicLock06Spec>;
#[doc = "Register `ATOMIC_LOCK_06` writer"]
pub type W = crate::W<AtomicLock06Spec>;
#[doc = "Field `ATOMIC_LOCK_06` reader - lock flag bit 06"]
pub type AtomicLock06R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_06` writer - lock flag bit 06"]
pub type AtomicLock06W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 06"]
    #[inline(always)]
    pub fn atomic_lock_06(&self) -> AtomicLock06R {
        AtomicLock06R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 06"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_06(&mut self) -> AtomicLock06W<AtomicLock06Spec> {
        AtomicLock06W::new(self, 0)
    }
}
#[doc = "Lock flag register 06\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_06::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_06::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock06Spec;
impl crate::RegisterSpec for AtomicLock06Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_06::R`](R) reader structure"]
impl crate::Readable for AtomicLock06Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_06::W`](W) writer structure"]
impl crate::Writable for AtomicLock06Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_06 to value 0"]
impl crate::Resettable for AtomicLock06Spec {
    const RESET_VALUE: u32 = 0;
}

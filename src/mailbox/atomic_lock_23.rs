#[doc = "Register `ATOMIC_LOCK_23` reader"]
pub type R = crate::R<AtomicLock23Spec>;
#[doc = "Register `ATOMIC_LOCK_23` writer"]
pub type W = crate::W<AtomicLock23Spec>;
#[doc = "Field `ATOMIC_LOCK_23` reader - lock flag bit 23"]
pub type AtomicLock23R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_23` writer - lock flag bit 23"]
pub type AtomicLock23W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 23"]
    #[inline(always)]
    pub fn atomic_lock_23(&self) -> AtomicLock23R {
        AtomicLock23R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_23(&mut self) -> AtomicLock23W<AtomicLock23Spec> {
        AtomicLock23W::new(self, 0)
    }
}
#[doc = "Lock flag register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock23Spec;
impl crate::RegisterSpec for AtomicLock23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_23::R`](R) reader structure"]
impl crate::Readable for AtomicLock23Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_23::W`](W) writer structure"]
impl crate::Writable for AtomicLock23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_23 to value 0"]
impl crate::Resettable for AtomicLock23Spec {
    const RESET_VALUE: u32 = 0;
}

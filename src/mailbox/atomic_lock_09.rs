#[doc = "Register `ATOMIC_LOCK_09` reader"]
pub type R = crate::R<AtomicLock09Spec>;
#[doc = "Register `ATOMIC_LOCK_09` writer"]
pub type W = crate::W<AtomicLock09Spec>;
#[doc = "Field `ATOMIC_LOCK_09` reader - lock flag bit 09"]
pub type AtomicLock09R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_09` writer - lock flag bit 09"]
pub type AtomicLock09W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 09"]
    #[inline(always)]
    pub fn atomic_lock_09(&self) -> AtomicLock09R {
        AtomicLock09R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 09"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_09(&mut self) -> AtomicLock09W<AtomicLock09Spec> {
        AtomicLock09W::new(self, 0)
    }
}
#[doc = "Lock flag register 09\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atomic_lock_09::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atomic_lock_09::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtomicLock09Spec;
impl crate::RegisterSpec for AtomicLock09Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atomic_lock_09::R`](R) reader structure"]
impl crate::Readable for AtomicLock09Spec {}
#[doc = "`write(|w| ..)` method takes [`atomic_lock_09::W`](W) writer structure"]
impl crate::Writable for AtomicLock09Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATOMIC_LOCK_09 to value 0"]
impl crate::Resettable for AtomicLock09Spec {
    const RESET_VALUE: u32 = 0;
}

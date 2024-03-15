#[doc = "Register `MAILBOX_ATOMIC_LOCK_15` reader"]
pub type R = crate::R<MailboxAtomicLock15Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_15` writer"]
pub type W = crate::W<MailboxAtomicLock15Spec>;
#[doc = "Field `ATOMIC_LOCK_15` reader - lock flag bit 15"]
pub type AtomicLock15R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_15` writer - lock flag bit 15"]
pub type AtomicLock15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 15"]
    #[inline(always)]
    pub fn atomic_lock_15(&self) -> AtomicLock15R {
        AtomicLock15R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_15(&mut self) -> AtomicLock15W<MailboxAtomicLock15Spec> {
        AtomicLock15W::new(self, 0)
    }
}
#[doc = "Lock flag register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock15Spec;
impl crate::RegisterSpec for MailboxAtomicLock15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_15::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock15Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_15::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_15 to value 0"]
impl crate::Resettable for MailboxAtomicLock15Spec {
    const RESET_VALUE: u32 = 0;
}

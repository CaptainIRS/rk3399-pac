#[doc = "Register `MAILBOX_ATOMIC_LOCK_28` reader"]
pub type R = crate::R<MailboxAtomicLock28Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_28` writer"]
pub type W = crate::W<MailboxAtomicLock28Spec>;
#[doc = "Field `ATOMIC_LOCK_28` reader - lock flag bit 28"]
pub type AtomicLock28R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_28` writer - lock flag bit 28"]
pub type AtomicLock28W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 28"]
    #[inline(always)]
    pub fn atomic_lock_28(&self) -> AtomicLock28R {
        AtomicLock28R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_28(&mut self) -> AtomicLock28W<MailboxAtomicLock28Spec> {
        AtomicLock28W::new(self, 0)
    }
}
#[doc = "Lock flag register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock28Spec;
impl crate::RegisterSpec for MailboxAtomicLock28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_28::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock28Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_28::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_28 to value 0"]
impl crate::Resettable for MailboxAtomicLock28Spec {
    const RESET_VALUE: u32 = 0;
}

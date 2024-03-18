#[doc = "Register `MAILBOX_ATOMIC_LOCK_29` reader"]
pub type R = crate::R<MailboxAtomicLock29Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_29` writer"]
pub type W = crate::W<MailboxAtomicLock29Spec>;
#[doc = "Field `ATOMIC_LOCK_29` reader - lock flag bit 29"]
pub type AtomicLock29R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_29` writer - lock flag bit 29"]
pub type AtomicLock29W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 29"]
    #[inline(always)]
    pub fn atomic_lock_29(&self) -> AtomicLock29R {
        AtomicLock29R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_29(&mut self) -> AtomicLock29W<MailboxAtomicLock29Spec> {
        AtomicLock29W::new(self, 0)
    }
}
#[doc = "Lock flag register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock29Spec;
impl crate::RegisterSpec for MailboxAtomicLock29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_29::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock29Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_29::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_29 to value 0"]
impl crate::Resettable for MailboxAtomicLock29Spec {
    const RESET_VALUE: u32 = 0;
}

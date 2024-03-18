#[doc = "Register `MAILBOX_ATOMIC_LOCK_12` reader"]
pub type R = crate::R<MailboxAtomicLock12Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_12` writer"]
pub type W = crate::W<MailboxAtomicLock12Spec>;
#[doc = "Field `ATOMIC_LOCK_12` reader - lock flag bit 12"]
pub type AtomicLock12R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_12` writer - lock flag bit 12"]
pub type AtomicLock12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 12"]
    #[inline(always)]
    pub fn atomic_lock_12(&self) -> AtomicLock12R {
        AtomicLock12R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_12(&mut self) -> AtomicLock12W<MailboxAtomicLock12Spec> {
        AtomicLock12W::new(self, 0)
    }
}
#[doc = "Lock flag register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock12Spec;
impl crate::RegisterSpec for MailboxAtomicLock12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_12::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock12Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_12::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_12 to value 0"]
impl crate::Resettable for MailboxAtomicLock12Spec {
    const RESET_VALUE: u32 = 0;
}

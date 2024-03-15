#[doc = "Register `MAILBOX_ATOMIC_LOCK_30` reader"]
pub type R = crate::R<MailboxAtomicLock30Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_30` writer"]
pub type W = crate::W<MailboxAtomicLock30Spec>;
#[doc = "Field `ATOMIC_LOCK_30` reader - lock flag bit 30"]
pub type AtomicLock30R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_30` writer - lock flag bit 30"]
pub type AtomicLock30W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 30"]
    #[inline(always)]
    pub fn atomic_lock_30(&self) -> AtomicLock30R {
        AtomicLock30R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_30(&mut self) -> AtomicLock30W<MailboxAtomicLock30Spec> {
        AtomicLock30W::new(self, 0)
    }
}
#[doc = "Lock flag register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock30Spec;
impl crate::RegisterSpec for MailboxAtomicLock30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_30::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock30Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_30::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_30 to value 0"]
impl crate::Resettable for MailboxAtomicLock30Spec {
    const RESET_VALUE: u32 = 0;
}

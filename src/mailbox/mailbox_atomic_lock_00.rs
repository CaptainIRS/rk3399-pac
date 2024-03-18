#[doc = "Register `MAILBOX_ATOMIC_LOCK_00` reader"]
pub type R = crate::R<MailboxAtomicLock00Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_00` writer"]
pub type W = crate::W<MailboxAtomicLock00Spec>;
#[doc = "Field `ATOMIC_LOCK_00` reader - lock flag bit 00"]
pub type AtomicLock00R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_00` writer - lock flag bit 00"]
pub type AtomicLock00W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 00"]
    #[inline(always)]
    pub fn atomic_lock_00(&self) -> AtomicLock00R {
        AtomicLock00R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 00"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_00(&mut self) -> AtomicLock00W<MailboxAtomicLock00Spec> {
        AtomicLock00W::new(self, 0)
    }
}
#[doc = "Lock flag register 00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_00::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_00::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock00Spec;
impl crate::RegisterSpec for MailboxAtomicLock00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_00::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock00Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_00::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_00 to value 0"]
impl crate::Resettable for MailboxAtomicLock00Spec {
    const RESET_VALUE: u32 = 0;
}

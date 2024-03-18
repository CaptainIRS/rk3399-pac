#[doc = "Register `MAILBOX_ATOMIC_LOCK_24` reader"]
pub type R = crate::R<MailboxAtomicLock24Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_24` writer"]
pub type W = crate::W<MailboxAtomicLock24Spec>;
#[doc = "Field `ATOMIC_LOCK_24` reader - lock flag bit 24"]
pub type AtomicLock24R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_24` writer - lock flag bit 24"]
pub type AtomicLock24W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 24"]
    #[inline(always)]
    pub fn atomic_lock_24(&self) -> AtomicLock24R {
        AtomicLock24R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_24(&mut self) -> AtomicLock24W<MailboxAtomicLock24Spec> {
        AtomicLock24W::new(self, 0)
    }
}
#[doc = "Lock flag register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock24Spec;
impl crate::RegisterSpec for MailboxAtomicLock24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_24::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock24Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_24::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_24 to value 0"]
impl crate::Resettable for MailboxAtomicLock24Spec {
    const RESET_VALUE: u32 = 0;
}

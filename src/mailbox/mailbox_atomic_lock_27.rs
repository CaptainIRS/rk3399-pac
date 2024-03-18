#[doc = "Register `MAILBOX_ATOMIC_LOCK_27` reader"]
pub type R = crate::R<MailboxAtomicLock27Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_27` writer"]
pub type W = crate::W<MailboxAtomicLock27Spec>;
#[doc = "Field `ATOMIC_LOCK_27` reader - lock flag bit 27"]
pub type AtomicLock27R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_27` writer - lock flag bit 27"]
pub type AtomicLock27W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 27"]
    #[inline(always)]
    pub fn atomic_lock_27(&self) -> AtomicLock27R {
        AtomicLock27R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_27(&mut self) -> AtomicLock27W<MailboxAtomicLock27Spec> {
        AtomicLock27W::new(self, 0)
    }
}
#[doc = "Lock flag register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock27Spec;
impl crate::RegisterSpec for MailboxAtomicLock27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_27::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock27Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_27::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_27 to value 0"]
impl crate::Resettable for MailboxAtomicLock27Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MAILBOX_ATOMIC_LOCK_01` reader"]
pub type R = crate::R<MailboxAtomicLock01Spec>;
#[doc = "Register `MAILBOX_ATOMIC_LOCK_01` writer"]
pub type W = crate::W<MailboxAtomicLock01Spec>;
#[doc = "Field `ATOMIC_LOCK_01` reader - lock flag bit 01"]
pub type AtomicLock01R = crate::BitReader;
#[doc = "Field `ATOMIC_LOCK_01` writer - lock flag bit 01"]
pub type AtomicLock01W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock flag bit 01"]
    #[inline(always)]
    pub fn atomic_lock_01(&self) -> AtomicLock01R {
        AtomicLock01R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock flag bit 01"]
    #[inline(always)]
    #[must_use]
    pub fn atomic_lock_01(&mut self) -> AtomicLock01W<MailboxAtomicLock01Spec> {
        AtomicLock01W::new(self, 0)
    }
}
#[doc = "Lock flag register 01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_atomic_lock_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_atomic_lock_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxAtomicLock01Spec;
impl crate::RegisterSpec for MailboxAtomicLock01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_atomic_lock_01::R`](R) reader structure"]
impl crate::Readable for MailboxAtomicLock01Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_atomic_lock_01::W`](W) writer structure"]
impl crate::Writable for MailboxAtomicLock01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_ATOMIC_LOCK_01 to value 0"]
impl crate::Resettable for MailboxAtomicLock01Spec {
    const RESET_VALUE: u32 = 0;
}

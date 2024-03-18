#[doc = "Register `MAILBOX_A2B_INTEN` reader"]
pub type R = crate::R<MailboxA2bIntenSpec>;
#[doc = "Register `MAILBOX_A2B_INTEN` writer"]
pub type W = crate::W<MailboxA2bIntenSpec>;
#[doc = "Field `INT0` reader - interrupt enable for int0"]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT0` writer - interrupt enable for int0"]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - interrupt enable for int1"]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT1` writer - interrupt enable for int1"]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2` reader - interrupt enable for int2"]
pub type Int2R = crate::BitReader;
#[doc = "Field `INT2` writer - interrupt enable for int2"]
pub type Int2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT3` reader - interrupt enable for int3"]
pub type Int3R = crate::BitReader;
#[doc = "Field `INT3` writer - interrupt enable for int3"]
pub type Int3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt enable for int0"]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt enable for int1"]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt enable for int2"]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt enable for int3"]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt enable for int0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<MailboxA2bIntenSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt enable for int1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<MailboxA2bIntenSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt enable for int2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<MailboxA2bIntenSpec> {
        Int2W::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt enable for int3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> Int3W<MailboxA2bIntenSpec> {
        Int3W::new(self, 3)
    }
}
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxA2bIntenSpec;
impl crate::RegisterSpec for MailboxA2bIntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_a2b_inten::R`](R) reader structure"]
impl crate::Readable for MailboxA2bIntenSpec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_a2b_inten::W`](W) writer structure"]
impl crate::Writable for MailboxA2bIntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_A2B_INTEN to value 0"]
impl crate::Resettable for MailboxA2bIntenSpec {
    const RESET_VALUE: u32 = 0;
}

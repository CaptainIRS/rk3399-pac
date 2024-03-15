#[doc = "Register `MAILBOX_B2A_DAT_1` reader"]
pub type R = crate::R<MailboxB2aDat1Spec>;
#[doc = "Register `MAILBOX_B2A_DAT_1` writer"]
pub type W = crate::W<MailboxB2aDat1Spec>;
#[doc = "Field `DATA` reader - data of MCU to Cortex-A53/Cortex-A72"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - data of MCU to Cortex-A53/Cortex-A72"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data of MCU to Cortex-A53/Cortex-A72"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data of MCU to Cortex-A53/Cortex-A72"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<MailboxB2aDat1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 data 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_dat_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_dat_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxB2aDat1Spec;
impl crate::RegisterSpec for MailboxB2aDat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_b2a_dat_1::R`](R) reader structure"]
impl crate::Readable for MailboxB2aDat1Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_b2a_dat_1::W`](W) writer structure"]
impl crate::Writable for MailboxB2aDat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_B2A_DAT_1 to value 0"]
impl crate::Resettable for MailboxB2aDat1Spec {
    const RESET_VALUE: u32 = 0;
}

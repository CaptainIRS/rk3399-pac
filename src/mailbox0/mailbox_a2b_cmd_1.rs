#[doc = "Register `MAILBOX_A2B_CMD_1` reader"]
pub type R = crate::R<MailboxA2bCmd1Spec>;
#[doc = "Register `MAILBOX_A2B_CMD_1` writer"]
pub type W = crate::W<MailboxA2bCmd1Spec>;
#[doc = "Field `COMMAND` reader - command of Cortex-A53/Cortex-A72 to Cortex-M0"]
pub type CommandR = crate::FieldReader<u32>;
#[doc = "Field `COMMAND` writer - command of Cortex-A53/Cortex-A72 to Cortex-M0"]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - command of Cortex-A53/Cortex-A72 to Cortex-M0"]
    #[inline(always)]
    pub fn command(&self) -> CommandR {
        CommandR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - command of Cortex-A53/Cortex-A72 to Cortex-M0"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> CommandW<MailboxA2bCmd1Spec> {
        CommandW::new(self, 0)
    }
}
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 command 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_a2b_cmd_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_a2b_cmd_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxA2bCmd1Spec;
impl crate::RegisterSpec for MailboxA2bCmd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_a2b_cmd_1::R`](R) reader structure"]
impl crate::Readable for MailboxA2bCmd1Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_a2b_cmd_1::W`](W) writer structure"]
impl crate::Writable for MailboxA2bCmd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_A2B_CMD_1 to value 0"]
impl crate::Resettable for MailboxA2bCmd1Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MAILBOX_B2A_CMD_0` reader"]
pub type R = crate::R<MailboxB2aCmd0Spec>;
#[doc = "Register `MAILBOX_B2A_CMD_0` writer"]
pub type W = crate::W<MailboxB2aCmd0Spec>;
#[doc = "Field `COMMAND` reader - command of MCU to Cortex-A53/Cortex-A72"]
pub type CommandR = crate::FieldReader<u32>;
#[doc = "Field `COMMAND` writer - command of MCU to Cortex-A53/Cortex-A72"]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - command of MCU to Cortex-A53/Cortex-A72"]
    #[inline(always)]
    pub fn command(&self) -> CommandR {
        CommandR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - command of MCU to Cortex-A53/Cortex-A72"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> CommandW<MailboxB2aCmd0Spec> {
        CommandW::new(self, 0)
    }
}
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mailbox_b2a_cmd_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mailbox_b2a_cmd_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MailboxB2aCmd0Spec;
impl crate::RegisterSpec for MailboxB2aCmd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox_b2a_cmd_0::R`](R) reader structure"]
impl crate::Readable for MailboxB2aCmd0Spec {}
#[doc = "`write(|w| ..)` method takes [`mailbox_b2a_cmd_0::W`](W) writer structure"]
impl crate::Writable for MailboxB2aCmd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAILBOX_B2A_CMD_0 to value 0"]
impl crate::Resettable for MailboxB2aCmd0Spec {
    const RESET_VALUE: u32 = 0;
}

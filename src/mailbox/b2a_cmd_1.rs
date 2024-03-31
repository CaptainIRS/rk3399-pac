#[doc = "Register `B2A_CMD_1` reader"]
pub type R = crate::R<B2aCmd1Spec>;
#[doc = "Register `B2A_CMD_1` writer"]
pub type W = crate::W<B2aCmd1Spec>;
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
    pub fn command(&mut self) -> CommandW<B2aCmd1Spec> {
        CommandW::new(self, 0)
    }
}
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_cmd_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_cmd_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B2aCmd1Spec;
impl crate::RegisterSpec for B2aCmd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b2a_cmd_1::R`](R) reader structure"]
impl crate::Readable for B2aCmd1Spec {}
#[doc = "`write(|w| ..)` method takes [`b2a_cmd_1::W`](W) writer structure"]
impl crate::Writable for B2aCmd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B2A_CMD_1 to value 0"]
impl crate::Resettable for B2aCmd1Spec {
    const RESET_VALUE: u32 = 0;
}

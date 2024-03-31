#[doc = "Register `B2A_CMD_2` reader"]
pub type R = crate::R<B2aCmd2Spec>;
#[doc = "Register `B2A_CMD_2` writer"]
pub type W = crate::W<B2aCmd2Spec>;
#[doc = "Field `COMMAND` reader - command of Cortex-M0 to\n\nCortex-A53/Cortex-A72"]
pub type CommandR = crate::FieldReader<u32>;
#[doc = "Field `COMMAND` writer - command of Cortex-M0 to\n\nCortex-A53/Cortex-A72"]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - command of Cortex-M0 to\n\nCortex-A53/Cortex-A72"]
    #[inline(always)]
    pub fn command(&self) -> CommandR {
        CommandR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - command of Cortex-M0 to\n\nCortex-A53/Cortex-A72"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> CommandW<B2aCmd2Spec> {
        CommandW::new(self, 0)
    }
}
#[doc = "Cortex-M0 to Cortex-A53/Cortex-A72 command 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b2a_cmd_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b2a_cmd_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B2aCmd2Spec;
impl crate::RegisterSpec for B2aCmd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b2a_cmd_2::R`](R) reader structure"]
impl crate::Readable for B2aCmd2Spec {}
#[doc = "`write(|w| ..)` method takes [`b2a_cmd_2::W`](W) writer structure"]
impl crate::Writable for B2aCmd2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B2A_CMD_2 to value 0"]
impl crate::Resettable for B2aCmd2Spec {
    const RESET_VALUE: u32 = 0;
}

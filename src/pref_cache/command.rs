#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<CommandSpec>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Field `COMMAND` reader - The possible command is\n\n1 = Clear entire cache"]
pub type CommandR = crate::FieldReader;
#[doc = "Field `COMMAND` writer - The possible command is\n\n1 = Clear entire cache"]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - The possible command is\n\n1 = Clear entire cache"]
    #[inline(always)]
    pub fn command(&self) -> CommandR {
        CommandR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The possible command is\n\n1 = Clear entire cache"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> CommandW<CommandSpec> {
        CommandW::new(self, 0)
    }
}
#[doc = "Command setting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for CommandSpec {
    const RESET_VALUE: u32 = 0;
}

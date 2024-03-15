#[doc = "Register `SDMMC_CMDARG` reader"]
pub type R = crate::R<SdmmcCmdargSpec>;
#[doc = "Register `SDMMC_CMDARG` writer"]
pub type W = crate::W<SdmmcCmdargSpec>;
#[doc = "Field `CMD_ARG` reader - Value indicates command argument to be passed to card."]
pub type CmdArgR = crate::FieldReader<u32>;
#[doc = "Field `CMD_ARG` writer - Value indicates command argument to be passed to card."]
pub type CmdArgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to card."]
    #[inline(always)]
    pub fn cmd_arg(&self) -> CmdArgR {
        CmdArgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to card."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_arg(&mut self) -> CmdArgW<SdmmcCmdargSpec> {
        CmdArgW::new(self, 0)
    }
}
#[doc = "Command-argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_cmdarg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_cmdarg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcCmdargSpec;
impl crate::RegisterSpec for SdmmcCmdargSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_cmdarg::R`](R) reader structure"]
impl crate::Readable for SdmmcCmdargSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_cmdarg::W`](W) writer structure"]
impl crate::Writable for SdmmcCmdargSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_CMDARG to value 0"]
impl crate::Resettable for SdmmcCmdargSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CMD_BASE` reader"]
pub type R = crate::R<CmdBaseSpec>;
#[doc = "Register `CMD_BASE` writer"]
pub type W = crate::W<CmdBaseSpec>;
#[doc = "Field `SW_CMD_BASE` reader - RGA command codes base address"]
pub type SwCmdBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_CMD_BASE` writer - RGA command codes base address"]
pub type SwCmdBaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RGA command codes base address"]
    #[inline(always)]
    pub fn sw_cmd_base(&self) -> SwCmdBaseR {
        SwCmdBaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RGA command codes base address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cmd_base(&mut self) -> SwCmdBaseW<CmdBaseSpec> {
        SwCmdBaseW::new(self, 0)
    }
}
#[doc = "RGA command codes base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdBaseSpec;
impl crate::RegisterSpec for CmdBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_base::R`](R) reader structure"]
impl crate::Readable for CmdBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_base::W`](W) writer structure"]
impl crate::Writable for CmdBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_BASE to value 0x1234_5678"]
impl crate::Resettable for CmdBaseSpec {
    const RESET_VALUE: u32 = 0x1234_5678;
}

#[doc = "Register `EDPI_CMD_SIZE` reader"]
pub type R = crate::R<EdpiCmdSizeSpec>;
#[doc = "Register `EDPI_CMD_SIZE` writer"]
pub type W = crate::W<EdpiCmdSizeSpec>;
#[doc = "Field `EDPI_ALLOWED_CMD_SIZE` reader - edpi_allowed_cmd_size\n\nThis field configures the maximum allowed size for an eDPI write\n\nmemory command, measured in pixels. Automatic partitioning of\n\ndata obtained from eDPI is permanently enabled."]
pub type EdpiAllowedCmdSizeR = crate::FieldReader<u16>;
#[doc = "Field `EDPI_ALLOWED_CMD_SIZE` writer - edpi_allowed_cmd_size\n\nThis field configures the maximum allowed size for an eDPI write\n\nmemory command, measured in pixels. Automatic partitioning of\n\ndata obtained from eDPI is permanently enabled."]
pub type EdpiAllowedCmdSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - edpi_allowed_cmd_size\n\nThis field configures the maximum allowed size for an eDPI write\n\nmemory command, measured in pixels. Automatic partitioning of\n\ndata obtained from eDPI is permanently enabled."]
    #[inline(always)]
    pub fn edpi_allowed_cmd_size(&self) -> EdpiAllowedCmdSizeR {
        EdpiAllowedCmdSizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - edpi_allowed_cmd_size\n\nThis field configures the maximum allowed size for an eDPI write\n\nmemory command, measured in pixels. Automatic partitioning of\n\ndata obtained from eDPI is permanently enabled."]
    #[inline(always)]
    #[must_use]
    pub fn edpi_allowed_cmd_size(&mut self) -> EdpiAllowedCmdSizeW<EdpiCmdSizeSpec> {
        EdpiAllowedCmdSizeW::new(self, 0)
    }
}
#[doc = "eDPI Packet Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edpi_cmd_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edpi_cmd_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EdpiCmdSizeSpec;
impl crate::RegisterSpec for EdpiCmdSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edpi_cmd_size::R`](R) reader structure"]
impl crate::Readable for EdpiCmdSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`edpi_cmd_size::W`](W) writer structure"]
impl crate::Writable for EdpiCmdSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EDPI_CMD_SIZE to value 0"]
impl crate::Resettable for EdpiCmdSizeSpec {
    const RESET_VALUE: u32 = 0;
}

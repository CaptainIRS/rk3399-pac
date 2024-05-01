#[doc = "Register `DPI_LP_CMD_TIM` reader"]
pub type R = crate::R<DpiLpCmdTimSpec>;
#[doc = "Register `DPI_LP_CMD_TIM` writer"]
pub type W = crate::W<DpiLpCmdTimSpec>;
#[doc = "Field `INVACT_LPCMD_TIME` reader - invact_lpcmd_time\n\nThis field is used for the transmission of commands in low-power\n\nmode. It defines the size, in bytes, of the largest packet that can fit\n\nin a\n\nline during the VACT region."]
pub type InvactLpcmdTimeR = crate::FieldReader;
#[doc = "Field `INVACT_LPCMD_TIME` writer - invact_lpcmd_time\n\nThis field is used for the transmission of commands in low-power\n\nmode. It defines the size, in bytes, of the largest packet that can fit\n\nin a\n\nline during the VACT region."]
pub type InvactLpcmdTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUTVACT_LPCMD_TIME` reader - outvact_lpcmd_time\n\nThis field is used for the transmission of commands in low-power\n\nmode. It defines the size, in bytes, of the largest packet that can fit\n\nin a\n\nline during the VSA, VBP, and VFP regions."]
pub type OutvactLpcmdTimeR = crate::FieldReader;
#[doc = "Field `OUTVACT_LPCMD_TIME` writer - outvact_lpcmd_time\n\nThis field is used for the transmission of commands in low-power\n\nmode. It defines the size, in bytes, of the largest packet that can fit\n\nin a\n\nline during the VSA, VBP, and VFP regions."]
pub type OutvactLpcmdTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - invact_lpcmd_time\n\nThis field is used for the transmission of commands in low-power\n\nmode. It defines the size, in bytes, of the largest packet that can fit\n\nin a\n\nline during the VACT region."]
    #[inline(always)]
    pub fn invact_lpcmd_time(&self) -> InvactLpcmdTimeR {
        InvactLpcmdTimeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - outvact_lpcmd_time\n\nThis field is used for the transmission of commands in low-power\n\nmode. It defines the size, in bytes, of the largest packet that can fit\n\nin a\n\nline during the VSA, VBP, and VFP regions."]
    #[inline(always)]
    pub fn outvact_lpcmd_time(&self) -> OutvactLpcmdTimeR {
        OutvactLpcmdTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - invact_lpcmd_time\n\nThis field is used for the transmission of commands in low-power\n\nmode. It defines the size, in bytes, of the largest packet that can fit\n\nin a\n\nline during the VACT region."]
    #[inline(always)]
    #[must_use]
    pub fn invact_lpcmd_time(&mut self) -> InvactLpcmdTimeW<DpiLpCmdTimSpec> {
        InvactLpcmdTimeW::new(self, 0)
    }
    #[doc = "Bits 16:23 - outvact_lpcmd_time\n\nThis field is used for the transmission of commands in low-power\n\nmode. It defines the size, in bytes, of the largest packet that can fit\n\nin a\n\nline during the VSA, VBP, and VFP regions."]
    #[inline(always)]
    #[must_use]
    pub fn outvact_lpcmd_time(&mut self) -> OutvactLpcmdTimeW<DpiLpCmdTimSpec> {
        OutvactLpcmdTimeW::new(self, 16)
    }
}
#[doc = "Low-Power Command Timing Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_lp_cmd_tim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_lp_cmd_tim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiLpCmdTimSpec;
impl crate::RegisterSpec for DpiLpCmdTimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_lp_cmd_tim::R`](R) reader structure"]
impl crate::Readable for DpiLpCmdTimSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi_lp_cmd_tim::W`](W) writer structure"]
impl crate::Writable for DpiLpCmdTimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_LP_CMD_TIM to value 0"]
impl crate::Resettable for DpiLpCmdTimSpec {
    const RESET_VALUE: u32 = 0;
}

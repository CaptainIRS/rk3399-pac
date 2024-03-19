#[doc = "Register `DDR_PI_REG_115` reader"]
pub type R = crate::R<DdrPiReg115Spec>;
#[doc = "Register `DDR_PI_REG_115` writer"]
pub type W = crate::W<DdrPiReg115Spec>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_F2` reader - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_init_start de-assertion and a\n\ndfi_init_complete assertion from the PHY. The suffix \"_f2\" of the\n\nparameter name is omitted when in non-DFS mode"]
pub type PiTdfiInitCompleteF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_F2` writer - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_init_start de-assertion and a\n\ndfi_init_complete assertion from the PHY. The suffix \"_f2\" of the\n\nparameter name is omitted when in non-DFS mode"]
pub type PiTdfiInitCompleteF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_CLKDISABLE_2_INIT_START` reader - Defines the delay from the asserting of dfi_dram_clk_disable to the\n\nasserting of dfi_init_start in DFI clock."]
pub type PiClkdisable2InitStartR = crate::FieldReader;
#[doc = "Field `PI_CLKDISABLE_2_INIT_START` writer - Defines the delay from the asserting of dfi_dram_clk_disable to the\n\nasserting of dfi_init_start in DFI clock."]
pub type PiClkdisable2InitStartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_INIT_STARTORCOMPLETE_2_CLKDISABLE` reader - Defines the delay from deasserting of dfi_init_start or asserting of\n\ndfi_init_complete to deasserting of dfi_dram_clk_disable in DFI\n\nclock."]
pub type PiInitStartorcomplete2ClkdisableR = crate::FieldReader;
#[doc = "Field `PI_INIT_STARTORCOMPLETE_2_CLKDISABLE` writer - Defines the delay from deasserting of dfi_init_start or asserting of\n\ndfi_init_complete to deasserting of dfi_dram_clk_disable in DFI\n\nclock."]
pub type PiInitStartorcomplete2ClkdisableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_init_start de-assertion and a\n\ndfi_init_complete assertion from the PHY. The suffix \"_f2\" of the\n\nparameter name is omitted when in non-DFS mode"]
    #[inline(always)]
    pub fn pi_tdfi_init_complete_f2(&self) -> PiTdfiInitCompleteF2R {
        PiTdfiInitCompleteF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Defines the delay from the asserting of dfi_dram_clk_disable to the\n\nasserting of dfi_init_start in DFI clock."]
    #[inline(always)]
    pub fn pi_clkdisable_2_init_start(&self) -> PiClkdisable2InitStartR {
        PiClkdisable2InitStartR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Defines the delay from deasserting of dfi_init_start or asserting of\n\ndfi_init_complete to deasserting of dfi_dram_clk_disable in DFI\n\nclock."]
    #[inline(always)]
    pub fn pi_init_startorcomplete_2_clkdisable(&self) -> PiInitStartorcomplete2ClkdisableR {
        PiInitStartorcomplete2ClkdisableR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_init_start de-assertion and a\n\ndfi_init_complete assertion from the PHY. The suffix \"_f2\" of the\n\nparameter name is omitted when in non-DFS mode"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_complete_f2(&mut self) -> PiTdfiInitCompleteF2W<DdrPiReg115Spec> {
        PiTdfiInitCompleteF2W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Defines the delay from the asserting of dfi_dram_clk_disable to the\n\nasserting of dfi_init_start in DFI clock."]
    #[inline(always)]
    #[must_use]
    pub fn pi_clkdisable_2_init_start(&mut self) -> PiClkdisable2InitStartW<DdrPiReg115Spec> {
        PiClkdisable2InitStartW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Defines the delay from deasserting of dfi_init_start or asserting of\n\ndfi_init_complete to deasserting of dfi_dram_clk_disable in DFI\n\nclock."]
    #[inline(always)]
    #[must_use]
    pub fn pi_init_startorcomplete_2_clkdisable(
        &mut self,
    ) -> PiInitStartorcomplete2ClkdisableW<DdrPiReg115Spec> {
        PiInitStartorcomplete2ClkdisableW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 115\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_115::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_115::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg115Spec;
impl crate::RegisterSpec for DdrPiReg115Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_115::R`](R) reader structure"]
impl crate::Readable for DdrPiReg115Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_115::W`](W) writer structure"]
impl crate::Writable for DdrPiReg115Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_115 to value 0"]
impl crate::Resettable for DdrPiReg115Spec {
    const RESET_VALUE: u32 = 0;
}

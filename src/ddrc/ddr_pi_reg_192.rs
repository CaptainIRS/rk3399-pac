#[doc = "Register `DDR_PI_REG_192` reader"]
pub type R = crate::R<DdrPiReg192Spec>;
#[doc = "Register `DDR_PI_REG_192` writer"]
pub type W = crate::W<DdrPiReg192Spec>;
#[doc = "Field `PI_TDFI_CTRLUPD_INTERVAL_F2` reader - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI\n\nclocks), the maximum cycles between dfi_ctrlupd_req assertions.\n\nIf programmed to a non-zero, a timing violation causes an interrupt\n\nand bit0 set in the PI_REG_193.pi_update_error_status\n\nparameter. The suffix '_f2' of the parameter name is omitted when\n\nin non-DFS mode."]
pub type PiTdfiCtrlupdIntervalF2R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_CTRLUPD_INTERVAL_F2` writer - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI\n\nclocks), the maximum cycles between dfi_ctrlupd_req assertions.\n\nIf programmed to a non-zero, a timing violation causes an interrupt\n\nand bit0 set in the PI_REG_193.pi_update_error_status\n\nparameter. The suffix '_f2' of the parameter name is omitted when\n\nin non-DFS mode."]
pub type PiTdfiCtrlupdIntervalF2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI\n\nclocks), the maximum cycles between dfi_ctrlupd_req assertions.\n\nIf programmed to a non-zero, a timing violation causes an interrupt\n\nand bit0 set in the PI_REG_193.pi_update_error_status\n\nparameter. The suffix '_f2' of the parameter name is omitted when\n\nin non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_ctrlupd_interval_f2(&self) -> PiTdfiCtrlupdIntervalF2R {
        PiTdfiCtrlupdIntervalF2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI\n\nclocks), the maximum cycles between dfi_ctrlupd_req assertions.\n\nIf programmed to a non-zero, a timing violation causes an interrupt\n\nand bit0 set in the PI_REG_193.pi_update_error_status\n\nparameter. The suffix '_f2' of the parameter name is omitted when\n\nin non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrlupd_interval_f2(&mut self) -> PiTdfiCtrlupdIntervalF2W<DdrPiReg192Spec> {
        PiTdfiCtrlupdIntervalF2W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 192\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_192::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_192::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg192Spec;
impl crate::RegisterSpec for DdrPiReg192Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_192::R`](R) reader structure"]
impl crate::Readable for DdrPiReg192Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_192::W`](W) writer structure"]
impl crate::Writable for DdrPiReg192Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_192 to value 0"]
impl crate::Resettable for DdrPiReg192Spec {
    const RESET_VALUE: u32 = 0;
}

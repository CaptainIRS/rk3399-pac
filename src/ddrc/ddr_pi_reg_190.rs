#[doc = "Register `DDR_PI_REG_190` reader"]
pub type R = crate::R<DdrPiReg190Spec>;
#[doc = "Register `DDR_PI_REG_190` writer"]
pub type W = crate::W<DdrPiReg190Spec>;
#[doc = "Field `PI_TDFI_CTRLUPD_INTERVAL_F1` reader - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI\n\nclocks), the maximum cycles between dfi_ctrlupd_req assertions.\n\nIf programmed to a non-zero, a timing violation causes an interrupt\n\nand bit0 set in the PI_REG_193.pi_update_error_status\n\nparameter. The suffix '_f1' of the parameter name is omitted when\n\nin non-DFS mode."]
pub type PiTdfiCtrlupdIntervalF1R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_CTRLUPD_INTERVAL_F1` writer - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI\n\nclocks), the maximum cycles between dfi_ctrlupd_req assertions.\n\nIf programmed to a non-zero, a timing violation causes an interrupt\n\nand bit0 set in the PI_REG_193.pi_update_error_status\n\nparameter. The suffix '_f1' of the parameter name is omitted when\n\nin non-DFS mode."]
pub type PiTdfiCtrlupdIntervalF1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI\n\nclocks), the maximum cycles between dfi_ctrlupd_req assertions.\n\nIf programmed to a non-zero, a timing violation causes an interrupt\n\nand bit0 set in the PI_REG_193.pi_update_error_status\n\nparameter. The suffix '_f1' of the parameter name is omitted when\n\nin non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_ctrlupd_interval_f1(&self) -> PiTdfiCtrlupdIntervalF1R {
        PiTdfiCtrlupdIntervalF1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI\n\nclocks), the maximum cycles between dfi_ctrlupd_req assertions.\n\nIf programmed to a non-zero, a timing violation causes an interrupt\n\nand bit0 set in the PI_REG_193.pi_update_error_status\n\nparameter. The suffix '_f1' of the parameter name is omitted when\n\nin non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrlupd_interval_f1(&mut self) -> PiTdfiCtrlupdIntervalF1W<DdrPiReg190Spec> {
        PiTdfiCtrlupdIntervalF1W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 190\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_190::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_190::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg190Spec;
impl crate::RegisterSpec for DdrPiReg190Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_190::R`](R) reader structure"]
impl crate::Readable for DdrPiReg190Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_190::W`](W) writer structure"]
impl crate::Writable for DdrPiReg190Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_190 to value 0"]
impl crate::Resettable for DdrPiReg190Spec {
    const RESET_VALUE: u32 = 0;
}

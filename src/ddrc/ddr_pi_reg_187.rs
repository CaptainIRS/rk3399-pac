#[doc = "Register `DDR_PI_REG_187` reader"]
pub type R = crate::R<DdrPiReg187Spec>;
#[doc = "Register `DDR_PI_REG_187` writer"]
pub type W = crate::W<DdrPiReg187Spec>;
#[doc = "Field `PI_TDFI_CTRLUPD_MIN` reader - Reports the DFI tCTRLUPD_MIN timing parameter (in DFI clocks), the minimum cycles that dfi_ctrlupd_req must be asserted."]
pub type PiTdfiCtrlupdMinR = crate::FieldReader;
#[doc = "Field `PI_TDFI_CTRLUPD_MAX_F0` reader - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit (1) set in the PI_REG_193.pi_update_error_status parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCtrlupdMaxF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CTRLUPD_MAX_F0` writer - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit (1) set in the PI_REG_193.pi_update_error_status parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCtrlupdMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Reports the DFI tCTRLUPD_MIN timing parameter (in DFI clocks), the minimum cycles that dfi_ctrlupd_req must be asserted."]
    #[inline(always)]
    pub fn pi_tdfi_ctrlupd_min(&self) -> PiTdfiCtrlupdMinR {
        PiTdfiCtrlupdMinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit (1) set in the PI_REG_193.pi_update_error_status parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_ctrlupd_max_f0(&self) -> PiTdfiCtrlupdMaxF0R {
        PiTdfiCtrlupdMaxF0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit (1) set in the PI_REG_193.pi_update_error_status parameter. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrlupd_max_f0(&mut self) -> PiTdfiCtrlupdMaxF0W<DdrPiReg187Spec> {
        PiTdfiCtrlupdMaxF0W::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 187\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_187::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_187::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg187Spec;
impl crate::RegisterSpec for DdrPiReg187Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_187::R`](R) reader structure"]
impl crate::Readable for DdrPiReg187Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_187::W`](W) writer structure"]
impl crate::Writable for DdrPiReg187Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_187 to value 0"]
impl crate::Resettable for DdrPiReg187Spec {
    const RESET_VALUE: u32 = 0;
}

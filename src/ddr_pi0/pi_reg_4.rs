#[doc = "Register `PI_REG_4` reader"]
pub type R = crate::R<PiReg4Spec>;
#[doc = "Register `PI_REG_4` writer"]
pub type W = crate::W<PiReg4Spec>;
#[doc = "Field `PI_TDFI_PHYMSTR_MAX_F1` reader - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit0 set in the PI_REG_22.pi_control_error_status parameter. The suffix f1 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrMaxF1R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYMSTR_MAX_F1` writer - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit0 set in the PI_REG_22.pi_control_error_status parameter. The suffix f1 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrMaxF1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit0 set in the PI_REG_22.pi_control_error_status parameter. The suffix f1 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phymstr_max_f1(&self) -> PiTdfiPhymstrMaxF1R {
        PiTdfiPhymstrMaxF1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit0 set in the PI_REG_22.pi_control_error_status parameter. The suffix f1 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phymstr_max_f1(&mut self) -> PiTdfiPhymstrMaxF1W<PiReg4Spec> {
        PiTdfiPhymstrMaxF1W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg4Spec;
impl crate::RegisterSpec for PiReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_4::R`](R) reader structure"]
impl crate::Readable for PiReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_4::W`](W) writer structure"]
impl crate::Writable for PiReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_4 to value 0"]
impl crate::Resettable for PiReg4Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DDR_PI_REG_6` reader"]
pub type R = crate::R<DdrPiReg6Spec>;
#[doc = "Register `DDR_PI_REG_6` writer"]
pub type W = crate::W<DdrPiReg6Spec>;
#[doc = "Field `PI_TDFI_PHYMSTR_MAX_F2` reader - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phymstr_req following the assertion\n\nof dfi_phymstr_ack can be asserted. If programmed to a non-zero,\n\na timing violation causes an interrupt and bit0 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrMaxF2R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYMSTR_MAX_F2` writer - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phymstr_req following the assertion\n\nof dfi_phymstr_ack can be asserted. If programmed to a non-zero,\n\na timing violation causes an interrupt and bit0 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phymstr_req following the assertion\n\nof dfi_phymstr_ack can be asserted. If programmed to a non-zero,\n\na timing violation causes an interrupt and bit0 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phymstr_max_f2(&self) -> PiTdfiPhymstrMaxF2R {
        PiTdfiPhymstrMaxF2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phymstr_req following the assertion\n\nof dfi_phymstr_ack can be asserted. If programmed to a non-zero,\n\na timing violation causes an interrupt and bit0 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phymstr_max_f2(&mut self) -> PiTdfiPhymstrMaxF2W<DdrPiReg6Spec> {
        PiTdfiPhymstrMaxF2W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg6Spec;
impl crate::RegisterSpec for DdrPiReg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_6::R`](R) reader structure"]
impl crate::Readable for DdrPiReg6Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_6::W`](W) writer structure"]
impl crate::Writable for DdrPiReg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_6 to value 0"]
impl crate::Resettable for DdrPiReg6Spec {
    const RESET_VALUE: u32 = 0;
}

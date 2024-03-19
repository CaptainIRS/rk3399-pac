#[doc = "Register `DDR_PI_REG_3` reader"]
pub type R = crate::R<DdrPiReg3Spec>;
#[doc = "Register `DDR_PI_REG_3` writer"]
pub type W = crate::W<DdrPiReg3Spec>;
#[doc = "Field `PI_TDFI_PHYMSTR_RESP_F0` reader - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrRespF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_PHYMSTR_RESP_F0` writer - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrRespF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phymstr_resp_f0(&self) -> PiTdfiPhymstrRespF0R {
        PiTdfiPhymstrRespF0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phymstr_resp_f0(&mut self) -> PiTdfiPhymstrRespF0W<DdrPiReg3Spec> {
        PiTdfiPhymstrRespF0W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg3Spec;
impl crate::RegisterSpec for DdrPiReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_3::R`](R) reader structure"]
impl crate::Readable for DdrPiReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_3::W`](W) writer structure"]
impl crate::Writable for DdrPiReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_3 to value 0"]
impl crate::Resettable for DdrPiReg3Spec {
    const RESET_VALUE: u32 = 0;
}

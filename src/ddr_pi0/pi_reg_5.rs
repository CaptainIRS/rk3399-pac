#[doc = "Register `PI_REG_5` reader"]
pub type R = crate::R<PiReg5Spec>;
#[doc = "Register `PI_REG_5` writer"]
pub type W = crate::W<PiReg5Spec>;
#[doc = "Field `PI_TDFI_PHYMSTR_RESP_F1` reader - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f1 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrRespF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_PHYMSTR_RESP_F1` writer - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f1 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrRespF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f1 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phymstr_resp_f1(&self) -> PiTdfiPhymstrRespF1R {
        PiTdfiPhymstrRespF1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f1 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phymstr_resp_f1(&mut self) -> PiTdfiPhymstrRespF1W<PiReg5Spec> {
        PiTdfiPhymstrRespF1W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg5Spec;
impl crate::RegisterSpec for PiReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_5::R`](R) reader structure"]
impl crate::Readable for PiReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_5::W`](W) writer structure"]
impl crate::Writable for PiReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_5 to value 0"]
impl crate::Resettable for PiReg5Spec {
    const RESET_VALUE: u32 = 0;
}

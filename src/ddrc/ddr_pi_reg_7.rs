#[doc = "Register `DDR_PI_REG_7` reader"]
pub type R = crate::R<DdrPiReg7Spec>;
#[doc = "Register `DDR_PI_REG_7` writer"]
pub type W = crate::W<DdrPiReg7Spec>;
#[doc = "Field `PI_TDFI_PHYMSTR_RESP_F2` reader - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f2 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrRespF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_PHYMSTR_RESP_F2` writer - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f2 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhymstrRespF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_TDFI_PHYUPD_RESP_F0` reader - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit8 set in the PI_REG_193.pi_update_error_status parameter and bit8 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdRespF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_PHYUPD_RESP_F0` writer - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit8 set in the PI_REG_193.pi_update_error_status parameter and bit8 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdRespF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f2 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phymstr_resp_f2(&self) -> PiTdfiPhymstrRespF2R {
        PiTdfiPhymstrRespF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit8 set in the PI_REG_193.pi_update_error_status parameter and bit8 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phyupd_resp_f0(&self) -> PiTdfiPhyupdRespF0R {
        PiTdfiPhyupdRespF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit1 set in the PI_REG_22.pi_control_error_status parameter. The suffix f2 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phymstr_resp_f2(&mut self) -> PiTdfiPhymstrRespF2W<DdrPiReg7Spec> {
        PiTdfiPhymstrRespF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation causes an interrupt and bit8 set in the PI_REG_193.pi_update_error_status parameter and bit8 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phyupd_resp_f0(&mut self) -> PiTdfiPhyupdRespF0W<DdrPiReg7Spec> {
        PiTdfiPhyupdRespF0W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg7Spec;
impl crate::RegisterSpec for DdrPiReg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_7::R`](R) reader structure"]
impl crate::Readable for DdrPiReg7Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_7::W`](W) writer structure"]
impl crate::Writable for DdrPiReg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_7 to value 0"]
impl crate::Resettable for DdrPiReg7Spec {
    const RESET_VALUE: u32 = 0;
}

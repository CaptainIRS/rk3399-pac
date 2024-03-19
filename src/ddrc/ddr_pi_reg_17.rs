#[doc = "Register `DDR_PI_REG_17` reader"]
pub type R = crate::R<DdrPiReg17Spec>;
#[doc = "Register `DDR_PI_REG_17` writer"]
pub type W = crate::W<DdrPiReg17Spec>;
#[doc = "Field `PI_TDFI_PHYUPD_RESP_F2` reader - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_phyupd_req assertion and a\n\ndfi_phyupd_ack assertion. If programmed to a non-zero, a timing\n\nviolation causes an interrupt and bit8 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit8 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdRespF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_PHYUPD_RESP_F2` writer - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_phyupd_req assertion and a\n\ndfi_phyupd_ack assertion. If programmed to a non-zero, a timing\n\nviolation causes an interrupt and bit8 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit8 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdRespF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_phyupd_req assertion and a\n\ndfi_phyupd_ack assertion. If programmed to a non-zero, a timing\n\nviolation causes an interrupt and bit8 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit8 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phyupd_resp_f2(&self) -> PiTdfiPhyupdRespF2R {
        PiTdfiPhyupdRespF2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_phyupd_req assertion and a\n\ndfi_phyupd_ack assertion. If programmed to a non-zero, a timing\n\nviolation causes an interrupt and bit8 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit8 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phyupd_resp_f2(&mut self) -> PiTdfiPhyupdRespF2W<DdrPiReg17Spec> {
        PiTdfiPhyupdRespF2W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg17Spec;
impl crate::RegisterSpec for DdrPiReg17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_17::R`](R) reader structure"]
impl crate::Readable for DdrPiReg17Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_17::W`](W) writer structure"]
impl crate::Writable for DdrPiReg17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_17 to value 0"]
impl crate::Resettable for DdrPiReg17Spec {
    const RESET_VALUE: u32 = 0;
}

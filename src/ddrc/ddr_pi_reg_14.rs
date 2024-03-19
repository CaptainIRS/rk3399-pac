#[doc = "Register `DDR_PI_REG_14` reader"]
pub type R = crate::R<DdrPiReg14Spec>;
#[doc = "Register `DDR_PI_REG_14` writer"]
pub type W = crate::W<DdrPiReg14Spec>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE1_F1` reader - Defines the DFI tPHYUPD_TYPE1 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 1. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit5 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit5 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f1 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType1F1R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE1_F1` writer - Defines the DFI tPHYUPD_TYPE1 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 1. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit5 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit5 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f1 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType1F1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE1 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 1. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit5 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit5 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f1 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phyupd_type1_f1(&self) -> PiTdfiPhyupdType1F1R {
        PiTdfiPhyupdType1F1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE1 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 1. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit5 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit5 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f1 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phyupd_type1_f1(&mut self) -> PiTdfiPhyupdType1F1W<DdrPiReg14Spec> {
        PiTdfiPhyupdType1F1W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg14Spec;
impl crate::RegisterSpec for DdrPiReg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_14::R`](R) reader structure"]
impl crate::Readable for DdrPiReg14Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_14::W`](W) writer structure"]
impl crate::Writable for DdrPiReg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_14 to value 0"]
impl crate::Resettable for DdrPiReg14Spec {
    const RESET_VALUE: u32 = 0;
}

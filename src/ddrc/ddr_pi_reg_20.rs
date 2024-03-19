#[doc = "Register `DDR_PI_REG_20` reader"]
pub type R = crate::R<DdrPiReg20Spec>;
#[doc = "Register `DDR_PI_REG_20` writer"]
pub type W = crate::W<DdrPiReg20Spec>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE2_F2` reader - Defines the DFI tPHYUPD_TYPE2 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 2. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit6 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit6 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType2F2R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE2_F2` writer - Defines the DFI tPHYUPD_TYPE2 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 2. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit6 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit6 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType2F2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE2 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 2. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit6 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit6 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phyupd_type2_f2(&self) -> PiTdfiPhyupdType2F2R {
        PiTdfiPhyupdType2F2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE2 timing parameter (in DFI clocks),\n\nthe maximum cycles that dfi_phyupd_req can assert after\n\ndfi_phyupd_ack for dfi_phyupd_type 2. If programmed to a\n\nnon-zero, a timing violation causes an interrupt and bit6 set in the\n\nPI_REG_193.pi_update_error_status parameter and bit6 set in the\n\nPI_REG_22.pi_control_error_status parameter. The suffix f2 of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phyupd_type2_f2(&mut self) -> PiTdfiPhyupdType2F2W<DdrPiReg20Spec> {
        PiTdfiPhyupdType2F2W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg20Spec;
impl crate::RegisterSpec for DdrPiReg20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_20::R`](R) reader structure"]
impl crate::Readable for DdrPiReg20Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_20::W`](W) writer structure"]
impl crate::Writable for DdrPiReg20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_20 to value 0"]
impl crate::Resettable for DdrPiReg20Spec {
    const RESET_VALUE: u32 = 0;
}

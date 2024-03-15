#[doc = "Register `PI_REG_8` reader"]
pub type R = crate::R<PiReg8Spec>;
#[doc = "Register `PI_REG_8` writer"]
pub type W = crate::W<PiReg8Spec>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE0_F0` reader - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation causes an interrupt and bit4 set in the PI_REG_193.pi_update_error_status parameter and bit4 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType0F0R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_PHYUPD_TYPE0_F0` writer - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation causes an interrupt and bit4 set in the PI_REG_193.pi_update_error_status parameter and bit4 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiPhyupdType0F0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation causes an interrupt and bit4 set in the PI_REG_193.pi_update_error_status parameter and bit4 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_phyupd_type0_f0(&self) -> PiTdfiPhyupdType0F0R {
        PiTdfiPhyupdType0F0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation causes an interrupt and bit4 set in the PI_REG_193.pi_update_error_status parameter and bit4 set in the PI_REG_22.pi_control_error_status parameter. The suffix f0 of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_phyupd_type0_f0(&mut self) -> PiTdfiPhyupdType0F0W<PiReg8Spec> {
        PiTdfiPhyupdType0F0W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg8Spec;
impl crate::RegisterSpec for PiReg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_8::R`](R) reader structure"]
impl crate::Readable for PiReg8Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_8::W`](W) writer structure"]
impl crate::Writable for PiReg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_8 to value 0"]
impl crate::Resettable for PiReg8Spec {
    const RESET_VALUE: u32 = 0;
}

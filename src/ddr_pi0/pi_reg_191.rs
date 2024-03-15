#[doc = "Register `PI_REG_191` reader"]
pub type R = crate::R<PiReg191Spec>;
#[doc = "Register `PI_REG_191` writer"]
pub type W = crate::W<PiReg191Spec>;
#[doc = "Field `PI_TDFI_CTRLUPD_MAX_F2` reader - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit (1) set in the PI_REG_193.pi_update_error_status parameter. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCtrlupdMaxF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CTRLUPD_MAX_F2` writer - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit (1) set in the PI_REG_193.pi_update_error_status parameter. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCtrlupdMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit (1) set in the PI_REG_193.pi_update_error_status parameter. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_ctrlupd_max_f2(&self) -> PiTdfiCtrlupdMaxF2R {
        PiTdfiCtrlupdMaxF2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation causes an interrupt and bit (1) set in the PI_REG_193.pi_update_error_status parameter. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrlupd_max_f2(&mut self) -> PiTdfiCtrlupdMaxF2W<PiReg191Spec> {
        PiTdfiCtrlupdMaxF2W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 191\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_191::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_191::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg191Spec;
impl crate::RegisterSpec for PiReg191Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_191::R`](R) reader structure"]
impl crate::Readable for PiReg191Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_191::W`](W) writer structure"]
impl crate::Writable for PiReg191Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_191 to value 0"]
impl crate::Resettable for PiReg191Spec {
    const RESET_VALUE: u32 = 0;
}

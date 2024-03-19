#[doc = "Register `DDR_DENALI_CTL_277` reader"]
pub type R = crate::R<DdrDenaliCtl277Spec>;
#[doc = "Register `DDR_DENALI_CTL_277` writer"]
pub type W = crate::W<DdrDenaliCtl277Spec>;
#[doc = "Field `TDFI_CTRLUPD_MAX_F0` reader - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdMaxF0R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CTRLUPD_MAX_F0` writer - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_ctrlupd_max_f0(&self) -> TdfiCtrlupdMaxF0R {
        TdfiCtrlupdMaxF0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlupd_max_f0(&mut self) -> TdfiCtrlupdMaxF0W<DdrDenaliCtl277Spec> {
        TdfiCtrlupdMaxF0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_277::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_277::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl277Spec;
impl crate::RegisterSpec for DdrDenaliCtl277Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_277::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl277Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_277::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl277Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_277 to value 0"]
impl crate::Resettable for DdrDenaliCtl277Spec {
    const RESET_VALUE: u32 = 0;
}

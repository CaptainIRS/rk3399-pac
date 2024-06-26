#[doc = "Register `DENALI_CTL_290` reader"]
pub type R = crate::R<DenaliCtl290Spec>;
#[doc = "Register `DENALI_CTL_290` writer"]
pub type W = crate::W<DenaliCtl290Spec>;
#[doc = "Field `TDFI_CTRLUPD_INTERVAL_F1` reader - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI clocks), the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdIntervalF1R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_CTRLUPD_INTERVAL_F1` writer - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI clocks), the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdIntervalF1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI clocks), the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_ctrlupd_interval_f1(&self) -> TdfiCtrlupdIntervalF1R {
        TdfiCtrlupdIntervalF1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tCTRLUPD_INTERVAL timing parameter (in DFI clocks), the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlupd_interval_f1(&mut self) -> TdfiCtrlupdIntervalF1W<DenaliCtl290Spec> {
        TdfiCtrlupdIntervalF1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_290::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_290::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl290Spec;
impl crate::RegisterSpec for DenaliCtl290Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_290::R`](R) reader structure"]
impl crate::Readable for DenaliCtl290Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_290::W`](W) writer structure"]
impl crate::Writable for DenaliCtl290Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_290 to value 0"]
impl crate::Resettable for DenaliCtl290Spec {
    const RESET_VALUE: u32 = 0;
}

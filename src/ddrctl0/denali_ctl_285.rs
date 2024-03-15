#[doc = "Register `DENALI_CTL_285` reader"]
pub type R = crate::R<DenaliCtl285Spec>;
#[doc = "Register `DENALI_CTL_285` writer"]
pub type W = crate::W<DenaliCtl285Spec>;
#[doc = "Field `TDFI_PHYUPD_TYPE0_F1` reader - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert afterdfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType0F1R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYUPD_TYPE0_F1` writer - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert afterdfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType0F1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert afterdfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phyupd_type0_f1(&self) -> TdfiPhyupdType0F1R {
        TdfiPhyupdType0F1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert afterdfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_type0_f1(&mut self) -> TdfiPhyupdType0F1W<DenaliCtl285Spec> {
        TdfiPhyupdType0F1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_285::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_285::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl285Spec;
impl crate::RegisterSpec for DenaliCtl285Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_285::R`](R) reader structure"]
impl crate::Readable for DenaliCtl285Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_285::W`](W) writer structure"]
impl crate::Writable for DenaliCtl285Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_285 to value 0"]
impl crate::Resettable for DenaliCtl285Spec {
    const RESET_VALUE: u32 = 0;
}

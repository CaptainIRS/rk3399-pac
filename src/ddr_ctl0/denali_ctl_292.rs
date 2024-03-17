#[doc = "Register `DENALI_CTL_292` reader"]
pub type R = crate::R<DenaliCtl292Spec>;
#[doc = "Register `DENALI_CTL_292` writer"]
pub type W = crate::W<DenaliCtl292Spec>;
#[doc = "Field `TDFI_PHYUPD_TYPE0_F2` reader - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType0F2R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYUPD_TYPE0_F2` writer - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType0F2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phyupd_type0_f2(&self) -> TdfiPhyupdType0F2R {
        TdfiPhyupdType0F2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE0 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (2) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_type0_f2(&mut self) -> TdfiPhyupdType0F2W<DenaliCtl292Spec> {
        TdfiPhyupdType0F2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_292::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_292::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl292Spec;
impl crate::RegisterSpec for DenaliCtl292Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_292::R`](R) reader structure"]
impl crate::Readable for DenaliCtl292Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_292::W`](W) writer structure"]
impl crate::Writable for DenaliCtl292Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_292 to value 0"]
impl crate::Resettable for DenaliCtl292Spec {
    const RESET_VALUE: u32 = 0;
}

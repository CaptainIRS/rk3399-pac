#[doc = "Register `DDR_DENALI_CTL_293` reader"]
pub type R = crate::R<DdrDenaliCtl293Spec>;
#[doc = "Register `DDR_DENALI_CTL_293` writer"]
pub type W = crate::W<DdrDenaliCtl293Spec>;
#[doc = "Field `TDFI_PHYUPD_TYPE1_F2` reader - Defines the DFI tPHYUPD_TYPE1 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (3) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType1F2R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYUPD_TYPE1_F2` writer - Defines the DFI tPHYUPD_TYPE1 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (3) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType1F2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE1 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (3) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phyupd_type1_f2(&self) -> TdfiPhyupdType1F2R {
        TdfiPhyupdType1F2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE1 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (3) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_type1_f2(&mut self) -> TdfiPhyupdType1F2W<DdrDenaliCtl293Spec> {
        TdfiPhyupdType1F2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_293::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_293::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl293Spec;
impl crate::RegisterSpec for DdrDenaliCtl293Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_293::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl293Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_293::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl293Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_293 to value 0"]
impl crate::Resettable for DdrDenaliCtl293Spec {
    const RESET_VALUE: u32 = 0;
}

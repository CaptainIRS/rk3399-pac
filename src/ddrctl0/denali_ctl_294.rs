#[doc = "Register `DENALI_CTL_294` reader"]
pub type R = crate::R<DenaliCtl294Spec>;
#[doc = "Register `DENALI_CTL_294` writer"]
pub type W = crate::W<DenaliCtl294Spec>;
#[doc = "Field `TDFI_PHYUPD_TYPE2_F2` reader - Defines the DFI tPHYUPD_TYPE2 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (4) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType2F2R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYUPD_TYPE2_F2` writer - Defines the DFI tPHYUPD_TYPE2 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (4) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType2F2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE2 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (4) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phyupd_type2_f2(&self) -> TdfiPhyupdType2F2R {
        TdfiPhyupdType2F2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE2 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (4) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_type2_f2(&mut self) -> TdfiPhyupdType2F2W<DenaliCtl294Spec> {
        TdfiPhyupdType2F2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_294::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_294::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl294Spec;
impl crate::RegisterSpec for DenaliCtl294Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_294::R`](R) reader structure"]
impl crate::Readable for DenaliCtl294Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_294::W`](W) writer structure"]
impl crate::Writable for DenaliCtl294Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_294 to value 0"]
impl crate::Resettable for DenaliCtl294Spec {
    const RESET_VALUE: u32 = 0;
}

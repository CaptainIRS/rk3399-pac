#[doc = "Register `DDR_DENALI_CTL_295` reader"]
pub type R = crate::R<DdrDenaliCtl295Spec>;
#[doc = "Register `DDR_DENALI_CTL_295` writer"]
pub type W = crate::W<DdrDenaliCtl295Spec>;
#[doc = "Field `TDFI_PHYUPD_TYPE3_F2` reader - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation will cause an interrupt and bit (5) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType3F2R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYUPD_TYPE3_F2` writer - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation will cause an interrupt and bit (5) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdType3F2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation will cause an interrupt and bit (5) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phyupd_type3_f2(&self) -> TdfiPhyupdType3F2R {
        TdfiPhyupdType3F2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tPHYUPD_TYPE3 timing parameter (in DFI clocks), the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation will cause an interrupt and bit (5) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_type3_f2(&mut self) -> TdfiPhyupdType3F2W<DdrDenaliCtl295Spec> {
        TdfiPhyupdType3F2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_295::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_295::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl295Spec;
impl crate::RegisterSpec for DdrDenaliCtl295Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_295::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl295Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_295::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl295Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_295 to value 0"]
impl crate::Resettable for DdrDenaliCtl295Spec {
    const RESET_VALUE: u32 = 0;
}

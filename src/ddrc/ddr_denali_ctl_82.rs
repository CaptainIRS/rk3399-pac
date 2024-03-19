#[doc = "Register `DDR_DENALI_CTL_82` reader"]
pub type R = crate::R<DdrDenaliCtl82Spec>;
#[doc = "Register `DDR_DENALI_CTL_82` writer"]
pub type W = crate::W<DdrDenaliCtl82Spec>;
#[doc = "Field `UPD_PHYUPD_DFI_PROMOTE_THRESHOLD_F2` reader - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type UpdPhyupdDfiPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `UPD_PHYUPD_DFI_PROMOTE_THRESHOLD_F2` writer - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type UpdPhyupdDfiPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TDFI_PHYMSTR_MAX_F0` reader - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrMaxF0R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_PHYMSTR_MAX_F0` writer - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn upd_phyupd_dfi_promote_threshold_f2(&self) -> UpdPhyupdDfiPromoteThresholdF2R {
        UpdPhyupdDfiPromoteThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phymstr_max_f0(&self) -> TdfiPhymstrMaxF0R {
        TdfiPhymstrMaxF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn upd_phyupd_dfi_promote_threshold_f2(
        &mut self,
    ) -> UpdPhyupdDfiPromoteThresholdF2W<DdrDenaliCtl82Spec> {
        UpdPhyupdDfiPromoteThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phymstr_max_f0(&mut self) -> TdfiPhymstrMaxF0W<DdrDenaliCtl82Spec> {
        TdfiPhymstrMaxF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_82::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_82::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl82Spec;
impl crate::RegisterSpec for DdrDenaliCtl82Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_82::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl82Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_82::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl82Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_82 to value 0"]
impl crate::Resettable for DdrDenaliCtl82Spec {
    const RESET_VALUE: u32 = 0;
}

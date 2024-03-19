#[doc = "Register `DDR_DENALI_CTL_284` reader"]
pub type R = crate::R<DdrDenaliCtl284Spec>;
#[doc = "Register `DDR_DENALI_CTL_284` writer"]
pub type W = crate::W<DdrDenaliCtl284Spec>;
#[doc = "Field `RDLAT_ADJ_F0` reader - Adjustment value for PHY read timing."]
pub type RdlatAdjF0R = crate::FieldReader;
#[doc = "Field `RDLAT_ADJ_F0` writer - Adjustment value for PHY read timing."]
pub type RdlatAdjF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRLAT_ADJ_F0` reader - Adjustment value for PHY write timing."]
pub type WrlatAdjF0R = crate::FieldReader;
#[doc = "Field `WRLAT_ADJ_F0` writer - Adjustment value for PHY write timing."]
pub type WrlatAdjF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_CTRLUPD_MAX_F1` reader - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdMaxF1R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CTRLUPD_MAX_F1` writer - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdMaxF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Adjustment value for PHY read timing."]
    #[inline(always)]
    pub fn rdlat_adj_f0(&self) -> RdlatAdjF0R {
        RdlatAdjF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Adjustment value for PHY write timing."]
    #[inline(always)]
    pub fn wrlat_adj_f0(&self) -> WrlatAdjF0R {
        WrlatAdjF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_ctrlupd_max_f1(&self) -> TdfiCtrlupdMaxF1R {
        TdfiCtrlupdMaxF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Adjustment value for PHY read timing."]
    #[inline(always)]
    #[must_use]
    pub fn rdlat_adj_f0(&mut self) -> RdlatAdjF0W<DdrDenaliCtl284Spec> {
        RdlatAdjF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Adjustment value for PHY write timing."]
    #[inline(always)]
    #[must_use]
    pub fn wrlat_adj_f0(&mut self) -> WrlatAdjF0W<DdrDenaliCtl284Spec> {
        WrlatAdjF0W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlupd_max_f1(&mut self) -> TdfiCtrlupdMaxF1W<DdrDenaliCtl284Spec> {
        TdfiCtrlupdMaxF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_284::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_284::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl284Spec;
impl crate::RegisterSpec for DdrDenaliCtl284Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_284::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl284Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_284::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl284Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_284 to value 0"]
impl crate::Resettable for DdrDenaliCtl284Spec {
    const RESET_VALUE: u32 = 0;
}

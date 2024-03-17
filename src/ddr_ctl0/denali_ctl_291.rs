#[doc = "Register `DENALI_CTL_291` reader"]
pub type R = crate::R<DenaliCtl291Spec>;
#[doc = "Register `DENALI_CTL_291` writer"]
pub type W = crate::W<DenaliCtl291Spec>;
#[doc = "Field `RDLAT_ADJ_F1` reader - Adjustment value for PHY read timing."]
pub type RdlatAdjF1R = crate::FieldReader;
#[doc = "Field `RDLAT_ADJ_F1` writer - Adjustment value for PHY read timing."]
pub type RdlatAdjF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRLAT_ADJ_F1` reader - Adjustment value for PHY write timing."]
pub type WrlatAdjF1R = crate::FieldReader;
#[doc = "Field `WRLAT_ADJ_F1` writer - Adjustment value for PHY write timing."]
pub type WrlatAdjF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_CTRLUPD_MAX_F2` reader - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdMaxF2R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CTRLUPD_MAX_F2` writer - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiCtrlupdMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Adjustment value for PHY read timing."]
    #[inline(always)]
    pub fn rdlat_adj_f1(&self) -> RdlatAdjF1R {
        RdlatAdjF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Adjustment value for PHY write timing."]
    #[inline(always)]
    pub fn wrlat_adj_f1(&self) -> WrlatAdjF1R {
        WrlatAdjF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_ctrlupd_max_f2(&self) -> TdfiCtrlupdMaxF2R {
        TdfiCtrlupdMaxF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Adjustment value for PHY read timing."]
    #[inline(always)]
    #[must_use]
    pub fn rdlat_adj_f1(&mut self) -> RdlatAdjF1W<DenaliCtl291Spec> {
        RdlatAdjF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Adjustment value for PHY write timing."]
    #[inline(always)]
    #[must_use]
    pub fn wrlat_adj_f1(&mut self) -> WrlatAdjF1W<DenaliCtl291Spec> {
        WrlatAdjF1W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Defines the DFI tCTRLUPD_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlupd_max_f2(&mut self) -> TdfiCtrlupdMaxF2W<DenaliCtl291Spec> {
        TdfiCtrlupdMaxF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_291::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_291::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl291Spec;
impl crate::RegisterSpec for DenaliCtl291Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_291::R`](R) reader structure"]
impl crate::Readable for DenaliCtl291Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_291::W`](W) writer structure"]
impl crate::Writable for DenaliCtl291Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_291 to value 0"]
impl crate::Resettable for DenaliCtl291Spec {
    const RESET_VALUE: u32 = 0;
}

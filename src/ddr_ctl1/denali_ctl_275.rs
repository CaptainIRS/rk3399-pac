#[doc = "Register `DENALI_CTL_275` reader"]
pub type R = crate::R<DenaliCtl275Spec>;
#[doc = "Register `DENALI_CTL_275` writer"]
pub type W = crate::W<DenaliCtl275Spec>;
#[doc = "Field `TDFI_PHY_WRLAT` reader - Holds the calculated DFI tPHY_WRLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_en assertion. READ-ONLY"]
pub type TdfiPhyWrlatR = crate::FieldReader;
#[doc = "Field `UPDATE_ERROR_STATUS` reader - Identifies the source of any DFI MC-initiated or PHY-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. READ-ONLY"]
pub type UpdateErrorStatusR = crate::FieldReader;
#[doc = "Field `TDFI_PHY_RDLAT_F0` reader - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
pub type TdfiPhyRdlatF0R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_RDLAT_F0` writer - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
pub type TdfiPhyRdlatF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_PHY_RDLAT_F1` reader - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
pub type TdfiPhyRdlatF1R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_RDLAT_F1` writer - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
pub type TdfiPhyRdlatF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Holds the calculated DFI tPHY_WRLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_en assertion. READ-ONLY"]
    #[inline(always)]
    pub fn tdfi_phy_wrlat(&self) -> TdfiPhyWrlatR {
        TdfiPhyWrlatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Identifies the source of any DFI MC-initiated or PHY-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. READ-ONLY"]
    #[inline(always)]
    pub fn update_error_status(&self) -> UpdateErrorStatusR {
        UpdateErrorStatusR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
    #[inline(always)]
    pub fn tdfi_phy_rdlat_f0(&self) -> TdfiPhyRdlatF0R {
        TdfiPhyRdlatF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
    #[inline(always)]
    pub fn tdfi_phy_rdlat_f1(&self) -> TdfiPhyRdlatF1R {
        TdfiPhyRdlatF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_rdlat_f0(&mut self) -> TdfiPhyRdlatF0W<DenaliCtl275Spec> {
        TdfiPhyRdlatF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_rdlat_f1(&mut self) -> TdfiPhyRdlatF1W<DenaliCtl275Spec> {
        TdfiPhyRdlatF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_275::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_275::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl275Spec;
impl crate::RegisterSpec for DenaliCtl275Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_275::R`](R) reader structure"]
impl crate::Readable for DenaliCtl275Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_275::W`](W) writer structure"]
impl crate::Writable for DenaliCtl275Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_275 to value 0x0606_0000"]
impl crate::Resettable for DenaliCtl275Spec {
    const RESET_VALUE: u32 = 0x0606_0000;
}

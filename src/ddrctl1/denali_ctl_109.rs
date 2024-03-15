#[doc = "Register `DENALI_CTL_109` reader"]
pub type R = crate::R<DenaliCtl109Spec>;
#[doc = "Register `DENALI_CTL_109` writer"]
pub type W = crate::W<DenaliCtl109Spec>;
#[doc = "Field `TDFI_INIT_START_F0` reader - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 0, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
pub type TdfiInitStartF0R = crate::FieldReader;
#[doc = "Field `TDFI_INIT_START_F0` writer - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 0, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
pub type TdfiInitStartF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_INIT_COMPLETE_F0` reader - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 0, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
pub type TdfiInitCompleteF0R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_INIT_COMPLETE_F0` writer - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 0, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
pub type TdfiInitCompleteF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TDFI_INIT_START_F1` reader - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 1, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
pub type TdfiInitStartF1R = crate::FieldReader;
#[doc = "Field `TDFI_INIT_START_F1` writer - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 1, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
pub type TdfiInitStartF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 0, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
    #[inline(always)]
    pub fn tdfi_init_start_f0(&self) -> TdfiInitStartF0R {
        TdfiInitStartF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 0, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
    #[inline(always)]
    pub fn tdfi_init_complete_f0(&self) -> TdfiInitCompleteF0R {
        TdfiInitCompleteF0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 1, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
    #[inline(always)]
    pub fn tdfi_init_start_f1(&self) -> TdfiInitStartF1R {
        TdfiInitStartF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 0, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_init_start_f0(&mut self) -> TdfiInitStartF0W<DenaliCtl109Spec> {
        TdfiInitStartF0W::new(self, 0)
    }
    #[doc = "Bits 8:23 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 0, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_init_complete_f0(&mut self) -> TdfiInitCompleteF0W<DenaliCtl109Spec> {
        TdfiInitCompleteF0W::new(self, 8)
    }
    #[doc = "Bits 24:31 - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 1, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_init_start_f1(&mut self) -> TdfiInitStartF1W<DenaliCtl109Spec> {
        TdfiInitStartF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_109::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_109::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl109Spec;
impl crate::RegisterSpec for DenaliCtl109Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_109::R`](R) reader structure"]
impl crate::Readable for DenaliCtl109Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_109::W`](W) writer structure"]
impl crate::Writable for DenaliCtl109Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_109 to value 0"]
impl crate::Resettable for DenaliCtl109Spec {
    const RESET_VALUE: u32 = 0;
}

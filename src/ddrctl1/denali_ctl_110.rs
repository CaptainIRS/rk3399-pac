#[doc = "Register `DENALI_CTL_110` reader"]
pub type R = crate::R<DenaliCtl110Spec>;
#[doc = "Register `DENALI_CTL_110` writer"]
pub type W = crate::W<DenaliCtl110Spec>;
#[doc = "Field `TDFI_INIT_COMPLETE_F1` reader - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 1, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
pub type TdfiInitCompleteF1R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_INIT_COMPLETE_F1` writer - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 1, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
pub type TdfiInitCompleteF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TDFI_INIT_START_F2` reader - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 2, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
pub type TdfiInitStartF2R = crate::FieldReader;
#[doc = "Field `TDFI_INIT_START_F2` writer - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 2, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
pub type TdfiInitStartF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 1, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
    #[inline(always)]
    pub fn tdfi_init_complete_f1(&self) -> TdfiInitCompleteF1R {
        TdfiInitCompleteF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 2, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
    #[inline(always)]
    pub fn tdfi_init_start_f2(&self) -> TdfiInitStartF2R {
        TdfiInitStartF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks) for frequency copy 1, the maximum cycles between a dfi_init_start de- assertion and a dfi_init_complete assertion from the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_init_complete_f1(&mut self) -> TdfiInitCompleteF1W<DenaliCtl110Spec> {
        TdfiInitCompleteF1W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Defines the DFI tINIT_START timing parameter (in DFI clocks) for frequency copy 2, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_init_start_f2(&mut self) -> TdfiInitStartF2W<DenaliCtl110Spec> {
        TdfiInitStartF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_110::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_110::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl110Spec;
impl crate::RegisterSpec for DenaliCtl110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_110::R`](R) reader structure"]
impl crate::Readable for DenaliCtl110Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_110::W`](W) writer structure"]
impl crate::Writable for DenaliCtl110Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_110 to value 0"]
impl crate::Resettable for DenaliCtl110Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DDR_DENALI_CTL_75` reader"]
pub type R = crate::R<DdrDenaliCtl75Spec>;
#[doc = "Register `DDR_DENALI_CTL_75` writer"]
pub type W = crate::W<DdrDenaliCtl75Spec>;
#[doc = "Field `UPD_CTRLUPD_NORM_THRESHOLD_F0` reader - DFI control update number of long counts until the normal priority request is asserted for frequency copy 0."]
pub type UpdCtrlupdNormThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_NORM_THRESHOLD_F0` writer - DFI control update number of long counts until the normal priority request is asserted for frequency copy 0."]
pub type UpdCtrlupdNormThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UPD_CTRLUPD_HIGH_THRESHOLD_F0` reader - DFI control update number of long counts until the high priority request is asserted for frequency copy 0."]
pub type UpdCtrlupdHighThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_HIGH_THRESHOLD_F0` writer - DFI control update number of long counts until the high priority request is asserted for frequency copy 0."]
pub type UpdCtrlupdHighThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the normal priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn upd_ctrlupd_norm_threshold_f0(&self) -> UpdCtrlupdNormThresholdF0R {
        UpdCtrlupdNormThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFI control update number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn upd_ctrlupd_high_threshold_f0(&self) -> UpdCtrlupdHighThresholdF0R {
        UpdCtrlupdHighThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the normal priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_norm_threshold_f0(
        &mut self,
    ) -> UpdCtrlupdNormThresholdF0W<DdrDenaliCtl75Spec> {
        UpdCtrlupdNormThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFI control update number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_high_threshold_f0(
        &mut self,
    ) -> UpdCtrlupdHighThresholdF0W<DdrDenaliCtl75Spec> {
        UpdCtrlupdHighThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_75::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_75::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl75Spec;
impl crate::RegisterSpec for DdrDenaliCtl75Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_75::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl75Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_75::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl75Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_75 to value 0"]
impl crate::Resettable for DdrDenaliCtl75Spec {
    const RESET_VALUE: u32 = 0;
}

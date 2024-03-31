#[doc = "Register `DENALI_CTL_79` reader"]
pub type R = crate::R<DenaliCtl79Spec>;
#[doc = "Register `DENALI_CTL_79` writer"]
pub type W = crate::W<DenaliCtl79Spec>;
#[doc = "Field `UPD_CTRLUPD_SW_PROMOTE_THRESHOLD_F1` reader - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 1."]
pub type UpdCtrlupdSwPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_SW_PROMOTE_THRESHOLD_F1` writer - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 1."]
pub type UpdCtrlupdSwPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UPD_PHYUPD_DFI_PROMOTE_THRESHOLD_F1` reader - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 1."]
pub type UpdPhyupdDfiPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `UPD_PHYUPD_DFI_PROMOTE_THRESHOLD_F1` writer - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 1."]
pub type UpdPhyupdDfiPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn upd_ctrlupd_sw_promote_threshold_f1(&self) -> UpdCtrlupdSwPromoteThresholdF1R {
        UpdCtrlupdSwPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn upd_phyupd_dfi_promote_threshold_f1(&self) -> UpdPhyupdDfiPromoteThresholdF1R {
        UpdPhyupdDfiPromoteThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_sw_promote_threshold_f1(
        &mut self,
    ) -> UpdCtrlupdSwPromoteThresholdF1W<DenaliCtl79Spec> {
        UpdCtrlupdSwPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn upd_phyupd_dfi_promote_threshold_f1(
        &mut self,
    ) -> UpdPhyupdDfiPromoteThresholdF1W<DenaliCtl79Spec> {
        UpdPhyupdDfiPromoteThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_79::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_79::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl79Spec;
impl crate::RegisterSpec for DenaliCtl79Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_79::R`](R) reader structure"]
impl crate::Readable for DenaliCtl79Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_79::W`](W) writer structure"]
impl crate::Writable for DenaliCtl79Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_79 to value 0"]
impl crate::Resettable for DenaliCtl79Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DENALI_CTL_77` reader"]
pub type R = crate::R<DenaliCtl77Spec>;
#[doc = "Register `DENALI_CTL_77` writer"]
pub type W = crate::W<DenaliCtl77Spec>;
#[doc = "Field `UPD_PHYUPD_DFI_PROMOTE_THRESHOLD_F0` reader - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type UpdPhyupdDfiPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `UPD_PHYUPD_DFI_PROMOTE_THRESHOLD_F0` writer - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type UpdPhyupdDfiPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UPD_CTRLUPD_NORM_THRESHOLD_F1` reader - DFI control update number of long counts until the normal priority request is asserted for frequency copy 1."]
pub type UpdCtrlupdNormThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_NORM_THRESHOLD_F1` writer - DFI control update number of long counts until the normal priority request is asserted for frequency copy 1."]
pub type UpdCtrlupdNormThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn upd_phyupd_dfi_promote_threshold_f0(&self) -> UpdPhyupdDfiPromoteThresholdF0R {
        UpdPhyupdDfiPromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFI control update number of long counts until the normal priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn upd_ctrlupd_norm_threshold_f1(&self) -> UpdCtrlupdNormThresholdF1R {
        UpdCtrlupdNormThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFI PHY update DFI promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn upd_phyupd_dfi_promote_threshold_f0(
        &mut self,
    ) -> UpdPhyupdDfiPromoteThresholdF0W<DenaliCtl77Spec> {
        UpdPhyupdDfiPromoteThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFI control update number of long counts until the normal priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_norm_threshold_f1(&mut self) -> UpdCtrlupdNormThresholdF1W<DenaliCtl77Spec> {
        UpdCtrlupdNormThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_77::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_77::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl77Spec;
impl crate::RegisterSpec for DenaliCtl77Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_77::R`](R) reader structure"]
impl crate::Readable for DenaliCtl77Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_77::W`](W) writer structure"]
impl crate::Writable for DenaliCtl77Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_77 to value 0"]
impl crate::Resettable for DenaliCtl77Spec {
    const RESET_VALUE: u32 = 0;
}

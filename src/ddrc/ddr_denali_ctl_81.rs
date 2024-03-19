#[doc = "Register `DDR_DENALI_CTL_81` reader"]
pub type R = crate::R<DdrDenaliCtl81Spec>;
#[doc = "Register `DDR_DENALI_CTL_81` writer"]
pub type W = crate::W<DdrDenaliCtl81Spec>;
#[doc = "Field `UPD_CTRLUPD_TIMEOUT_F2` reader - DFI control update number of long counts until the timeout is asserted for frequency copy 2."]
pub type UpdCtrlupdTimeoutF2R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_TIMEOUT_F2` writer - DFI control update number of long counts until the timeout is asserted for frequency copy 2."]
pub type UpdCtrlupdTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UPD_CTRLUPD_SW_PROMOTE_THRESHOLD_F2` reader - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type UpdCtrlupdSwPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_SW_PROMOTE_THRESHOLD_F2` writer - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type UpdCtrlupdSwPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn upd_ctrlupd_timeout_f2(&self) -> UpdCtrlupdTimeoutF2R {
        UpdCtrlupdTimeoutF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn upd_ctrlupd_sw_promote_threshold_f2(&self) -> UpdCtrlupdSwPromoteThresholdF2R {
        UpdCtrlupdSwPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_timeout_f2(&mut self) -> UpdCtrlupdTimeoutF2W<DdrDenaliCtl81Spec> {
        UpdCtrlupdTimeoutF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_sw_promote_threshold_f2(
        &mut self,
    ) -> UpdCtrlupdSwPromoteThresholdF2W<DdrDenaliCtl81Spec> {
        UpdCtrlupdSwPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_81::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_81::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl81Spec;
impl crate::RegisterSpec for DdrDenaliCtl81Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_81::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl81Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_81::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl81Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_81 to value 0"]
impl crate::Resettable for DdrDenaliCtl81Spec {
    const RESET_VALUE: u32 = 0;
}

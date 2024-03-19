#[doc = "Register `DDR_DENALI_CTL_76` reader"]
pub type R = crate::R<DdrDenaliCtl76Spec>;
#[doc = "Register `DDR_DENALI_CTL_76` writer"]
pub type W = crate::W<DdrDenaliCtl76Spec>;
#[doc = "Field `UPD_CTRLUPD_TIMEOUT_F0` reader - DFI control update number of long counts until the timeout is asserted for frequency copy 0."]
pub type UpdCtrlupdTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_TIMEOUT_F0` writer - DFI control update number of long counts until the timeout is asserted for frequency copy 0."]
pub type UpdCtrlupdTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UPD_CTRLUPD_SW_PROMOTE_THRESHOLD_F0` reader - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type UpdCtrlupdSwPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_SW_PROMOTE_THRESHOLD_F0` writer - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type UpdCtrlupdSwPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn upd_ctrlupd_timeout_f0(&self) -> UpdCtrlupdTimeoutF0R {
        UpdCtrlupdTimeoutF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn upd_ctrlupd_sw_promote_threshold_f0(&self) -> UpdCtrlupdSwPromoteThresholdF0R {
        UpdCtrlupdSwPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_timeout_f0(&mut self) -> UpdCtrlupdTimeoutF0W<DdrDenaliCtl76Spec> {
        UpdCtrlupdTimeoutF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFI control update SW promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_sw_promote_threshold_f0(
        &mut self,
    ) -> UpdCtrlupdSwPromoteThresholdF0W<DdrDenaliCtl76Spec> {
        UpdCtrlupdSwPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_76::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_76::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl76Spec;
impl crate::RegisterSpec for DdrDenaliCtl76Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_76::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl76Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_76::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl76Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_76 to value 0"]
impl crate::Resettable for DdrDenaliCtl76Spec {
    const RESET_VALUE: u32 = 0;
}

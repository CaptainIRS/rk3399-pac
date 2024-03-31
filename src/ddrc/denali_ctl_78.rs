#[doc = "Register `DENALI_CTL_78` reader"]
pub type R = crate::R<DenaliCtl78Spec>;
#[doc = "Register `DENALI_CTL_78` writer"]
pub type W = crate::W<DenaliCtl78Spec>;
#[doc = "Field `UPD_CTRLUPD_HIGH_THRESHOLD_F1` reader - DFI control update number of long counts until the high priority request is asserted for frequency copy 1."]
pub type UpdCtrlupdHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_HIGH_THRESHOLD_F1` writer - DFI control update number of long counts until the high priority request is asserted for frequency copy 1."]
pub type UpdCtrlupdHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UPD_CTRLUPD_TIMEOUT_F1` reader - DFI control update number of long counts until the timeout is asserted for frequency copy 1."]
pub type UpdCtrlupdTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_TIMEOUT_F1` writer - DFI control update number of long counts until the timeout is asserted for frequency copy 1."]
pub type UpdCtrlupdTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn upd_ctrlupd_high_threshold_f1(&self) -> UpdCtrlupdHighThresholdF1R {
        UpdCtrlupdHighThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFI control update number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn upd_ctrlupd_timeout_f1(&self) -> UpdCtrlupdTimeoutF1R {
        UpdCtrlupdTimeoutF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_high_threshold_f1(&mut self) -> UpdCtrlupdHighThresholdF1W<DenaliCtl78Spec> {
        UpdCtrlupdHighThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFI control update number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_timeout_f1(&mut self) -> UpdCtrlupdTimeoutF1W<DenaliCtl78Spec> {
        UpdCtrlupdTimeoutF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_78::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_78::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl78Spec;
impl crate::RegisterSpec for DenaliCtl78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_78::R`](R) reader structure"]
impl crate::Readable for DenaliCtl78Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_78::W`](W) writer structure"]
impl crate::Writable for DenaliCtl78Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_78 to value 0"]
impl crate::Resettable for DenaliCtl78Spec {
    const RESET_VALUE: u32 = 0;
}

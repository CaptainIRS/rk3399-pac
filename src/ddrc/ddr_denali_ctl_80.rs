#[doc = "Register `DDR_DENALI_CTL_80` reader"]
pub type R = crate::R<DdrDenaliCtl80Spec>;
#[doc = "Register `DDR_DENALI_CTL_80` writer"]
pub type W = crate::W<DdrDenaliCtl80Spec>;
#[doc = "Field `UPD_CTRLUPD_NORM_THRESHOLD_F2` reader - DFI control update number of long counts until the normal priority request is asserted for frequency copy 2."]
pub type UpdCtrlupdNormThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_NORM_THRESHOLD_F2` writer - DFI control update number of long counts until the normal priority request is asserted for frequency copy 2."]
pub type UpdCtrlupdNormThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UPD_CTRLUPD_HIGH_THRESHOLD_F2` reader - DFI control update number of long counts until the high priority request is asserted for frequency copy 2."]
pub type UpdCtrlupdHighThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_HIGH_THRESHOLD_F2` writer - DFI control update number of long counts until the high priority request is asserted for frequency copy 2."]
pub type UpdCtrlupdHighThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the normal priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn upd_ctrlupd_norm_threshold_f2(&self) -> UpdCtrlupdNormThresholdF2R {
        UpdCtrlupdNormThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFI control update number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn upd_ctrlupd_high_threshold_f2(&self) -> UpdCtrlupdHighThresholdF2R {
        UpdCtrlupdHighThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFI control update number of long counts until the normal priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_norm_threshold_f2(
        &mut self,
    ) -> UpdCtrlupdNormThresholdF2W<DdrDenaliCtl80Spec> {
        UpdCtrlupdNormThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFI control update number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_high_threshold_f2(
        &mut self,
    ) -> UpdCtrlupdHighThresholdF2W<DdrDenaliCtl80Spec> {
        UpdCtrlupdHighThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_80::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_80::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl80Spec;
impl crate::RegisterSpec for DdrDenaliCtl80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_80::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl80Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_80::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl80Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_80 to value 0"]
impl crate::Resettable for DdrDenaliCtl80Spec {
    const RESET_VALUE: u32 = 0;
}

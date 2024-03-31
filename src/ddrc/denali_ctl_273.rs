#[doc = "Register `DENALI_CTL_273` reader"]
pub type R = crate::R<DenaliCtl273Spec>;
#[doc = "Register `DENALI_CTL_273` writer"]
pub type W = crate::W<DenaliCtl273Spec>;
#[doc = "Field `CALVL_DFI_PROMOTE_THRESHOLD_F2` reader - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type CalvlDfiPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_DFI_PROMOTE_THRESHOLD_F2` writer - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type CalvlDfiPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DENALI0_ALLDATAUSED_ENABLE` reader - Enables use of the ALLDATAUSED signal for DENALI port 0. Set to 1 to enable."]
pub type Denali0AlldatausedEnableR = crate::BitReader;
#[doc = "Field `DENALI0_ALLDATAUSED_ENABLE` writer - Enables use of the ALLDATAUSED signal for DENALI port 0. Set to 1 to enable."]
pub type Denali0AlldatausedEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKE_STATUS` reader - Register access to cke_status signal."]
pub type CkeStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn calvl_dfi_promote_threshold_f2(&self) -> CalvlDfiPromoteThresholdF2R {
        CalvlDfiPromoteThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enables use of the ALLDATAUSED signal for DENALI port 0. Set to 1 to enable."]
    #[inline(always)]
    pub fn denali0_alldataused_enable(&self) -> Denali0AlldatausedEnableR {
        Denali0AlldatausedEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Register access to cke_status signal."]
    #[inline(always)]
    pub fn cke_status(&self) -> CkeStatusR {
        CkeStatusR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_dfi_promote_threshold_f2(
        &mut self,
    ) -> CalvlDfiPromoteThresholdF2W<DenaliCtl273Spec> {
        CalvlDfiPromoteThresholdF2W::new(self, 0)
    }
    #[doc = "Bit 16 - Enables use of the ALLDATAUSED signal for DENALI port 0. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn denali0_alldataused_enable(&mut self) -> Denali0AlldatausedEnableW<DenaliCtl273Spec> {
        Denali0AlldatausedEnableW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_273::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_273::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl273Spec;
impl crate::RegisterSpec for DenaliCtl273Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_273::R`](R) reader structure"]
impl crate::Readable for DenaliCtl273Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_273::W`](W) writer structure"]
impl crate::Writable for DenaliCtl273Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_273 to value 0"]
impl crate::Resettable for DenaliCtl273Spec {
    const RESET_VALUE: u32 = 0;
}

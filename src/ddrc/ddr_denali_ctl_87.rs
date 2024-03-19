#[doc = "Register `DDR_DENALI_CTL_87` reader"]
pub type R = crate::R<DdrDenaliCtl87Spec>;
#[doc = "Register `DDR_DENALI_CTL_87` writer"]
pub type W = crate::W<DdrDenaliCtl87Spec>;
#[doc = "Field `PHYMSTR_NO_AREF` reader - Disables refreshes during the PHY master interface sequence. Set to 1 to disable. Refreshes during reset are only supported for DFI 4.0 and this parameter may be set or cleared for DFI 4.0. For all other DFI versions, this parameter must be set to 1."]
pub type PhymstrNoArefR = crate::BitReader;
#[doc = "Field `PHYMSTR_NO_AREF` writer - Disables refreshes during the PHY master interface sequence. Set to 1 to disable. Refreshes during reset are only supported for DFI 4.0 and this parameter may be set or cleared for DFI 4.0. For all other DFI versions, this parameter must be set to 1."]
pub type PhymstrNoArefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYMSTR_ERROR_STATUS` reader - Identifies the source of any DFI PHY Master Interface errors. Value of 1 indicates a timing violation of the associated timing parameter. READ-ONLY"]
pub type PhymstrErrorStatusR = crate::FieldReader;
#[doc = "Field `MRR_TEMPCHK_NORM_THRESHOLD_F0` reader - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 0."]
pub type MrrTempchkNormThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `MRR_TEMPCHK_NORM_THRESHOLD_F0` writer - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 0."]
pub type MrrTempchkNormThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Disables refreshes during the PHY master interface sequence. Set to 1 to disable. Refreshes during reset are only supported for DFI 4.0 and this parameter may be set or cleared for DFI 4.0. For all other DFI versions, this parameter must be set to 1."]
    #[inline(always)]
    pub fn phymstr_no_aref(&self) -> PhymstrNoArefR {
        PhymstrNoArefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Identifies the source of any DFI PHY Master Interface errors. Value of 1 indicates a timing violation of the associated timing parameter. READ-ONLY"]
    #[inline(always)]
    pub fn phymstr_error_status(&self) -> PhymstrErrorStatusR {
        PhymstrErrorStatusR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn mrr_tempchk_norm_threshold_f0(&self) -> MrrTempchkNormThresholdF0R {
        MrrTempchkNormThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Disables refreshes during the PHY master interface sequence. Set to 1 to disable. Refreshes during reset are only supported for DFI 4.0 and this parameter may be set or cleared for DFI 4.0. For all other DFI versions, this parameter must be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn phymstr_no_aref(&mut self) -> PhymstrNoArefW<DdrDenaliCtl87Spec> {
        PhymstrNoArefW::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_norm_threshold_f0(
        &mut self,
    ) -> MrrTempchkNormThresholdF0W<DdrDenaliCtl87Spec> {
        MrrTempchkNormThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_87::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_87::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl87Spec;
impl crate::RegisterSpec for DdrDenaliCtl87Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_87::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl87Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_87::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl87Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_87 to value 0"]
impl crate::Resettable for DdrDenaliCtl87Spec {
    const RESET_VALUE: u32 = 0;
}

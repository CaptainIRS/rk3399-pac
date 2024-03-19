#[doc = "Register `DDR_DENALI_CTL_164` reader"]
pub type R = crate::R<DdrDenaliCtl164Spec>;
#[doc = "Register `DDR_DENALI_CTL_164` writer"]
pub type W = crate::W<DdrDenaliCtl164Spec>;
#[doc = "Field `LONG_COUNT_MASK` reader - Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 (1024 cycles), 0x10 (512 clocks), 0x18 (256 clocks), 0x1C (128 clocks), 0x1E (64 clocks) and 0x1F (32 clocks)."]
pub type LongCountMaskR = crate::FieldReader;
#[doc = "Field `LONG_COUNT_MASK` writer - Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 (1024 cycles), 0x10 (512 clocks), 0x18 (256 clocks), 0x1C (128 clocks), 0x1E (64 clocks) and 0x1F (32 clocks)."]
pub type LongCountMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AREF_NORM_THRESHOLD` reader - AREF number of pending refreshes until the normal priority request is asserted."]
pub type ArefNormThresholdR = crate::FieldReader;
#[doc = "Field `AREF_NORM_THRESHOLD` writer - AREF number of pending refreshes until the normal priority request is asserted."]
pub type ArefNormThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AREF_HIGH_THRESHOLD` reader - AREF number of pending refreshes until the high priority request is asserted."]
pub type ArefHighThresholdR = crate::FieldReader;
#[doc = "Field `AREF_HIGH_THRESHOLD` writer - AREF number of pending refreshes until the high priority request is asserted."]
pub type ArefHighThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AREF_MAX_DEFICIT` reader - AREF number of pending refreshes until the maximum number of refreshes has been exceeded."]
pub type ArefMaxDeficitR = crate::FieldReader;
#[doc = "Field `AREF_MAX_DEFICIT` writer - AREF number of pending refreshes until the maximum number of refreshes has been exceeded."]
pub type ArefMaxDeficitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 (1024 cycles), 0x10 (512 clocks), 0x18 (256 clocks), 0x1C (128 clocks), 0x1E (64 clocks) and 0x1F (32 clocks)."]
    #[inline(always)]
    pub fn long_count_mask(&self) -> LongCountMaskR {
        LongCountMaskR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - AREF number of pending refreshes until the normal priority request is asserted."]
    #[inline(always)]
    pub fn aref_norm_threshold(&self) -> ArefNormThresholdR {
        ArefNormThresholdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - AREF number of pending refreshes until the high priority request is asserted."]
    #[inline(always)]
    pub fn aref_high_threshold(&self) -> ArefHighThresholdR {
        ArefHighThresholdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - AREF number of pending refreshes until the maximum number of refreshes has been exceeded."]
    #[inline(always)]
    pub fn aref_max_deficit(&self) -> ArefMaxDeficitR {
        ArefMaxDeficitR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 (1024 cycles), 0x10 (512 clocks), 0x18 (256 clocks), 0x1C (128 clocks), 0x1E (64 clocks) and 0x1F (32 clocks)."]
    #[inline(always)]
    #[must_use]
    pub fn long_count_mask(&mut self) -> LongCountMaskW<DdrDenaliCtl164Spec> {
        LongCountMaskW::new(self, 0)
    }
    #[doc = "Bits 8:12 - AREF number of pending refreshes until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn aref_norm_threshold(&mut self) -> ArefNormThresholdW<DdrDenaliCtl164Spec> {
        ArefNormThresholdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - AREF number of pending refreshes until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn aref_high_threshold(&mut self) -> ArefHighThresholdW<DdrDenaliCtl164Spec> {
        ArefHighThresholdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - AREF number of pending refreshes until the maximum number of refreshes has been exceeded."]
    #[inline(always)]
    #[must_use]
    pub fn aref_max_deficit(&mut self) -> ArefMaxDeficitW<DdrDenaliCtl164Spec> {
        ArefMaxDeficitW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_164::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_164::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl164Spec;
impl crate::RegisterSpec for DdrDenaliCtl164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_164::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl164Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_164::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl164Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_164 to value 0"]
impl crate::Resettable for DdrDenaliCtl164Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DENALI_CTL_24` reader"]
pub type R = crate::R<DenaliCtl24Spec>;
#[doc = "Register `DENALI_CTL_24` writer"]
pub type W = crate::W<DenaliCtl24Spec>;
#[doc = "Field `ADDITIVE_LAT_F0` reader - DRAM additive latency value for frequency copy 0 in cycles."]
pub type AdditiveLatF0R = crate::FieldReader;
#[doc = "Field `ADDITIVE_LAT_F0` writer - DRAM additive latency value for frequency copy 0 in cycles."]
pub type AdditiveLatF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CASLAT_LIN_F1` reader - Sets latency from read command send to data receive from/to controller for frequency copy 1. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type CaslatLinF1R = crate::FieldReader;
#[doc = "Field `CASLAT_LIN_F1` writer - Sets latency from read command send to data receive from/to controller for frequency copy 1. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type CaslatLinF1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRLAT_F1` reader - DRAM WRLAT value for frequency copy 1 in cycles."]
pub type WrlatF1R = crate::FieldReader;
#[doc = "Field `WRLAT_F1` writer - DRAM WRLAT value for frequency copy 1 in cycles."]
pub type WrlatF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADDITIVE_LAT_F1` reader - DRAM additive latency value for frequency copy 1 in cycles."]
pub type AdditiveLatF1R = crate::FieldReader;
#[doc = "Field `ADDITIVE_LAT_F1` writer - DRAM additive latency value for frequency copy 1 in cycles."]
pub type AdditiveLatF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - DRAM additive latency value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn additive_lat_f0(&self) -> AdditiveLatF0R {
        AdditiveLatF0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - Sets latency from read command send to data receive from/to controller for frequency copy 1. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    pub fn caslat_lin_f1(&self) -> CaslatLinF1R {
        CaslatLinF1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - DRAM WRLAT value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn wrlat_f1(&self) -> WrlatF1R {
        WrlatF1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:29 - DRAM additive latency value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn additive_lat_f1(&self) -> AdditiveLatF1R {
        AdditiveLatF1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DRAM additive latency value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn additive_lat_f0(&mut self) -> AdditiveLatF0W<DenaliCtl24Spec> {
        AdditiveLatF0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Sets latency from read command send to data receive from/to controller for frequency copy 1. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    #[must_use]
    pub fn caslat_lin_f1(&mut self) -> CaslatLinF1W<DenaliCtl24Spec> {
        CaslatLinF1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - DRAM WRLAT value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn wrlat_f1(&mut self) -> WrlatF1W<DenaliCtl24Spec> {
        WrlatF1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - DRAM additive latency value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn additive_lat_f1(&mut self) -> AdditiveLatF1W<DenaliCtl24Spec> {
        AdditiveLatF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl24Spec;
impl crate::RegisterSpec for DenaliCtl24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_24::R`](R) reader structure"]
impl crate::Readable for DenaliCtl24Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_24::W`](W) writer structure"]
impl crate::Writable for DenaliCtl24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_24 to value 0"]
impl crate::Resettable for DenaliCtl24Spec {
    const RESET_VALUE: u32 = 0;
}

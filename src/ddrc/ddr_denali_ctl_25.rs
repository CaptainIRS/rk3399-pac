#[doc = "Register `DDR_DENALI_CTL_25` reader"]
pub type R = crate::R<DdrDenaliCtl25Spec>;
#[doc = "Register `DDR_DENALI_CTL_25` writer"]
pub type W = crate::W<DdrDenaliCtl25Spec>;
#[doc = "Field `CASLAT_LIN_F2` reader - Sets latency from read command send to data receive from/to controller for frequency copy 2. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type CaslatLinF2R = crate::FieldReader;
#[doc = "Field `CASLAT_LIN_F2` writer - Sets latency from read command send to data receive from/to controller for frequency copy 2. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type CaslatLinF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRLAT_F2` reader - DRAM WRLAT value for frequency copy 2 in cycles."]
pub type WrlatF2R = crate::FieldReader;
#[doc = "Field `WRLAT_F2` writer - DRAM WRLAT value for frequency copy 2 in cycles."]
pub type WrlatF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADDITIVE_LAT_F2` reader - DRAM additive latency value for frequency copy 2 in cycles."]
pub type AdditiveLatF2R = crate::FieldReader;
#[doc = "Field `ADDITIVE_LAT_F2` writer - DRAM additive latency value for frequency copy 2 in cycles."]
pub type AdditiveLatF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TBST_INT_INTERVAL` reader - DRAM burst interrupt interval value in cycles."]
pub type TbstIntIntervalR = crate::FieldReader;
#[doc = "Field `TBST_INT_INTERVAL` writer - DRAM burst interrupt interval value in cycles."]
pub type TbstIntIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - Sets latency from read command send to data receive from/to controller for frequency copy 2. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    pub fn caslat_lin_f2(&self) -> CaslatLinF2R {
        CaslatLinF2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:12 - DRAM WRLAT value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn wrlat_f2(&self) -> WrlatF2R {
        WrlatF2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - DRAM additive latency value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn additive_lat_f2(&self) -> AdditiveLatF2R {
        AdditiveLatF2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - DRAM burst interrupt interval value in cycles."]
    #[inline(always)]
    pub fn tbst_int_interval(&self) -> TbstIntIntervalR {
        TbstIntIntervalR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets latency from read command send to data receive from/to controller for frequency copy 2. Bit (0) is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    #[must_use]
    pub fn caslat_lin_f2(&mut self) -> CaslatLinF2W<DdrDenaliCtl25Spec> {
        CaslatLinF2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - DRAM WRLAT value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn wrlat_f2(&mut self) -> WrlatF2W<DdrDenaliCtl25Spec> {
        WrlatF2W::new(self, 8)
    }
    #[doc = "Bits 16:21 - DRAM additive latency value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn additive_lat_f2(&mut self) -> AdditiveLatF2W<DdrDenaliCtl25Spec> {
        AdditiveLatF2W::new(self, 16)
    }
    #[doc = "Bits 24:26 - DRAM burst interrupt interval value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tbst_int_interval(&mut self) -> TbstIntIntervalW<DdrDenaliCtl25Spec> {
        TbstIntIntervalW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl25Spec;
impl crate::RegisterSpec for DdrDenaliCtl25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_25::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl25Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_25::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_25 to value 0"]
impl crate::Resettable for DdrDenaliCtl25Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DDR_PI_REG_44` reader"]
pub type R = crate::R<DdrPiReg44Spec>;
#[doc = "Register `DDR_PI_REG_44` writer"]
pub type W = crate::W<DdrPiReg44Spec>;
#[doc = "Field `PI_ADDITIVE_LAT_F1` reader - Indicates DRAM additive latency value in cycles. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiAdditiveLatF1R = crate::FieldReader;
#[doc = "Field `PI_ADDITIVE_LAT_F1` writer - Indicates DRAM additive latency value in cycles. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiAdditiveLatF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_CASLAT_LIN_F1` reader - Sets latency from read command send to data receive from/to\n\ncontroller. Bit0 is half-cycle increment and the upper bits define\n\nmemory CAS latency for the controller. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiCaslatLinF1R = crate::FieldReader;
#[doc = "Field `PI_CASLAT_LIN_F1` writer - Sets latency from read command send to data receive from/to\n\ncontroller. Bit0 is half-cycle increment and the upper bits define\n\nmemory CAS latency for the controller. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiCaslatLinF1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_WRLAT_F2` reader - Indicates DRAM WRLAT value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiWrlatF2R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_F2` writer - Indicates DRAM WRLAT value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiWrlatF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_ADDITIVE_LAT_F2` reader - Indicates DRAM additive latency value in cycles. The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiAdditiveLatF2R = crate::FieldReader;
#[doc = "Field `PI_ADDITIVE_LAT_F2` writer - Indicates DRAM additive latency value in cycles. The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiAdditiveLatF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Indicates DRAM additive latency value in cycles. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_additive_lat_f1(&self) -> PiAdditiveLatF1R {
        PiAdditiveLatF1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - Sets latency from read command send to data receive from/to\n\ncontroller. Bit0 is half-cycle increment and the upper bits define\n\nmemory CAS latency for the controller. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_caslat_lin_f1(&self) -> PiCaslatLinF1R {
        PiCaslatLinF1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - Indicates DRAM WRLAT value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_wrlat_f2(&self) -> PiWrlatF2R {
        PiWrlatF2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:29 - Indicates DRAM additive latency value in cycles. The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_additive_lat_f2(&self) -> PiAdditiveLatF2R {
        PiAdditiveLatF2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Indicates DRAM additive latency value in cycles. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_additive_lat_f1(&mut self) -> PiAdditiveLatF1W<DdrPiReg44Spec> {
        PiAdditiveLatF1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Sets latency from read command send to data receive from/to\n\ncontroller. Bit0 is half-cycle increment and the upper bits define\n\nmemory CAS latency for the controller. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_caslat_lin_f1(&mut self) -> PiCaslatLinF1W<DdrPiReg44Spec> {
        PiCaslatLinF1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Indicates DRAM WRLAT value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_f2(&mut self) -> PiWrlatF2W<DdrPiReg44Spec> {
        PiWrlatF2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Indicates DRAM additive latency value in cycles. The suffix '_f2' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_additive_lat_f2(&mut self) -> PiAdditiveLatF2W<DdrPiReg44Spec> {
        PiAdditiveLatF2W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg44Spec;
impl crate::RegisterSpec for DdrPiReg44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_44::R`](R) reader structure"]
impl crate::Readable for DdrPiReg44Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_44::W`](W) writer structure"]
impl crate::Writable for DdrPiReg44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_44 to value 0"]
impl crate::Resettable for DdrPiReg44Spec {
    const RESET_VALUE: u32 = 0;
}

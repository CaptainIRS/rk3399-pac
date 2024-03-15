#[doc = "Register `PI_REG_43` reader"]
pub type R = crate::R<PiReg43Spec>;
#[doc = "Register `PI_REG_43` writer"]
pub type W = crate::W<PiReg43Spec>;
#[doc = "Field `PI_WRLAT_F0` reader - Indicates DRAM WRLAT value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatF0R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_F0` writer - Indicates DRAM WRLAT value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_ADDITIVE_LAT_F0` reader - Indicates DRAM additive latency value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiAdditiveLatF0R = crate::FieldReader;
#[doc = "Field `PI_ADDITIVE_LAT_F0` writer - Indicates DRAM additive latency value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiAdditiveLatF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_CASLAT_LIN_F0` reader - Sets latency from read command send to data receive from/to controller. Bit0 is half-cycle increment and the upper bits define memory CAS latency for the controller. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiCaslatLinF0R = crate::FieldReader;
#[doc = "Field `PI_CASLAT_LIN_F0` writer - Sets latency from read command send to data receive from/to controller. Bit0 is half-cycle increment and the upper bits define memory CAS latency for the controller. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiCaslatLinF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_WRLAT_F1` reader - Indicates DRAM WRLAT value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatF1R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_F1` writer - Indicates DRAM WRLAT value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Indicates DRAM WRLAT value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_wrlat_f0(&self) -> PiWrlatF0R {
        PiWrlatF0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Indicates DRAM additive latency value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_additive_lat_f0(&self) -> PiAdditiveLatF0R {
        PiAdditiveLatF0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Sets latency from read command send to data receive from/to controller. Bit0 is half-cycle increment and the upper bits define memory CAS latency for the controller. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_caslat_lin_f0(&self) -> PiCaslatLinF0R {
        PiCaslatLinF0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:28 - Indicates DRAM WRLAT value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_wrlat_f1(&self) -> PiWrlatF1R {
        PiWrlatF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Indicates DRAM WRLAT value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_f0(&mut self) -> PiWrlatF0W<PiReg43Spec> {
        PiWrlatF0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Indicates DRAM additive latency value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_additive_lat_f0(&mut self) -> PiAdditiveLatF0W<PiReg43Spec> {
        PiAdditiveLatF0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Sets latency from read command send to data receive from/to controller. Bit0 is half-cycle increment and the upper bits define memory CAS latency for the controller. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_caslat_lin_f0(&mut self) -> PiCaslatLinF0W<PiReg43Spec> {
        PiCaslatLinF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Indicates DRAM WRLAT value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_f1(&mut self) -> PiWrlatF1W<PiReg43Spec> {
        PiWrlatF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg43Spec;
impl crate::RegisterSpec for PiReg43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_43::R`](R) reader structure"]
impl crate::Readable for PiReg43Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_43::W`](W) writer structure"]
impl crate::Writable for PiReg43Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_43 to value 0"]
impl crate::Resettable for PiReg43Spec {
    const RESET_VALUE: u32 = 0;
}

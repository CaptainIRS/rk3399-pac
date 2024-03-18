#[doc = "Register `PI_REG_166` reader"]
pub type R = crate::R<PiReg166Spec>;
#[doc = "Register `PI_REG_166` writer"]
pub type W = crate::W<PiReg166Spec>;
#[doc = "Field `PI_TWTR_F2` reader - Indicates DRAM TWTR value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTwtrF2R = crate::FieldReader;
#[doc = "Field `PI_TWTR_F2` writer - Indicates DRAM TWTR value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTwtrF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TWR_F2` reader - Indicates DRAM TWR value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTwrF2R = crate::FieldReader;
#[doc = "Field `PI_TWR_F2` writer - Indicates DRAM TWR value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTwrF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Indicates DRAM TWTR value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_twtr_f2(&self) -> PiTwtrF2R {
        PiTwtrF2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Indicates DRAM TWR value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_twr_f2(&self) -> PiTwrF2R {
        PiTwrF2R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Indicates DRAM TWTR value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twtr_f2(&mut self) -> PiTwtrF2W<PiReg166Spec> {
        PiTwtrF2W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Indicates DRAM TWR value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twr_f2(&mut self) -> PiTwrF2W<PiReg166Spec> {
        PiTwrF2W::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 166\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_166::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_166::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg166Spec;
impl crate::RegisterSpec for PiReg166Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_166::R`](R) reader structure"]
impl crate::Readable for PiReg166Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_166::W`](W) writer structure"]
impl crate::Writable for PiReg166Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_166 to value 0"]
impl crate::Resettable for PiReg166Spec {
    const RESET_VALUE: u32 = 0;
}

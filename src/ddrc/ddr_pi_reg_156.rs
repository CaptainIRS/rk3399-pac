#[doc = "Register `DDR_PI_REG_156` reader"]
pub type R = crate::R<DdrPiReg156Spec>;
#[doc = "Register `DDR_PI_REG_156` writer"]
pub type W = crate::W<DdrPiReg156Spec>;
#[doc = "Field `PI_TFC_F0` reader - Indicates the delay in PHY clock cycles from setting MR13.OP7 to\n\nany valid command. The suffix '_f0' of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTfcF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TFC_F0` writer - Indicates the delay in PHY clock cycles from setting MR13.OP7 to\n\nany valid command. The suffix '_f0' of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTfcF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TFC_F1` reader - Indicates the delay in PHY clock cycles from setting MR13.OP7 to\n\nany valid command. The suffix '_f1' of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTfcF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TFC_F1` writer - Indicates the delay in PHY clock cycles from setting MR13.OP7 to\n\nany valid command. The suffix '_f1' of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTfcF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Indicates the delay in PHY clock cycles from setting MR13.OP7 to\n\nany valid command. The suffix '_f0' of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tfc_f0(&self) -> PiTfcF0R {
        PiTfcF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Indicates the delay in PHY clock cycles from setting MR13.OP7 to\n\nany valid command. The suffix '_f1' of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tfc_f1(&self) -> PiTfcF1R {
        PiTfcF1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Indicates the delay in PHY clock cycles from setting MR13.OP7 to\n\nany valid command. The suffix '_f0' of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tfc_f0(&mut self) -> PiTfcF0W<DdrPiReg156Spec> {
        PiTfcF0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Indicates the delay in PHY clock cycles from setting MR13.OP7 to\n\nany valid command. The suffix '_f1' of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tfc_f1(&mut self) -> PiTfcF1W<DdrPiReg156Spec> {
        PiTfcF1W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_156::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_156::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg156Spec;
impl crate::RegisterSpec for DdrPiReg156Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_156::R`](R) reader structure"]
impl crate::Readable for DdrPiReg156Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_156::W`](W) writer structure"]
impl crate::Writable for DdrPiReg156Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_156 to value 0"]
impl crate::Resettable for DdrPiReg156Spec {
    const RESET_VALUE: u32 = 0;
}

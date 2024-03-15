#[doc = "Register `PI_REG_136` reader"]
pub type R = crate::R<PiReg136Spec>;
#[doc = "Register `PI_REG_136` writer"]
pub type W = crate::W<PiReg136Spec>;
#[doc = "Field `PI_MR2_DATA_F1_1` reader - Indicates data to program into memory mode register 2 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr2DataF1_1R = crate::FieldReader<u16>;
#[doc = "Field `PI_MR2_DATA_F1_1` writer - Indicates data to program into memory mode register 2 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr2DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_MR3_DATA_F1_1` reader - Indicates data to program into memory mode register 3 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr3DataF1_1R = crate::FieldReader<u16>;
#[doc = "Field `PI_MR3_DATA_F1_1` writer - Indicates data to program into memory mode register 3 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr3DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates data to program into memory mode register 2 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr2_data_f1_1(&self) -> PiMr2DataF1_1R {
        PiMr2DataF1_1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Indicates data to program into memory mode register 3 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr3_data_f1_1(&self) -> PiMr3DataF1_1R {
        PiMr3DataF1_1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates data to program into memory mode register 2 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr2_data_f1_1(&mut self) -> PiMr2DataF1_1W<PiReg136Spec> {
        PiMr2DataF1_1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Indicates data to program into memory mode register 3 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr3_data_f1_1(&mut self) -> PiMr3DataF1_1W<PiReg136Spec> {
        PiMr3DataF1_1W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_136::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_136::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg136Spec;
impl crate::RegisterSpec for PiReg136Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_136::R`](R) reader structure"]
impl crate::Readable for PiReg136Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_136::W`](W) writer structure"]
impl crate::Writable for PiReg136Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_136 to value 0"]
impl crate::Resettable for PiReg136Spec {
    const RESET_VALUE: u32 = 0;
}

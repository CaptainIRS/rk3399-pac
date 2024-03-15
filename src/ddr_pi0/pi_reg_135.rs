#[doc = "Register `PI_REG_135` reader"]
pub type R = crate::R<PiReg135Spec>;
#[doc = "Register `PI_REG_135` writer"]
pub type W = crate::W<PiReg135Spec>;
#[doc = "Field `PI_MR14_DATA_F0_1` reader - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr14DataF0_1R = crate::FieldReader;
#[doc = "Field `PI_MR14_DATA_F0_1` writer - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr14DataF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR1_DATA_F1_1` reader - Indicates data to program into memory mode register 1 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr1DataF1_1R = crate::FieldReader<u16>;
#[doc = "Field `PI_MR1_DATA_F1_1` writer - Indicates data to program into memory mode register 1 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr1DataF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr14_data_f0_1(&self) -> PiMr14DataF0_1R {
        PiMr14DataF0_1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Indicates data to program into memory mode register 1 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr1_data_f1_1(&self) -> PiMr1DataF1_1R {
        PiMr1DataF1_1R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr14_data_f0_1(&mut self) -> PiMr14DataF0_1W<PiReg135Spec> {
        PiMr14DataF0_1W::new(self, 0)
    }
    #[doc = "Bits 8:23 - Indicates data to program into memory mode register 1 for chip select 1. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr1_data_f1_1(&mut self) -> PiMr1DataF1_1W<PiReg135Spec> {
        PiMr1DataF1_1W::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 135\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_135::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_135::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg135Spec;
impl crate::RegisterSpec for PiReg135Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_135::R`](R) reader structure"]
impl crate::Readable for PiReg135Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_135::W`](W) writer structure"]
impl crate::Writable for PiReg135Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_135 to value 0"]
impl crate::Resettable for PiReg135Spec {
    const RESET_VALUE: u32 = 0;
}

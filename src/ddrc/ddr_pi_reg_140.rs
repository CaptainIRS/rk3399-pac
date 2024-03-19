#[doc = "Register `DDR_PI_REG_140` reader"]
pub type R = crate::R<DdrPiReg140Spec>;
#[doc = "Register `DDR_PI_REG_140` writer"]
pub type W = crate::W<DdrPiReg140Spec>;
#[doc = "Field `PI_MR14_DATA_F2_1` reader - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr14DataF2_1R = crate::FieldReader;
#[doc = "Field `PI_MR14_DATA_F2_1` writer - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr14DataF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR13_DATA_1` reader - Indicates data to program into memory mode register 13 for chip select 0."]
pub type PiMr13Data1R = crate::FieldReader;
#[doc = "Field `PI_MR13_DATA_1` writer - Indicates data to program into memory mode register 13 for chip select 0."]
pub type PiMr13Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr14_data_f2_1(&self) -> PiMr14DataF2_1R {
        PiMr14DataF2_1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates data to program into memory mode register 13 for chip select 0."]
    #[inline(always)]
    pub fn pi_mr13_data_1(&self) -> PiMr13Data1R {
        PiMr13Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 14 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr14_data_f2_1(&mut self) -> PiMr14DataF2_1W<DdrPiReg140Spec> {
        PiMr14DataF2_1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates data to program into memory mode register 13 for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr13_data_1(&mut self) -> PiMr13Data1W<DdrPiReg140Spec> {
        PiMr13Data1W::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_140::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_140::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg140Spec;
impl crate::RegisterSpec for DdrPiReg140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_140::R`](R) reader structure"]
impl crate::Readable for DdrPiReg140Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_140::W`](W) writer structure"]
impl crate::Writable for DdrPiReg140Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_140 to value 0"]
impl crate::Resettable for DdrPiReg140Spec {
    const RESET_VALUE: u32 = 0;
}

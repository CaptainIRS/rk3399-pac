#[doc = "Register `PI_REG_127` reader"]
pub type R = crate::R<PiReg127Spec>;
#[doc = "Register `PI_REG_127` writer"]
pub type W = crate::W<PiReg127Spec>;
#[doc = "Field `PI_MR11_DATA_F0_0` reader - Indicates data to program into memory mode register 11 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr11DataF0_0R = crate::FieldReader;
#[doc = "Field `PI_MR11_DATA_F0_0` writer - Indicates data to program into memory mode register 11 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr11DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR12_DATA_F0_0` reader - Indicates data to program into memory mode register 12 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr12DataF0_0R = crate::FieldReader;
#[doc = "Field `PI_MR12_DATA_F0_0` writer - Indicates data to program into memory mode register 12 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr12DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR14_DATA_F0_0` reader - Indicates data to program into memory mode register 14 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr14DataF0_0R = crate::FieldReader;
#[doc = "Field `PI_MR14_DATA_F0_0` writer - Indicates data to program into memory mode register 14 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr14DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 11 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr11_data_f0_0(&self) -> PiMr11DataF0_0R {
        PiMr11DataF0_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates data to program into memory mode register 12 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr12_data_f0_0(&self) -> PiMr12DataF0_0R {
        PiMr12DataF0_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 14 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr14_data_f0_0(&self) -> PiMr14DataF0_0R {
        PiMr14DataF0_0R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 11 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr11_data_f0_0(&mut self) -> PiMr11DataF0_0W<PiReg127Spec> {
        PiMr11DataF0_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates data to program into memory mode register 12 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr12_data_f0_0(&mut self) -> PiMr12DataF0_0W<PiReg127Spec> {
        PiMr12DataF0_0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 14 for chip select 0. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr14_data_f0_0(&mut self) -> PiMr14DataF0_0W<PiReg127Spec> {
        PiMr14DataF0_0W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_127::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_127::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg127Spec;
impl crate::RegisterSpec for PiReg127Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_127::R`](R) reader structure"]
impl crate::Readable for PiReg127Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_127::W`](W) writer structure"]
impl crate::Writable for PiReg127Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_127 to value 0"]
impl crate::Resettable for PiReg127Spec {
    const RESET_VALUE: u32 = 0;
}

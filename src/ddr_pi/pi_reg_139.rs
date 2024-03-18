#[doc = "Register `PI_REG_139` reader"]
pub type R = crate::R<PiReg139Spec>;
#[doc = "Register `PI_REG_139` writer"]
pub type W = crate::W<PiReg139Spec>;
#[doc = "Field `PI_MR3_DATA_F2_1` reader - Indicates data to program into memory mode register 3 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr3DataF2_1R = crate::FieldReader<u16>;
#[doc = "Field `PI_MR3_DATA_F2_1` writer - Indicates data to program into memory mode register 3 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr3DataF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_MR11_DATA_F2_1` reader - Indicates data to program into memory mode register 11 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr11DataF2_1R = crate::FieldReader;
#[doc = "Field `PI_MR11_DATA_F2_1` writer - Indicates data to program into memory mode register 11 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr11DataF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR12_DATA_F2_1` reader - Indicates data to program into memory mode register 12 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr12DataF2_1R = crate::FieldReader;
#[doc = "Field `PI_MR12_DATA_F2_1` writer - Indicates data to program into memory mode register 12 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiMr12DataF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Indicates data to program into memory mode register 3 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr3_data_f2_1(&self) -> PiMr3DataF2_1R {
        PiMr3DataF2_1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 11 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr11_data_f2_1(&self) -> PiMr11DataF2_1R {
        PiMr11DataF2_1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates data to program into memory mode register 12 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_mr12_data_f2_1(&self) -> PiMr12DataF2_1R {
        PiMr12DataF2_1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates data to program into memory mode register 3 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr3_data_f2_1(&mut self) -> PiMr3DataF2_1W<PiReg139Spec> {
        PiMr3DataF2_1W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 11 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr11_data_f2_1(&mut self) -> PiMr11DataF2_1W<PiReg139Spec> {
        PiMr11DataF2_1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates data to program into memory mode register 12 for chip select 1. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr12_data_f2_1(&mut self) -> PiMr12DataF2_1W<PiReg139Spec> {
        PiMr12DataF2_1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 139\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_139::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_139::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg139Spec;
impl crate::RegisterSpec for PiReg139Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_139::R`](R) reader structure"]
impl crate::Readable for PiReg139Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_139::W`](W) writer structure"]
impl crate::Writable for PiReg139Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_139 to value 0"]
impl crate::Resettable for PiReg139Spec {
    const RESET_VALUE: u32 = 0;
}

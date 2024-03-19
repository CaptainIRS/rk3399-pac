#[doc = "Register `DDR_PI_REG_132` reader"]
pub type R = crate::R<DdrPiReg132Spec>;
#[doc = "Register `DDR_PI_REG_132` writer"]
pub type W = crate::W<DdrPiReg132Spec>;
#[doc = "Field `PI_MR11_DATA_F2_0` reader - Indicates data to program into memory mode register 11 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr11DataF2_0R = crate::FieldReader;
#[doc = "Field `PI_MR11_DATA_F2_0` writer - Indicates data to program into memory mode register 11 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr11DataF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR12_DATA_F2_0` reader - Indicates data to program into memory mode register 12 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr12DataF2_0R = crate::FieldReader;
#[doc = "Field `PI_MR12_DATA_F2_0` writer - Indicates data to program into memory mode register 12 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr12DataF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR14_DATA_F2_0` reader - Indicates data to program into memory mode register 14 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr14DataF2_0R = crate::FieldReader;
#[doc = "Field `PI_MR14_DATA_F2_0` writer - Indicates data to program into memory mode register 14 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr14DataF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR13_DATA_0` reader - Indicates data to program into memory mode register 13 for chip\n\nselect 0."]
pub type PiMr13Data0R = crate::FieldReader;
#[doc = "Field `PI_MR13_DATA_0` writer - Indicates data to program into memory mode register 13 for chip\n\nselect 0."]
pub type PiMr13Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 11 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    pub fn pi_mr11_data_f2_0(&self) -> PiMr11DataF2_0R {
        PiMr11DataF2_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates data to program into memory mode register 12 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    pub fn pi_mr12_data_f2_0(&self) -> PiMr12DataF2_0R {
        PiMr12DataF2_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 14 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    pub fn pi_mr14_data_f2_0(&self) -> PiMr14DataF2_0R {
        PiMr14DataF2_0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates data to program into memory mode register 13 for chip\n\nselect 0."]
    #[inline(always)]
    pub fn pi_mr13_data_0(&self) -> PiMr13Data0R {
        PiMr13Data0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates data to program into memory mode register 11 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr11_data_f2_0(&mut self) -> PiMr11DataF2_0W<DdrPiReg132Spec> {
        PiMr11DataF2_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates data to program into memory mode register 12 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr12_data_f2_0(&mut self) -> PiMr12DataF2_0W<DdrPiReg132Spec> {
        PiMr12DataF2_0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 14 for chip\n\nselect 0. The suffix \"_f2\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr14_data_f2_0(&mut self) -> PiMr14DataF2_0W<DdrPiReg132Spec> {
        PiMr14DataF2_0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates data to program into memory mode register 13 for chip\n\nselect 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr13_data_0(&mut self) -> PiMr13Data0W<DdrPiReg132Spec> {
        PiMr13Data0W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_132::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_132::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg132Spec;
impl crate::RegisterSpec for DdrPiReg132Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_132::R`](R) reader structure"]
impl crate::Readable for DdrPiReg132Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_132::W`](W) writer structure"]
impl crate::Writable for DdrPiReg132Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_132 to value 0"]
impl crate::Resettable for DdrPiReg132Spec {
    const RESET_VALUE: u32 = 0;
}

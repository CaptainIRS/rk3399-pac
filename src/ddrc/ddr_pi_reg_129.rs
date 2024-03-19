#[doc = "Register `DDR_PI_REG_129` reader"]
pub type R = crate::R<DdrPiReg129Spec>;
#[doc = "Register `DDR_PI_REG_129` writer"]
pub type W = crate::W<DdrPiReg129Spec>;
#[doc = "Field `PI_MR3_DATA_F1_0` reader - Indicates data to program into memory mode register 3 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr3DataF1_0R = crate::FieldReader<u16>;
#[doc = "Field `PI_MR3_DATA_F1_0` writer - Indicates data to program into memory mode register 3 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr3DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_MR11_DATA_F1_0` reader - Indicates data to program into memory mode register 11 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr11DataF1_0R = crate::FieldReader;
#[doc = "Field `PI_MR11_DATA_F1_0` writer - Indicates data to program into memory mode register 11 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr11DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MR12_DATA_F1_0` reader - Indicates data to program into memory mode register 12 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr12DataF1_0R = crate::FieldReader;
#[doc = "Field `PI_MR12_DATA_F1_0` writer - Indicates data to program into memory mode register 12 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr12DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Indicates data to program into memory mode register 3 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    pub fn pi_mr3_data_f1_0(&self) -> PiMr3DataF1_0R {
        PiMr3DataF1_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 11 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    pub fn pi_mr11_data_f1_0(&self) -> PiMr11DataF1_0R {
        PiMr11DataF1_0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates data to program into memory mode register 12 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    pub fn pi_mr12_data_f1_0(&self) -> PiMr12DataF1_0R {
        PiMr12DataF1_0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates data to program into memory mode register 3 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr3_data_f1_0(&mut self) -> PiMr3DataF1_0W<DdrPiReg129Spec> {
        PiMr3DataF1_0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Indicates data to program into memory mode register 11 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr11_data_f1_0(&mut self) -> PiMr11DataF1_0W<DdrPiReg129Spec> {
        PiMr11DataF1_0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates data to program into memory mode register 12 for chip\n\nselect 0. The suffix \"_f1\" of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr12_data_f1_0(&mut self) -> PiMr12DataF1_0W<DdrPiReg129Spec> {
        PiMr12DataF1_0W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 129\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_129::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_129::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg129Spec;
impl crate::RegisterSpec for DdrPiReg129Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_129::R`](R) reader structure"]
impl crate::Readable for DdrPiReg129Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_129::W`](W) writer structure"]
impl crate::Writable for DdrPiReg129Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_129 to value 0"]
impl crate::Resettable for DdrPiReg129Spec {
    const RESET_VALUE: u32 = 0;
}

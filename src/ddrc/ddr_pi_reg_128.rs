#[doc = "Register `DDR_PI_REG_128` reader"]
pub type R = crate::R<DdrPiReg128Spec>;
#[doc = "Register `DDR_PI_REG_128` writer"]
pub type W = crate::W<DdrPiReg128Spec>;
#[doc = "Field `PI_MR1_DATA_F1_0` reader - Indicates data to program into memory mode register 1 for chip\n\nselect 0. The suffix '_f1' of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr1DataF1_0R = crate::FieldReader<u16>;
#[doc = "Field `PI_MR1_DATA_F1_0` writer - Indicates data to program into memory mode register 1 for chip\n\nselect 0. The suffix '_f1' of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr1DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_MR2_DATA_F1_0` reader - Indicates data to program into memory mode register 2 for chip\n\nselect 0. The suffix '_f1' of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr2DataF1_0R = crate::FieldReader<u16>;
#[doc = "Field `PI_MR2_DATA_F1_0` writer - Indicates data to program into memory mode register 2 for chip\n\nselect 0. The suffix '_f1' of the parameter name is omitted when in\n\nnon-DFS mode."]
pub type PiMr2DataF1_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates data to program into memory mode register 1 for chip\n\nselect 0. The suffix '_f1' of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    pub fn pi_mr1_data_f1_0(&self) -> PiMr1DataF1_0R {
        PiMr1DataF1_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Indicates data to program into memory mode register 2 for chip\n\nselect 0. The suffix '_f1' of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    pub fn pi_mr2_data_f1_0(&self) -> PiMr2DataF1_0R {
        PiMr2DataF1_0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates data to program into memory mode register 1 for chip\n\nselect 0. The suffix '_f1' of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr1_data_f1_0(&mut self) -> PiMr1DataF1_0W<DdrPiReg128Spec> {
        PiMr1DataF1_0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Indicates data to program into memory mode register 2 for chip\n\nselect 0. The suffix '_f1' of the parameter name is omitted when in\n\nnon-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr2_data_f1_0(&mut self) -> PiMr2DataF1_0W<DdrPiReg128Spec> {
        PiMr2DataF1_0W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_128::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_128::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg128Spec;
impl crate::RegisterSpec for DdrPiReg128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_128::R`](R) reader structure"]
impl crate::Readable for DdrPiReg128Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_128::W`](W) writer structure"]
impl crate::Writable for DdrPiReg128Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_128 to value 0"]
impl crate::Resettable for DdrPiReg128Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DDR_PI_REG_67` reader"]
pub type R = crate::R<DdrPiReg67Spec>;
#[doc = "Register `DDR_PI_REG_67` writer"]
pub type W = crate::W<DdrPiReg67Spec>;
#[doc = "Field `PI_ODT_EN_F0` reader - Enables support of DRAM ODT. When enabled, PI asserts and\n\nde-asserts ODT output to DRAM as needed. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiOdtEnF0R = crate::BitReader;
#[doc = "Field `PI_ODT_EN_F0` writer - Enables support of DRAM ODT. When enabled, PI asserts and\n\nde-asserts ODT output to DRAM as needed. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiOdtEnF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TODTL_2CMD_F1` reader - Defines the DRAM delay from an ODT de-assertion to the next\n\nnon-write, non-read command. The suffix \"_f1\" of the parameter\n\nname is omitted when in non-DFS mode."]
pub type PiTodtl2cmdF1R = crate::FieldReader;
#[doc = "Field `PI_TODTL_2CMD_F1` writer - Defines the DRAM delay from an ODT de-assertion to the next\n\nnon-write, non-read command. The suffix \"_f1\" of the parameter\n\nname is omitted when in non-DFS mode."]
pub type PiTodtl2cmdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_ODT_EN_F1` reader - Enables support of DRAM ODT. When enabled, PI asserts and\n\nde-asserts ODT output to DRAM as needed. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiOdtEnF1R = crate::BitReader;
#[doc = "Field `PI_ODT_EN_F1` writer - Enables support of DRAM ODT. When enabled, PI asserts and\n\nde-asserts ODT output to DRAM as needed. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiOdtEnF1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TODTL_2CMD_F2` reader - Defines the DRAM delay from an ODT de-assertion to the next\n\nnon-write, non-read command. The suffix \"_f2\" of the parameter\n\nname is omitted when in non-DFS mode."]
pub type PiTodtl2cmdF2R = crate::FieldReader;
#[doc = "Field `PI_TODTL_2CMD_F2` writer - Defines the DRAM delay from an ODT de-assertion to the next\n\nnon-write, non-read command. The suffix \"_f2\" of the parameter\n\nname is omitted when in non-DFS mode."]
pub type PiTodtl2cmdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enables support of DRAM ODT. When enabled, PI asserts and\n\nde-asserts ODT output to DRAM as needed. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_odt_en_f0(&self) -> PiOdtEnF0R {
        PiOdtEnF0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Defines the DRAM delay from an ODT de-assertion to the next\n\nnon-write, non-read command. The suffix \"_f1\" of the parameter\n\nname is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_todtl_2cmd_f1(&self) -> PiTodtl2cmdF1R {
        PiTodtl2cmdF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enables support of DRAM ODT. When enabled, PI asserts and\n\nde-asserts ODT output to DRAM as needed. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_odt_en_f1(&self) -> PiOdtEnF1R {
        PiOdtEnF1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Defines the DRAM delay from an ODT de-assertion to the next\n\nnon-write, non-read command. The suffix \"_f2\" of the parameter\n\nname is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_todtl_2cmd_f2(&self) -> PiTodtl2cmdF2R {
        PiTodtl2cmdF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables support of DRAM ODT. When enabled, PI asserts and\n\nde-asserts ODT output to DRAM as needed. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_en_f0(&mut self) -> PiOdtEnF0W<DdrPiReg67Spec> {
        PiOdtEnF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Defines the DRAM delay from an ODT de-assertion to the next\n\nnon-write, non-read command. The suffix \"_f1\" of the parameter\n\nname is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todtl_2cmd_f1(&mut self) -> PiTodtl2cmdF1W<DdrPiReg67Spec> {
        PiTodtl2cmdF1W::new(self, 8)
    }
    #[doc = "Bit 16 - Enables support of DRAM ODT. When enabled, PI asserts and\n\nde-asserts ODT output to DRAM as needed. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_en_f1(&mut self) -> PiOdtEnF1W<DdrPiReg67Spec> {
        PiOdtEnF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Defines the DRAM delay from an ODT de-assertion to the next\n\nnon-write, non-read command. The suffix \"_f2\" of the parameter\n\nname is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todtl_2cmd_f2(&mut self) -> PiTodtl2cmdF2W<DdrPiReg67Spec> {
        PiTodtl2cmdF2W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg67Spec;
impl crate::RegisterSpec for DdrPiReg67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_67::R`](R) reader structure"]
impl crate::Readable for DdrPiReg67Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_67::W`](W) writer structure"]
impl crate::Writable for DdrPiReg67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_67 to value 0"]
impl crate::Resettable for DdrPiReg67Spec {
    const RESET_VALUE: u32 = 0;
}

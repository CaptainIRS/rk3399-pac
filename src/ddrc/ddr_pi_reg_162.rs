#[doc = "Register `DDR_PI_REG_162` reader"]
pub type R = crate::R<DdrPiReg162Spec>;
#[doc = "Register `DDR_PI_REG_162` writer"]
pub type W = crate::W<DdrPiReg162Spec>;
#[doc = "Field `PI_TWTR_F1` reader - Indicates DRAM TWTR value in cycles. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTwtrF1R = crate::FieldReader;
#[doc = "Field `PI_TWTR_F1` writer - Indicates DRAM TWTR value in cycles. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTwtrF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TWR_F1` reader - Indicates DRAM TWR value in cycles. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTwrF1R = crate::FieldReader;
#[doc = "Field `PI_TWR_F1` writer - Indicates DRAM TWR value in cycles. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTwrF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Indicates DRAM TWTR value in cycles. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_twtr_f1(&self) -> PiTwtrF1R {
        PiTwtrF1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Indicates DRAM TWR value in cycles. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_twr_f1(&self) -> PiTwrF1R {
        PiTwrF1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Indicates DRAM TWTR value in cycles. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twtr_f1(&mut self) -> PiTwtrF1W<DdrPiReg162Spec> {
        PiTwtrF1W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Indicates DRAM TWR value in cycles. The suffix '_f1' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twr_f1(&mut self) -> PiTwrF1W<DdrPiReg162Spec> {
        PiTwrF1W::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 162\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_162::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_162::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg162Spec;
impl crate::RegisterSpec for DdrPiReg162Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_162::R`](R) reader structure"]
impl crate::Readable for DdrPiReg162Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_162::W`](W) writer structure"]
impl crate::Writable for DdrPiReg162Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_162 to value 0"]
impl crate::Resettable for DdrPiReg162Spec {
    const RESET_VALUE: u32 = 0;
}

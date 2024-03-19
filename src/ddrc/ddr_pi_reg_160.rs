#[doc = "Register `DDR_PI_REG_160` reader"]
pub type R = crate::R<DdrPiReg160Spec>;
#[doc = "Register `DDR_PI_REG_160` writer"]
pub type W = crate::W<DdrPiReg160Spec>;
#[doc = "Field `PI_TDQSCK_MAX_F0` reader - Indicates additional delay that is needed for tDQSCK. The suffix\n\n\"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdqsckMaxF0R = crate::FieldReader;
#[doc = "Field `PI_TDQSCK_MAX_F0` writer - Indicates additional delay that is needed for tDQSCK. The suffix\n\n\"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdqsckMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TCCDMW_F0` reader - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTccdmwF0R = crate::FieldReader;
#[doc = "Field `PI_TCCDMW_F0` writer - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTccdmwF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TMRD_F0` reader - Indicates DRAM TMRD value in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTmrdF0R = crate::FieldReader;
#[doc = "Field `PI_TMRD_F0` writer - Indicates DRAM TMRD value in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTmrdF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TMRW_F0` reader - Indicates DRAM TMRW value in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTmrwF0R = crate::FieldReader;
#[doc = "Field `PI_TMRW_F0` writer - Indicates DRAM TMRW value in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTmrwF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Indicates additional delay that is needed for tDQSCK. The suffix\n\n\"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdqsck_max_f0(&self) -> PiTdqsckMaxF0R {
        PiTdqsckMaxF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tccdmw_f0(&self) -> PiTccdmwF0R {
        PiTccdmwF0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Indicates DRAM TMRD value in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmrd_f0(&self) -> PiTmrdF0R {
        PiTmrdF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TMRW value in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmrw_f0(&self) -> PiTmrwF0R {
        PiTmrwF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates additional delay that is needed for tDQSCK. The suffix\n\n\"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdqsck_max_f0(&mut self) -> PiTdqsckMaxF0W<DdrPiReg160Spec> {
        PiTdqsckMaxF0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccdmw_f0(&mut self) -> PiTccdmwF0W<DdrPiReg160Spec> {
        PiTccdmwF0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Indicates DRAM TMRD value in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrd_f0(&mut self) -> PiTmrdF0W<DdrPiReg160Spec> {
        PiTmrdF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TMRW value in cycles. The suffix \"_f0\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrw_f0(&mut self) -> PiTmrwF0W<DdrPiReg160Spec> {
        PiTmrwF0W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 160\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_160::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_160::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg160Spec;
impl crate::RegisterSpec for DdrPiReg160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_160::R`](R) reader structure"]
impl crate::Readable for DdrPiReg160Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_160::W`](W) writer structure"]
impl crate::Writable for DdrPiReg160Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_160 to value 0"]
impl crate::Resettable for DdrPiReg160Spec {
    const RESET_VALUE: u32 = 0;
}

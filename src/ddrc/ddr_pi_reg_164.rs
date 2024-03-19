#[doc = "Register `DDR_PI_REG_164` reader"]
pub type R = crate::R<DdrPiReg164Spec>;
#[doc = "Register `DDR_PI_REG_164` writer"]
pub type W = crate::W<DdrPiReg164Spec>;
#[doc = "Field `PI_TDQSCK_MAX_F1` reader - Indicates additional delay that is needed for tDQSCK. The suffix\n\n\"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdqsckMaxF1R = crate::FieldReader;
#[doc = "Field `PI_TDQSCK_MAX_F1` writer - Indicates additional delay that is needed for tDQSCK. The suffix\n\n\"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdqsckMaxF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TCCDMW_F1` reader - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTccdmwF1R = crate::FieldReader;
#[doc = "Field `PI_TCCDMW_F1` writer - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTccdmwF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TMRD_F1` reader - Indicates DRAM TMRD value in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTmrdF1R = crate::FieldReader;
#[doc = "Field `PI_TMRD_F1` writer - Indicates DRAM TMRD value in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTmrdF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TMRW_F1` reader - Indicates DRAM TMRW value in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTmrwF1R = crate::FieldReader;
#[doc = "Field `PI_TMRW_F1` writer - Indicates DRAM TMRW value in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTmrwF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Indicates additional delay that is needed for tDQSCK. The suffix\n\n\"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdqsck_max_f1(&self) -> PiTdqsckMaxF1R {
        PiTdqsckMaxF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tccdmw_f1(&self) -> PiTccdmwF1R {
        PiTccdmwF1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Indicates DRAM TMRD value in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmrd_f1(&self) -> PiTmrdF1R {
        PiTmrdF1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TMRW value in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmrw_f1(&self) -> PiTmrwF1R {
        PiTmrwF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates additional delay that is needed for tDQSCK. The suffix\n\n\"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdqsck_max_f1(&mut self) -> PiTdqsckMaxF1W<DdrPiReg164Spec> {
        PiTdqsckMaxF1W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccdmw_f1(&mut self) -> PiTccdmwF1W<DdrPiReg164Spec> {
        PiTccdmwF1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Indicates DRAM TMRD value in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrd_f1(&mut self) -> PiTmrdF1W<DdrPiReg164Spec> {
        PiTmrdF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TMRW value in cycles. The suffix \"_f1\" of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrw_f1(&mut self) -> PiTmrwF1W<DdrPiReg164Spec> {
        PiTmrwF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 164\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_164::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_164::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg164Spec;
impl crate::RegisterSpec for DdrPiReg164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_164::R`](R) reader structure"]
impl crate::Readable for DdrPiReg164Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_164::W`](W) writer structure"]
impl crate::Writable for DdrPiReg164Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_164 to value 0"]
impl crate::Resettable for DdrPiReg164Spec {
    const RESET_VALUE: u32 = 0;
}

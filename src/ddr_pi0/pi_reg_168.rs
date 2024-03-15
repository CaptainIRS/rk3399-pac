#[doc = "Register `PI_REG_168` reader"]
pub type R = crate::R<PiReg168Spec>;
#[doc = "Register `PI_REG_168` writer"]
pub type W = crate::W<PiReg168Spec>;
#[doc = "Field `PI_TDQSCK_MAX_F2` reader - Indicates additional delay that is needed for tDQSCK. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdqsckMaxF2R = crate::FieldReader;
#[doc = "Field `PI_TDQSCK_MAX_F2` writer - Indicates additional delay that is needed for tDQSCK. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdqsckMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TCCDMW_F2` reader - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTccdmwF2R = crate::FieldReader;
#[doc = "Field `PI_TCCDMW_F2` writer - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTccdmwF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TMRD_F2` reader - Indicates DRAM TMRD value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrdF2R = crate::FieldReader;
#[doc = "Field `PI_TMRD_F2` writer - Indicates DRAM TMRD value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrdF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TMRW_F2` reader - Indicates DRAM TMRW value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrwF2R = crate::FieldReader;
#[doc = "Field `PI_TMRW_F2` writer - Indicates DRAM TMRW value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrwF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Indicates additional delay that is needed for tDQSCK. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdqsck_max_f2(&self) -> PiTdqsckMaxF2R {
        PiTdqsckMaxF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tccdmw_f2(&self) -> PiTccdmwF2R {
        PiTccdmwF2R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Indicates DRAM TMRD value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmrd_f2(&self) -> PiTmrdF2R {
        PiTmrdF2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TMRW value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmrw_f2(&self) -> PiTmrwF2R {
        PiTmrwF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates additional delay that is needed for tDQSCK. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdqsck_max_f2(&mut self) -> PiTdqsckMaxF2W<PiReg168Spec> {
        PiTdqsckMaxF2W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Indicates LPDDR4 DRAM TCCDMW in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccdmw_f2(&mut self) -> PiTccdmwF2W<PiReg168Spec> {
        PiTccdmwF2W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Indicates DRAM TMRD value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrd_f2(&mut self) -> PiTmrdF2W<PiReg168Spec> {
        PiTmrdF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TMRW value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrw_f2(&mut self) -> PiTmrwF2W<PiReg168Spec> {
        PiTmrwF2W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 168\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_168::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_168::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg168Spec;
impl crate::RegisterSpec for PiReg168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_168::R`](R) reader structure"]
impl crate::Readable for PiReg168Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_168::W`](W) writer structure"]
impl crate::Writable for PiReg168Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_168 to value 0"]
impl crate::Resettable for PiReg168Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PI_REG_158` reader"]
pub type R = crate::R<PiReg158Spec>;
#[doc = "Register `PI_REG_158` writer"]
pub type W = crate::W<PiReg158Spec>;
#[doc = "Field `PI_TRP_F0` reader - Indicates DRAM TRP value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrpF0R = crate::FieldReader;
#[doc = "Field `PI_TRP_F0` writer - Indicates DRAM TRP value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrpF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRCD_F0` reader - Indicates DRAM TRCD value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrcdF0R = crate::FieldReader;
#[doc = "Field `PI_TRCD_F0` writer - Indicates DRAM TRCD value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrcdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TWTR_F0` reader - Indicates DRAM TWTR value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTwtrF0R = crate::FieldReader;
#[doc = "Field `PI_TWTR_F0` writer - Indicates DRAM TWTR value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTwtrF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TWR_F0` reader - Indicates DRAM TWR value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTwrF0R = crate::FieldReader;
#[doc = "Field `PI_TWR_F0` writer - Indicates DRAM TWR value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTwrF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - Indicates DRAM TRP value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trp_f0(&self) -> PiTrpF0R {
        PiTrpF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates DRAM TRCD value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trcd_f0(&self) -> PiTrcdF0R {
        PiTrcdF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - Indicates DRAM TWTR value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_twtr_f0(&self) -> PiTwtrF0R {
        PiTwtrF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Indicates DRAM TWR value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_twr_f0(&self) -> PiTwrF0R {
        PiTwrF0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates DRAM TRP value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trp_f0(&mut self) -> PiTrpF0W<PiReg158Spec> {
        PiTrpF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates DRAM TRCD value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trcd_f0(&mut self) -> PiTrcdF0W<PiReg158Spec> {
        PiTrcdF0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Indicates DRAM TWTR value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twtr_f0(&mut self) -> PiTwtrF0W<PiReg158Spec> {
        PiTwtrF0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Indicates DRAM TWR value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twr_f0(&mut self) -> PiTwrF0W<PiReg158Spec> {
        PiTwrF0W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 158\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_158::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_158::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg158Spec;
impl crate::RegisterSpec for PiReg158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_158::R`](R) reader structure"]
impl crate::Readable for PiReg158Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_158::W`](W) writer structure"]
impl crate::Writable for PiReg158Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_158 to value 0"]
impl crate::Resettable for PiReg158Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PI_REG_165` reader"]
pub type R = crate::R<PiReg165Spec>;
#[doc = "Register `PI_REG_165` writer"]
pub type W = crate::W<PiReg165Spec>;
#[doc = "Field `PI_TMOD_F1` reader - Indicates TMOD value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmodF1R = crate::FieldReader;
#[doc = "Field `PI_TMOD_F1` writer - Indicates TMOD value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmodF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRTP_F2` reader - Indicates DRAM TRTP value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrtpF2R = crate::FieldReader;
#[doc = "Field `PI_TRTP_F2` writer - Indicates DRAM TRTP value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrtpF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRP_F2` reader - Indicates DRAM TRP value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrpF2R = crate::FieldReader;
#[doc = "Field `PI_TRP_F2` writer - Indicates DRAM TRP value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrpF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRCD_F2` reader - Indicates DRAM TRCD value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrcdF2R = crate::FieldReader;
#[doc = "Field `PI_TRCD_F2` writer - Indicates DRAM TRCD value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrcdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates TMOD value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmod_f1(&self) -> PiTmodF1R {
        PiTmodF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates DRAM TRTP value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trtp_f2(&self) -> PiTrtpF2R {
        PiTrtpF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Indicates DRAM TRP value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trp_f2(&self) -> PiTrpF2R {
        PiTrpF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRCD value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trcd_f2(&self) -> PiTrcdF2R {
        PiTrcdF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates TMOD value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmod_f1(&mut self) -> PiTmodF1W<PiReg165Spec> {
        PiTmodF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates DRAM TRTP value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trtp_f2(&mut self) -> PiTrtpF2W<PiReg165Spec> {
        PiTrtpF2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates DRAM TRP value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trp_f2(&mut self) -> PiTrpF2W<PiReg165Spec> {
        PiTrpF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRCD value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trcd_f2(&mut self) -> PiTrcdF2W<PiReg165Spec> {
        PiTrcdF2W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 165\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_165::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_165::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg165Spec;
impl crate::RegisterSpec for PiReg165Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_165::R`](R) reader structure"]
impl crate::Readable for PiReg165Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_165::W`](W) writer structure"]
impl crate::Writable for PiReg165Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_165 to value 0"]
impl crate::Resettable for PiReg165Spec {
    const RESET_VALUE: u32 = 0;
}

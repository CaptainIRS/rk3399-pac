#[doc = "Register `DDR_PI_REG_161` reader"]
pub type R = crate::R<DdrPiReg161Spec>;
#[doc = "Register `DDR_PI_REG_161` writer"]
pub type W = crate::W<DdrPiReg161Spec>;
#[doc = "Field `PI_TMOD_F0` reader - Indicates TMOD value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmodF0R = crate::FieldReader;
#[doc = "Field `PI_TMOD_F0` writer - Indicates TMOD value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmodF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRTP_F1` reader - Indicates DRAM TRTP value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrtpF1R = crate::FieldReader;
#[doc = "Field `PI_TRTP_F1` writer - Indicates DRAM TRTP value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrtpF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRP_F1` reader - Indicates DRAM TRP value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrpF1R = crate::FieldReader;
#[doc = "Field `PI_TRP_F1` writer - Indicates DRAM TRP value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrpF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRCD_F1` reader - Indicates DRAM TRCD value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrcdF1R = crate::FieldReader;
#[doc = "Field `PI_TRCD_F1` writer - Indicates DRAM TRCD value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrcdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates TMOD value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmod_f0(&self) -> PiTmodF0R {
        PiTmodF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates DRAM TRTP value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trtp_f1(&self) -> PiTrtpF1R {
        PiTrtpF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Indicates DRAM TRP value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trp_f1(&self) -> PiTrpF1R {
        PiTrpF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRCD value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trcd_f1(&self) -> PiTrcdF1R {
        PiTrcdF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates TMOD value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmod_f0(&mut self) -> PiTmodF0W<DdrPiReg161Spec> {
        PiTmodF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates DRAM TRTP value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trtp_f1(&mut self) -> PiTrtpF1W<DdrPiReg161Spec> {
        PiTrtpF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates DRAM TRP value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trp_f1(&mut self) -> PiTrpF1W<DdrPiReg161Spec> {
        PiTrpF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates DRAM TRCD value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trcd_f1(&mut self) -> PiTrcdF1W<DdrPiReg161Spec> {
        PiTrcdF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 161\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_161::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_161::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg161Spec;
impl crate::RegisterSpec for DdrPiReg161Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_161::R`](R) reader structure"]
impl crate::Readable for DdrPiReg161Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_161::W`](W) writer structure"]
impl crate::Writable for DdrPiReg161Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_161 to value 0"]
impl crate::Resettable for DdrPiReg161Spec {
    const RESET_VALUE: u32 = 0;
}

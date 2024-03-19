#[doc = "Register `DDR_PI_REG_47` reader"]
pub type R = crate::R<DdrPiReg47Spec>;
#[doc = "Register `DDR_PI_REG_47` writer"]
pub type W = crate::W<DdrPiReg47Spec>;
#[doc = "Field `PI_TRFC_F1` reader - Indicates DRAM TRFC value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrfcF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TRFC_F1` writer - Indicates DRAM TRFC value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrfcF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TREF_F1` reader - Indicates DRAM TREF value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrefF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TREF_F1` writer - Indicates DRAM TREF value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTrefF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - Indicates DRAM TRFC value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trfc_f1(&self) -> PiTrfcF1R {
        PiTrfcF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Indicates DRAM TREF value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tref_f1(&self) -> PiTrefF1R {
        PiTrefF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Indicates DRAM TRFC value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trfc_f1(&mut self) -> PiTrfcF1W<DdrPiReg47Spec> {
        PiTrfcF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Indicates DRAM TREF value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tref_f1(&mut self) -> PiTrefF1W<DdrPiReg47Spec> {
        PiTrefF1W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg47Spec;
impl crate::RegisterSpec for DdrPiReg47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_47::R`](R) reader structure"]
impl crate::Readable for DdrPiReg47Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_47::W`](W) writer structure"]
impl crate::Writable for DdrPiReg47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_47 to value 0"]
impl crate::Resettable for DdrPiReg47Spec {
    const RESET_VALUE: u32 = 0;
}

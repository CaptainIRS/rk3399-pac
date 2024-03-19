#[doc = "Register `DDR_PI_REG_103` reader"]
pub type R = crate::R<DdrPiReg103Spec>;
#[doc = "Register `DDR_PI_REG_103` writer"]
pub type W = crate::W<DdrPiReg103Spec>;
#[doc = "Field `PI_TMRZ_F1` reader - Indicates DRAM TMRZ value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrzF1R = crate::FieldReader;
#[doc = "Field `PI_TMRZ_F1` writer - Indicates DRAM TMRZ value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrzF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TCAENT_F1` reader - Indicates DRAM TCAENT value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTcaentF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TCAENT_F1` writer - Indicates DRAM TCAENT value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTcaentF1W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PI_TMRZ_F2` reader - Indicates DRAM TMRZ value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrzF2R = crate::FieldReader;
#[doc = "Field `PI_TMRZ_F2` writer - Indicates DRAM TMRZ value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrzF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Indicates DRAM TMRZ value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmrz_f1(&self) -> PiTmrzF1R {
        PiTmrzF1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:21 - Indicates DRAM TCAENT value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tcaent_f1(&self) -> PiTcaentF1R {
        PiTcaentF1R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 24:28 - Indicates DRAM TMRZ value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmrz_f2(&self) -> PiTmrzF2R {
        PiTmrzF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Indicates DRAM TMRZ value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrz_f1(&mut self) -> PiTmrzF1W<DdrPiReg103Spec> {
        PiTmrzF1W::new(self, 0)
    }
    #[doc = "Bits 8:21 - Indicates DRAM TCAENT value in cycles. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcaent_f1(&mut self) -> PiTcaentF1W<DdrPiReg103Spec> {
        PiTcaentF1W::new(self, 8)
    }
    #[doc = "Bits 24:28 - Indicates DRAM TMRZ value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrz_f2(&mut self) -> PiTmrzF2W<DdrPiReg103Spec> {
        PiTmrzF2W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 103\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_103::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_103::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg103Spec;
impl crate::RegisterSpec for DdrPiReg103Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_103::R`](R) reader structure"]
impl crate::Readable for DdrPiReg103Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_103::W`](W) writer structure"]
impl crate::Writable for DdrPiReg103Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_103 to value 0"]
impl crate::Resettable for DdrPiReg103Spec {
    const RESET_VALUE: u32 = 0;
}

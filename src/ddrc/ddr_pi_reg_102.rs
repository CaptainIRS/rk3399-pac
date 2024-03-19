#[doc = "Register `DDR_PI_REG_102` reader"]
pub type R = crate::R<DdrPiReg102Spec>;
#[doc = "Register `DDR_PI_REG_102` writer"]
pub type W = crate::W<DdrPiReg102Spec>;
#[doc = "Field `PI_TCACKEH` reader - Indicates DRAM TCACKEH value in cycles."]
pub type PiTcackehR = crate::FieldReader;
#[doc = "Field `PI_TCACKEH` writer - Indicates DRAM TCACKEH value in cycles."]
pub type PiTcackehW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TMRZ_F0` reader - Indicates DRAM TMRZ value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrzF0R = crate::FieldReader;
#[doc = "Field `PI_TMRZ_F0` writer - Indicates DRAM TMRZ value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTmrzF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TCAENT_F0` reader - Indicates DRAM TCAENT value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTcaentF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TCAENT_F0` writer - Indicates DRAM TCAENT value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTcaentF0W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:4 - Indicates DRAM TCACKEH value in cycles."]
    #[inline(always)]
    pub fn pi_tcackeh(&self) -> PiTcackehR {
        PiTcackehR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Indicates DRAM TMRZ value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tmrz_f0(&self) -> PiTmrzF0R {
        PiTmrzF0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:29 - Indicates DRAM TCAENT value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tcaent_f0(&self) -> PiTcaentF0R {
        PiTcaentF0R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Indicates DRAM TCACKEH value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcackeh(&mut self) -> PiTcackehW<DdrPiReg102Spec> {
        PiTcackehW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Indicates DRAM TMRZ value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrz_f0(&mut self) -> PiTmrzF0W<DdrPiReg102Spec> {
        PiTmrzF0W::new(self, 8)
    }
    #[doc = "Bits 16:29 - Indicates DRAM TCAENT value in cycles. The suffix \"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcaent_f0(&mut self) -> PiTcaentF0W<DdrPiReg102Spec> {
        PiTcaentF0W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 102\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_102::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_102::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg102Spec;
impl crate::RegisterSpec for DdrPiReg102Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_102::R`](R) reader structure"]
impl crate::Readable for DdrPiReg102Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_102::W`](W) writer structure"]
impl crate::Writable for DdrPiReg102Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_102 to value 0"]
impl crate::Resettable for DdrPiReg102Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DDR_PI_REG_48` reader"]
pub type R = crate::R<DdrPiReg48Spec>;
#[doc = "Register `DDR_PI_REG_48` writer"]
pub type W = crate::W<DdrPiReg48Spec>;
#[doc = "Field `PI_TRFC_F2` reader - Indicates DRAM TRFC value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTrfcF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TRFC_F2` writer - Indicates DRAM TRFC value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTrfcF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TREF_F2` reader - Indicates DRAM TREF value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTrefF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TREF_F2` writer - Indicates DRAM TREF value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
pub type PiTrefF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - Indicates DRAM TRFC value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_trfc_f2(&self) -> PiTrfcF2R {
        PiTrfcF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Indicates DRAM TREF value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tref_f2(&self) -> PiTrefF2R {
        PiTrefF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Indicates DRAM TRFC value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trfc_f2(&mut self) -> PiTrfcF2W<DdrPiReg48Spec> {
        PiTrfcF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Indicates DRAM TREF value in cycles. The suffix '_f2' of the\n\nparameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tref_f2(&mut self) -> PiTrefF2W<DdrPiReg48Spec> {
        PiTrefF2W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg48Spec;
impl crate::RegisterSpec for DdrPiReg48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_48::R`](R) reader structure"]
impl crate::Readable for DdrPiReg48Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_48::W`](W) writer structure"]
impl crate::Writable for DdrPiReg48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_48 to value 0"]
impl crate::Resettable for DdrPiReg48Spec {
    const RESET_VALUE: u32 = 0;
}

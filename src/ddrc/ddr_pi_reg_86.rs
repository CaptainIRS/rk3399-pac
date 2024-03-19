#[doc = "Register `DDR_PI_REG_86` reader"]
pub type R = crate::R<DdrPiReg86Spec>;
#[doc = "Register `DDR_PI_REG_86` writer"]
pub type W = crate::W<DdrPiReg86Spec>;
#[doc = "Field `PI_LPDDR4_RDLVL_PATTERN_9` reader - Indicates user-defined LPDDR4 read data pattern 9."]
pub type PiLpddr4RdlvlPattern9R = crate::FieldReader<u32>;
#[doc = "Field `PI_LPDDR4_RDLVL_PATTERN_9` writer - Indicates user-defined LPDDR4 read data pattern 9."]
pub type PiLpddr4RdlvlPattern9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates user-defined LPDDR4 read data pattern 9."]
    #[inline(always)]
    pub fn pi_lpddr4_rdlvl_pattern_9(&self) -> PiLpddr4RdlvlPattern9R {
        PiLpddr4RdlvlPattern9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates user-defined LPDDR4 read data pattern 9."]
    #[inline(always)]
    #[must_use]
    pub fn pi_lpddr4_rdlvl_pattern_9(&mut self) -> PiLpddr4RdlvlPattern9W<DdrPiReg86Spec> {
        PiLpddr4RdlvlPattern9W::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_86::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_86::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg86Spec;
impl crate::RegisterSpec for DdrPiReg86Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_86::R`](R) reader structure"]
impl crate::Readable for DdrPiReg86Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_86::W`](W) writer structure"]
impl crate::Writable for DdrPiReg86Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_86 to value 0"]
impl crate::Resettable for DdrPiReg86Spec {
    const RESET_VALUE: u32 = 0;
}

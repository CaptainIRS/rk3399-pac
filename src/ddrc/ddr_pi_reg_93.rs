#[doc = "Register `DDR_PI_REG_93` reader"]
pub type R = crate::R<DdrPiReg93Spec>;
#[doc = "Register `DDR_PI_REG_93` writer"]
pub type W = crate::W<DdrPiReg93Spec>;
#[doc = "Field `PI_CALVL_SEQ_EN` reader - Specifies the CA training patterns that are to be used. Set to 0 for pattern 0 only, set to 1 for patterns 0 and 1, set to 2 for patterns 0, 1, and 2, or set to 3 for all patterns."]
pub type PiCalvlSeqEnR = crate::FieldReader;
#[doc = "Field `PI_CALVL_SEQ_EN` writer - Specifies the CA training patterns that are to be used. Set to 0 for pattern 0 only, set to 1 for patterns 0 and 1, set to 2 for patterns 0, 1, and 2, or set to 3 for all patterns."]
pub type PiCalvlSeqEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_CALVL_PERIODIC` reader - Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
pub type PiCalvlPeriodicR = crate::BitReader;
#[doc = "Field `PI_CALVL_PERIODIC` writer - Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
pub type PiCalvlPeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - Specifies the CA training patterns that are to be used. Set to 0 for pattern 0 only, set to 1 for patterns 0 and 1, set to 2 for patterns 0, 1, and 2, or set to 3 for all patterns."]
    #[inline(always)]
    pub fn pi_calvl_seq_en(&self) -> PiCalvlSeqEnR {
        PiCalvlSeqEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_calvl_periodic(&self) -> PiCalvlPeriodicR {
        PiCalvlPeriodicR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Specifies the CA training patterns that are to be used. Set to 0 for pattern 0 only, set to 1 for patterns 0 and 1, set to 2 for patterns 0, 1, and 2, or set to 3 for all patterns."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_seq_en(&mut self) -> PiCalvlSeqEnW<DdrPiReg93Spec> {
        PiCalvlSeqEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_periodic(&mut self) -> PiCalvlPeriodicW<DdrPiReg93Spec> {
        PiCalvlPeriodicW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 93\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_93::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_93::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg93Spec;
impl crate::RegisterSpec for DdrPiReg93Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_93::R`](R) reader structure"]
impl crate::Readable for DdrPiReg93Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_93::W`](W) writer structure"]
impl crate::Writable for DdrPiReg93Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_93 to value 0"]
impl crate::Resettable for DdrPiReg93Spec {
    const RESET_VALUE: u32 = 0;
}

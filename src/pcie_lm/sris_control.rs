#[doc = "Register `SRIS_CONTROL` reader"]
pub type R = crate::R<SrisControlSpec>;
#[doc = "Register `SRIS_CONTROL` writer"]
pub type W = crate::W<SrisControlSpec>;
#[doc = "Field `SRISE` reader - SRIS Enable \\[SRISE\\]
Setting this bit enables SRIS mode in the PHY layer. This bit should be before link training begins by holding the LINK_TRAINING_ENABLE input to 1'b0."]
pub type SriseR = crate::BitReader;
#[doc = "Field `SRISE` writer - SRIS Enable \\[SRISE\\]
Setting this bit enables SRIS mode in the PHY layer. This bit should be before link training begins by holding the LINK_TRAINING_ENABLE input to 1'b0."]
pub type SriseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R31` reader - Reserved \\[R31\\]
Reserved"]
pub type R31R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - SRIS Enable \\[SRISE\\]
Setting this bit enables SRIS mode in the PHY layer. This bit should be before link training begins by holding the LINK_TRAINING_ENABLE input to 1'b0."]
    #[inline(always)]
    pub fn srise(&self) -> SriseR {
        SriseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved \\[R31\\]
Reserved"]
    #[inline(always)]
    pub fn r31(&self) -> R31R {
        R31R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - SRIS Enable \\[SRISE\\]
Setting this bit enables SRIS mode in the PHY layer. This bit should be before link training begins by holding the LINK_TRAINING_ENABLE input to 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn srise(&mut self) -> SriseW<SrisControlSpec> {
        SriseW::new(self, 0)
    }
}
#[doc = "SRIS Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sris_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sris_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrisControlSpec;
impl crate::RegisterSpec for SrisControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sris_control::R`](R) reader structure"]
impl crate::Readable for SrisControlSpec {}
#[doc = "`write(|w| ..)` method takes [`sris_control::W`](W) writer structure"]
impl crate::Writable for SrisControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRIS_CONTROL to value 0"]
impl crate::Resettable for SrisControlSpec {
    const RESET_VALUE: u32 = 0;
}

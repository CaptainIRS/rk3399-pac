#[doc = "Register `SRSZ_PHASE_HY` reader"]
pub type R = crate::R<SrszPhaseHySpec>;
#[doc = "Register `SRSZ_PHASE_HY` writer"]
pub type W = crate::W<SrszPhaseHySpec>;
#[doc = "Field `phase_hy` reader - This register is set to the horizontal luminance phase\n\noffset\n\n"]
pub type PhaseHyR = crate::FieldReader<u16>;
#[doc = "Field `phase_hy` writer - This register is set to the horizontal luminance phase\n\noffset\n\n"]
pub type PhaseHyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal luminance phase\n\noffset\n\n"]
    #[inline(always)]
    pub fn phase_hy(&self) -> PhaseHyR {
        PhaseHyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the horizontal luminance phase\n\noffset\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn phase_hy(&mut self) -> PhaseHyW<SrszPhaseHySpec> {
        PhaseHyW::new(self, 0)
    }
}
#[doc = "horizontal luminance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_hy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_phase_hy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszPhaseHySpec;
impl crate::RegisterSpec for SrszPhaseHySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_phase_hy::R`](R) reader structure"]
impl crate::Readable for SrszPhaseHySpec {}
#[doc = "`write(|w| ..)` method takes [`srsz_phase_hy::W`](W) writer structure"]
impl crate::Writable for SrszPhaseHySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSZ_PHASE_HY to value 0"]
impl crate::Resettable for SrszPhaseHySpec {
    const RESET_VALUE: u32 = 0;
}

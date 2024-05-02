#[doc = "Register `MRSZ_PHASE_VY` reader"]
pub type R = crate::R<MrszPhaseVySpec>;
#[doc = "Register `MRSZ_PHASE_VY` writer"]
pub type W = crate::W<MrszPhaseVySpec>;
#[doc = "Field `phase_vy` reader - This register is set to the vertical luminance phase\n\noffset\n\n"]
pub type PhaseVyR = crate::FieldReader<u16>;
#[doc = "Field `phase_vy` writer - This register is set to the vertical luminance phase\n\noffset\n\n"]
pub type PhaseVyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the vertical luminance phase\n\noffset\n\n"]
    #[inline(always)]
    pub fn phase_vy(&self) -> PhaseVyR {
        PhaseVyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the vertical luminance phase\n\noffset\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn phase_vy(&mut self) -> PhaseVyW<MrszPhaseVySpec> {
        PhaseVyW::new(self, 0)
    }
}
#[doc = "vertical luminance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_vy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_phase_vy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszPhaseVySpec;
impl crate::RegisterSpec for MrszPhaseVySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_phase_vy::R`](R) reader structure"]
impl crate::Readable for MrszPhaseVySpec {}
#[doc = "`write(|w| ..)` method takes [`mrsz_phase_vy::W`](W) writer structure"]
impl crate::Writable for MrszPhaseVySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRSZ_PHASE_VY to value 0"]
impl crate::Resettable for MrszPhaseVySpec {
    const RESET_VALUE: u32 = 0;
}

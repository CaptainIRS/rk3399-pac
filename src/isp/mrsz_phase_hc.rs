#[doc = "Register `MRSZ_PHASE_HC` reader"]
pub type R = crate::R<MrszPhaseHcSpec>;
#[doc = "Register `MRSZ_PHASE_HC` writer"]
pub type W = crate::W<MrszPhaseHcSpec>;
#[doc = "Field `phase_hc` reader - This register is set to the horizontal chrominance\n\nphase offset\n\n"]
pub type PhaseHcR = crate::FieldReader<u16>;
#[doc = "Field `phase_hc` writer - This register is set to the horizontal chrominance\n\nphase offset\n\n"]
pub type PhaseHcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal chrominance\n\nphase offset\n\n"]
    #[inline(always)]
    pub fn phase_hc(&self) -> PhaseHcR {
        PhaseHcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the horizontal chrominance\n\nphase offset\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn phase_hc(&mut self) -> PhaseHcW<MrszPhaseHcSpec> {
        PhaseHcW::new(self, 0)
    }
}
#[doc = "horizontal chrominance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_hc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_phase_hc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszPhaseHcSpec;
impl crate::RegisterSpec for MrszPhaseHcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_phase_hc::R`](R) reader structure"]
impl crate::Readable for MrszPhaseHcSpec {}
#[doc = "`write(|w| ..)` method takes [`mrsz_phase_hc::W`](W) writer structure"]
impl crate::Writable for MrszPhaseHcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRSZ_PHASE_HC to value 0"]
impl crate::Resettable for MrszPhaseHcSpec {
    const RESET_VALUE: u32 = 0;
}

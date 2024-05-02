#[doc = "Register `SRSZ_PHASE_VC` reader"]
pub type R = crate::R<SrszPhaseVcSpec>;
#[doc = "Register `SRSZ_PHASE_VC` writer"]
pub type W = crate::W<SrszPhaseVcSpec>;
#[doc = "Field `phase_vc` reader - This register is set to the vertical chrominance phase\n\noffset"]
pub type PhaseVcR = crate::FieldReader<u16>;
#[doc = "Field `phase_vc` writer - This register is set to the vertical chrominance phase\n\noffset"]
pub type PhaseVcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the vertical chrominance phase\n\noffset"]
    #[inline(always)]
    pub fn phase_vc(&self) -> PhaseVcR {
        PhaseVcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is set to the vertical chrominance phase\n\noffset"]
    #[inline(always)]
    #[must_use]
    pub fn phase_vc(&mut self) -> PhaseVcW<SrszPhaseVcSpec> {
        PhaseVcW::new(self, 0)
    }
}
#[doc = "vertical chrominance phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_vc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_phase_vc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszPhaseVcSpec;
impl crate::RegisterSpec for SrszPhaseVcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_phase_vc::R`](R) reader structure"]
impl crate::Readable for SrszPhaseVcSpec {}
#[doc = "`write(|w| ..)` method takes [`srsz_phase_vc::W`](W) writer structure"]
impl crate::Writable for SrszPhaseVcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSZ_PHASE_VC to value 0"]
impl crate::Resettable for SrszPhaseVcSpec {
    const RESET_VALUE: u32 = 0;
}

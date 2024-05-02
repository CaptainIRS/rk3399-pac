#[doc = "Register `MRSZ_PHASE_HC_SHD` reader"]
pub type R = crate::R<MrszPhaseHcShdSpec>;
#[doc = "Field `phase_hc_shd` reader - This register is set to the horizontal chrominance\n\nphase offset"]
pub type PhaseHcShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal chrominance\n\nphase offset"]
    #[inline(always)]
    pub fn phase_hc_shd(&self) -> PhaseHcShdR {
        PhaseHcShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "horizontal chrominance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_hc_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszPhaseHcShdSpec;
impl crate::RegisterSpec for MrszPhaseHcShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_phase_hc_shd::R`](R) reader structure"]
impl crate::Readable for MrszPhaseHcShdSpec {}
#[doc = "`reset()` method sets MRSZ_PHASE_HC_SHD to value 0"]
impl crate::Resettable for MrszPhaseHcShdSpec {
    const RESET_VALUE: u32 = 0;
}

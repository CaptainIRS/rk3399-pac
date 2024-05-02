#[doc = "Register `SRSZ_PHASE_VC_SHD` reader"]
pub type R = crate::R<SrszPhaseVcShdSpec>;
#[doc = "Field `phase_vc_shd` reader - This register is set to the vertical chrominance phase\n\noffset\n\n"]
pub type PhaseVcShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the vertical chrominance phase\n\noffset\n\n"]
    #[inline(always)]
    pub fn phase_vc_shd(&self) -> PhaseVcShdR {
        PhaseVcShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "vertical chrominance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_vc_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszPhaseVcShdSpec;
impl crate::RegisterSpec for SrszPhaseVcShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_phase_vc_shd::R`](R) reader structure"]
impl crate::Readable for SrszPhaseVcShdSpec {}
#[doc = "`reset()` method sets SRSZ_PHASE_VC_SHD to value 0"]
impl crate::Resettable for SrszPhaseVcShdSpec {
    const RESET_VALUE: u32 = 0;
}

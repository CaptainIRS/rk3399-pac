#[doc = "Register `SRSZ_PHASE_HY_SHD` reader"]
pub type R = crate::R<SrszPhaseHyShdSpec>;
#[doc = "Field `phase_hy_shd` reader - This register is set to the horizontal luminance phase\n\noffset\n\n"]
pub type PhaseHyShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal luminance phase\n\noffset\n\n"]
    #[inline(always)]
    pub fn phase_hy_shd(&self) -> PhaseHyShdR {
        PhaseHyShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "horizontal luminance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_phase_hy_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszPhaseHyShdSpec;
impl crate::RegisterSpec for SrszPhaseHyShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_phase_hy_shd::R`](R) reader structure"]
impl crate::Readable for SrszPhaseHyShdSpec {}
#[doc = "`reset()` method sets SRSZ_PHASE_HY_SHD to value 0"]
impl crate::Resettable for SrszPhaseHyShdSpec {
    const RESET_VALUE: u32 = 0;
}

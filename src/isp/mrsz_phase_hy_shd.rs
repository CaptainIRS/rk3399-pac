#[doc = "Register `MRSZ_PHASE_HY_SHD` reader"]
pub type R = crate::R<MrszPhaseHyShdSpec>;
#[doc = "Field `phase_hy_shd` reader - This register is set to the horizontal luminance phase\n\noffset"]
pub type PhaseHyShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the horizontal luminance phase\n\noffset"]
    #[inline(always)]
    pub fn phase_hy_shd(&self) -> PhaseHyShdR {
        PhaseHyShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "horizontal luminance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_hy_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszPhaseHyShdSpec;
impl crate::RegisterSpec for MrszPhaseHyShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_phase_hy_shd::R`](R) reader structure"]
impl crate::Readable for MrszPhaseHyShdSpec {}
#[doc = "`reset()` method sets MRSZ_PHASE_HY_SHD to value 0"]
impl crate::Resettable for MrszPhaseHyShdSpec {
    const RESET_VALUE: u32 = 0;
}

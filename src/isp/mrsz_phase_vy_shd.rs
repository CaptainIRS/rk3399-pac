#[doc = "Register `MRSZ_PHASE_VY_SHD` reader"]
pub type R = crate::R<MrszPhaseVyShdSpec>;
#[doc = "Field `phase_vy_shd` reader - This register is set to the vertical luminance phase\n\noffset\n\n"]
pub type PhaseVyShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register is set to the vertical luminance phase\n\noffset\n\n"]
    #[inline(always)]
    pub fn phase_vy_shd(&self) -> PhaseVyShdR {
        PhaseVyShdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "vertical luminance phase shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_phase_vy_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszPhaseVyShdSpec;
impl crate::RegisterSpec for MrszPhaseVyShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_phase_vy_shd::R`](R) reader structure"]
impl crate::Readable for MrszPhaseVyShdSpec {}
#[doc = "`reset()` method sets MRSZ_PHASE_VY_SHD to value 0"]
impl crate::Resettable for MrszPhaseVyShdSpec {
    const RESET_VALUE: u32 = 0;
}

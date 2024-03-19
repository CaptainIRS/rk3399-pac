#[doc = "Register `VP_STATUS` reader"]
pub type R = crate::R<VpStatusSpec>;
#[doc = "Field `PACKING_PHASE` reader - Read only register that holds the \"packing phase\"\n\noutput of the Video Packetizer block."]
pub type PackingPhaseR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Read only register that holds the \"packing phase\"\n\noutput of the Video Packetizer block."]
    #[inline(always)]
    pub fn packing_phase(&self) -> PackingPhaseR {
        PackingPhaseR::new(self.bits & 0x0f)
    }
}
#[doc = "Video Packetizer Packing Phase Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vp_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VpStatusSpec;
impl crate::RegisterSpec for VpStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vp_status::R`](R) reader structure"]
impl crate::Readable for VpStatusSpec {}
#[doc = "`reset()` method sets VP_STATUS to value 0"]
impl crate::Resettable for VpStatusSpec {
    const RESET_VALUE: u8 = 0;
}

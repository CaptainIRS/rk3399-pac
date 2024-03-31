#[doc = "Register `DEBUG_OUT_0` reader"]
pub type R = crate::R<DebugOut0Spec>;
#[doc = "Field `LTSSM_STATE` reader - Link training and status state\n\nCurrent state of the Link Training and Status State\n\nMachine within the core. The encodings of this\n\noutput are described in Appendix B"]
pub type LtssmStateR = crate::FieldReader;
#[doc = "Field `DEBUG_DATA_OUT` reader - Output data from the debug bus\n\n16-bit output data from the debug bus, described in Appendix A"]
pub type DebugDataOutR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - Link training and status state\n\nCurrent state of the Link Training and Status State\n\nMachine within the core. The encodings of this\n\noutput are described in Appendix B"]
    #[inline(always)]
    pub fn ltssm_state(&self) -> LtssmStateR {
        LtssmStateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Output data from the debug bus\n\n16-bit output data from the debug bus, described in Appendix A"]
    #[inline(always)]
    pub fn debug_data_out(&self) -> DebugDataOutR {
        DebugDataOutR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Debug information 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_out_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugOut0Spec;
impl crate::RegisterSpec for DebugOut0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_out_0::R`](R) reader structure"]
impl crate::Readable for DebugOut0Spec {}
#[doc = "`reset()` method sets DEBUG_OUT_0 to value 0"]
impl crate::Resettable for DebugOut0Spec {
    const RESET_VALUE: u32 = 0;
}

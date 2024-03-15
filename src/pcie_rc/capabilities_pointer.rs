#[doc = "Register `CAPABILITIES_POINTER` reader"]
pub type R = crate::R<CapabilitiesPointerSpec>;
#[doc = "Field `CP` reader - Capabilities Pointer \\[CP\\]
Contains pointer to the first PCI Capability Structure. This field is set by default to the value defined in the RTL file reg_defaults.h. It can be re- written independently for every Function from the local management APB bus."]
pub type CpR = crate::FieldReader;
#[doc = "Field `R15` reader - Reserved \\[R15\\]
Reserved"]
pub type R15R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Capabilities Pointer \\[CP\\]
Contains pointer to the first PCI Capability Structure. This field is set by default to the value defined in the RTL file reg_defaults.h. It can be re- written independently for every Function from the local management APB bus."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved \\[R15\\]
Reserved"]
    #[inline(always)]
    pub fn r15(&self) -> R15R {
        R15R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Capabilities Pointer Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities_pointer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapabilitiesPointerSpec;
impl crate::RegisterSpec for CapabilitiesPointerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities_pointer::R`](R) reader structure"]
impl crate::Readable for CapabilitiesPointerSpec {}
#[doc = "`reset()` method sets CAPABILITIES_POINTER to value 0x80"]
impl crate::Resettable for CapabilitiesPointerSpec {
    const RESET_VALUE: u32 = 0x80;
}

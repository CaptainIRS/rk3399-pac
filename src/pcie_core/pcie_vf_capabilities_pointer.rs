#[doc = "Register `PCIE_VF_CAPABILITIES_POINTER` reader"]
pub type R = crate::R<PcieVfCapabilitiesPointerSpec>;
#[doc = "Field `CP` reader - Capabilities Pointer \\[CP\\]
Contains pointer to the first PCI Capability Structure. This field is set by default to point to the Power Management Capability Structure. It can be modified by writing to VF 0 from the local management bus, and the setting is common across all VFs."]
pub type CpR = crate::FieldReader;
#[doc = "Field `R6` reader - Reserved \\[R6\\]
Reserved"]
pub type R6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Capabilities Pointer \\[CP\\]
Contains pointer to the first PCI Capability Structure. This field is set by default to point to the Power Management Capability Structure. It can be modified by writing to VF 0 from the local management bus, and the setting is common across all VFs."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved \\[R6\\]
Reserved"]
    #[inline(always)]
    pub fn r6(&self) -> R6R {
        R6R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Capabilities Pointer Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_capabilities_pointer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfCapabilitiesPointerSpec;
impl crate::RegisterSpec for PcieVfCapabilitiesPointerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_capabilities_pointer::R`](R) reader structure"]
impl crate::Readable for PcieVfCapabilitiesPointerSpec {}
#[doc = "`reset()` method sets PCIE_VF_CAPABILITIES_POINTER to value 0x80"]
impl crate::Resettable for PcieVfCapabilitiesPointerSpec {
    const RESET_VALUE: u32 = 0x80;
}

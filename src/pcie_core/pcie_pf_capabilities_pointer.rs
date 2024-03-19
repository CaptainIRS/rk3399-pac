#[doc = "Register `PCIE_PF_CAPABILITIES_POINTER` reader"]
pub type R = crate::R<PciePfCapabilitiesPointerSpec>;
#[doc = "Field `CP` reader - Capabilities Pointer \\[CP\\]\n\nContains pointer to the first PCI\n\nCapability Structure. This field is set\n\nby default to the value defined in the\n\nRTL file reg_defaults.h. It can be re-\n\nwritten independently for every\n\nFunction from the local management\n\nbus."]
pub type CpR = crate::FieldReader;
#[doc = "Field `R15` reader - Reserved \\[R15\\]\n\nReserved"]
pub type R15R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Capabilities Pointer \\[CP\\]\n\nContains pointer to the first PCI\n\nCapability Structure. This field is set\n\nby default to the value defined in the\n\nRTL file reg_defaults.h. It can be re-\n\nwritten independently for every\n\nFunction from the local management\n\nbus."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved \\[R15\\]\n\nReserved"]
    #[inline(always)]
    pub fn r15(&self) -> R15R {
        R15R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Capabilities Pointer\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_capabilities_pointer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfCapabilitiesPointerSpec;
impl crate::RegisterSpec for PciePfCapabilitiesPointerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_capabilities_pointer::R`](R) reader structure"]
impl crate::Readable for PciePfCapabilitiesPointerSpec {}
#[doc = "`reset()` method sets PCIE_PF_CAPABILITIES_POINTER to value 0x80"]
impl crate::Resettable for PciePfCapabilitiesPointerSpec {
    const RESET_VALUE: u32 = 0x80;
}

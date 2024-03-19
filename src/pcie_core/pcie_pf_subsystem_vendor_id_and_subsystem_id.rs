#[doc = "Register `PCIE_PF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID` reader"]
pub type R = crate::R<PciePfSubsystemVendorIdAndSubsystemIdSpec>;
#[doc = "Field `SVID` reader - Subsystem Vendor ID \\[SVID\\]\n\nSpecifies the Subsystem Vendor ID\n\nassigned by the PCI SIG to the\n\nmanufacturer of the device. Its value\n\ncomes from the Subsystem Vendor\n\nID Register in the local management\n\nregister block."]
pub type SvidR = crate::FieldReader<u16>;
#[doc = "Field `SID` reader - Subsystem ID \\[SID\\]\n\nSpecifies the Subsystem ID assigned\n\nby the manufacturer of the device.\n\nOn power-up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be re-\n\nwritten independently for each\n\nFunction from the local management\n\nbus."]
pub type SidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Subsystem Vendor ID \\[SVID\\]\n\nSpecifies the Subsystem Vendor ID\n\nassigned by the PCI SIG to the\n\nmanufacturer of the device. Its value\n\ncomes from the Subsystem Vendor\n\nID Register in the local management\n\nregister block."]
    #[inline(always)]
    pub fn svid(&self) -> SvidR {
        SvidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Subsystem ID \\[SID\\]\n\nSpecifies the Subsystem ID assigned\n\nby the manufacturer of the device.\n\nOn power-up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be re-\n\nwritten independently for each\n\nFunction from the local management\n\nbus."]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Subsystem Vendor ID and Subsystem ID Register\n\nSpecifies the Subsystem ID assigned\n\nby the manufacturer of the device.\n\nOn power-up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be re-\n\nwritten independently for each\n\nFunction from the local management\n\nbus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_subsystem_vendor_id_and_subsystem_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfSubsystemVendorIdAndSubsystemIdSpec;
impl crate::RegisterSpec for PciePfSubsystemVendorIdAndSubsystemIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_subsystem_vendor_id_and_subsystem_id::R`](R) reader structure"]
impl crate::Readable for PciePfSubsystemVendorIdAndSubsystemIdSpec {}
#[doc = "`reset()` method sets PCIE_PF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID to value 0x17cd"]
impl crate::Resettable for PciePfSubsystemVendorIdAndSubsystemIdSpec {
    const RESET_VALUE: u32 = 0x17cd;
}

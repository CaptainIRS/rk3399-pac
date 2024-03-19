#[doc = "Register `PCIE_VF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID` reader"]
pub type R = crate::R<PcieVfSubsystemVendorIdAndSubsystemIdSpec>;
#[doc = "Field `SVID` reader - Subsystem Vendor ID \\[SVID\\]\n\nSpecifies the Subsystem Vendor ID\n\nassigned by the PCI SIG to the\n\nmanufacturer of the device. Its\n\nvalue comes from the Subsystem\n\nVendor ID Register in the local\n\nmanagement register block."]
pub type SvidR = crate::FieldReader<u16>;
#[doc = "Field `SID` reader - Subsystem ID \\[SID\\]\n\nSpecifies the Subsystem ID assigned\n\nby the manufacturer of the device.\n\nThis field reflects the setting of the\n\ncorresponding register in the\n\nconfiguration space of the associated\n\nPhysical Function."]
pub type SidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Subsystem Vendor ID \\[SVID\\]\n\nSpecifies the Subsystem Vendor ID\n\nassigned by the PCI SIG to the\n\nmanufacturer of the device. Its\n\nvalue comes from the Subsystem\n\nVendor ID Register in the local\n\nmanagement register block."]
    #[inline(always)]
    pub fn svid(&self) -> SvidR {
        SvidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Subsystem ID \\[SID\\]\n\nSpecifies the Subsystem ID assigned\n\nby the manufacturer of the device.\n\nThis field reflects the setting of the\n\ncorresponding register in the\n\nconfiguration space of the associated\n\nPhysical Function."]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Subsystem Vendor ID and Subsystem ID Register\n\nSpecifies the Subsystem ID assigned\n\nby the manufacturer of the device.\n\nThis field reflects the setting of the\n\ncorresponding register in the\n\nconfiguration space of the associated\n\nPhysical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_subsystem_vendor_id_and_subsystem_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfSubsystemVendorIdAndSubsystemIdSpec;
impl crate::RegisterSpec for PcieVfSubsystemVendorIdAndSubsystemIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_subsystem_vendor_id_and_subsystem_id::R`](R) reader structure"]
impl crate::Readable for PcieVfSubsystemVendorIdAndSubsystemIdSpec {}
#[doc = "`reset()` method sets PCIE_VF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID to value 0x17cd"]
impl crate::Resettable for PcieVfSubsystemVendorIdAndSubsystemIdSpec {
    const RESET_VALUE: u32 = 0x17cd;
}

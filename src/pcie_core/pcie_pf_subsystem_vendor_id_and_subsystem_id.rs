#[doc = "Register `PCIE_PF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID` reader"]
pub type R = crate::R<PciePfSubsystemVendorIdAndSubsystemIdSpec>;
#[doc = "Field `SVID` reader - Subsystem Vendor ID \\[SVID\\]
Specifies the Subsystem Vendor ID assigned by the PCI SIG to the manufacturer of the device. Its value comes from the Subsystem Vendor ID Register in the local management register block."]
pub type SvidR = crate::FieldReader<u16>;
#[doc = "Field `SID` reader - Subsystem ID \\[SID\\]
Specifies the Subsystem ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be re- written independently for each Function from the local management bus."]
pub type SidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Subsystem Vendor ID \\[SVID\\]
Specifies the Subsystem Vendor ID assigned by the PCI SIG to the manufacturer of the device. Its value comes from the Subsystem Vendor ID Register in the local management register block."]
    #[inline(always)]
    pub fn svid(&self) -> SvidR {
        SvidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Subsystem ID \\[SID\\]
Specifies the Subsystem ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be re- written independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be re- written independently for each Function from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_subsystem_vendor_id_and_subsystem_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
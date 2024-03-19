#[doc = "Register `PCIE_VF_SUBSYSTEM_VENDOR_ID_AND_SUBSYSTEM_ID` reader"]
pub type R = crate::R<PcieVfSubsystemVendorIdAndSubsystemIdSpec>;
#[doc = "Field `SVID` reader - Subsystem Vendor ID \\[SVID\\]
Specifies the Subsystem Vendor ID assigned by the PCI SIG to the manufacturer of the device. Its value comes from the Subsystem Vendor ID Register in the local management register block."]
pub type SvidR = crate::FieldReader<u16>;
#[doc = "Field `SID` reader - Subsystem ID \\[SID\\]
Specifies the Subsystem ID assigned by the manufacturer of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
pub type SidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Subsystem Vendor ID \\[SVID\\]
Specifies the Subsystem Vendor ID assigned by the PCI SIG to the manufacturer of the device. Its value comes from the Subsystem Vendor ID Register in the local management register block."]
    #[inline(always)]
    pub fn svid(&self) -> SvidR {
        SvidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Subsystem ID \\[SID\\]
Specifies the Subsystem ID assigned by the manufacturer of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function."]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Subsystem Vendor ID and Subsystem ID Register Specifies the Subsystem ID assigned by the manufacturer of the device. This field reflects the setting of the corresponding register in the configuration space of the associated Physical Function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_subsystem_vendor_id_and_subsystem_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

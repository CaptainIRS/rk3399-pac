#[doc = "Register `PCIE_VF_VENDOR_ID_AND_DEVICE_ID` reader"]
pub type R = crate::R<PcieVfVendorIdAndDeviceIdSpec>;
#[doc = "Field `VID` reader - Vendor ID \\[VID\\]\n\nThis is the Vendor ID assigned by\n\nthe PCI SIG to the manufacturer\n\nof the device The Vendor ID is set in\n\nthe Vendor ID Register within the\n\nlocal management register block."]
pub type VidR = crate::FieldReader<u16>;
#[doc = "Field `DID` reader - Device ID \\[DID\\]\n\nDevice ID assigned by the\n\nmanufacturer of the device. On\n\npower-up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be\n\nwritten independently for each\n\nFunction from the local management\n\nbus."]
pub type DidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Vendor ID \\[VID\\]\n\nThis is the Vendor ID assigned by\n\nthe PCI SIG to the manufacturer\n\nof the device The Vendor ID is set in\n\nthe Vendor ID Register within the\n\nlocal management register block."]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Device ID \\[DID\\]\n\nDevice ID assigned by the\n\nmanufacturer of the device. On\n\npower-up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be\n\nwritten independently for each\n\nFunction from the local management\n\nbus."]
    #[inline(always)]
    pub fn did(&self) -> DidR {
        DidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Vendor ID and Device ID\n\nDevice ID assigned by the\n\nmanufacturer of the device. On\n\npower-up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be\n\nwritten independently for each\n\nFunction from the local management\n\nbus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_vendor_id_and_device_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfVendorIdAndDeviceIdSpec;
impl crate::RegisterSpec for PcieVfVendorIdAndDeviceIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_vendor_id_and_device_id::R`](R) reader structure"]
impl crate::Readable for PcieVfVendorIdAndDeviceIdSpec {}
#[doc = "`reset()` method sets PCIE_VF_VENDOR_ID_AND_DEVICE_ID to value 0xffff_ffff"]
impl crate::Resettable for PcieVfVendorIdAndDeviceIdSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

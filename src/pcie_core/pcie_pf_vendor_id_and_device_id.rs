#[doc = "Register `PCIE_PF_VENDOR_ID_AND_DEVICE_ID` reader"]
pub type R = crate::R<PciePfVendorIdAndDeviceIdSpec>;
#[doc = "Field `VID` reader - Vendor ID \\[VID\\]\n\nThis is the Vendor ID assigned by\n\nPCI SIG to the manufacturer of the\n\ndevice. The Vendor ID is set in the\n\nVendor ID Register within the local\n\nmanagement register block."]
pub type VidR = crate::FieldReader<u16>;
#[doc = "Field `DID` reader - Device ID \\[DID\\]\n\nDevice ID assigned by the\n\nmanufacturer of the device. On\n\npower-up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus."]
pub type DidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Vendor ID \\[VID\\]\n\nThis is the Vendor ID assigned by\n\nPCI SIG to the manufacturer of the\n\ndevice. The Vendor ID is set in the\n\nVendor ID Register within the local\n\nmanagement register block."]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Device ID \\[DID\\]\n\nDevice ID assigned by the\n\nmanufacturer of the device. On\n\npower-up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus."]
    #[inline(always)]
    pub fn did(&self) -> DidR {
        DidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Vendor ID and Device ID\n\nDevice ID assigned by the\n\nmanufacturer of the device. On\n\npower-up, the core sets it to the\n\nvalue defined in the RTL file\n\nreg_defaults.h. This field can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vendor_id_and_device_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfVendorIdAndDeviceIdSpec;
impl crate::RegisterSpec for PciePfVendorIdAndDeviceIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_vendor_id_and_device_id::R`](R) reader structure"]
impl crate::Readable for PciePfVendorIdAndDeviceIdSpec {}
#[doc = "`reset()` method sets PCIE_PF_VENDOR_ID_AND_DEVICE_ID to value 0x0100_17cd"]
impl crate::Resettable for PciePfVendorIdAndDeviceIdSpec {
    const RESET_VALUE: u32 = 0x0100_17cd;
}

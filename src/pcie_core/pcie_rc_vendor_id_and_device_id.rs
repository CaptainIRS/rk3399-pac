#[doc = "Register `PCIE_RC_VENDOR_ID_AND_DEVICE_ID` reader"]
pub type R = crate::R<PcieRcVendorIdAndDeviceIdSpec>;
#[doc = "Field `VID` reader - Vendor ID \\[VID\\]
This is the Vendor ID assigned by PCI SIG to the manufacturer of the device. The Vendor ID is set in the Vendor ID Register within the local management register block."]
pub type VidR = crate::FieldReader<u16>;
#[doc = "Field `DID` reader - Device ID \\[DID\\]
Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type DidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Vendor ID \\[VID\\]
This is the Vendor ID assigned by PCI SIG to the manufacturer of the device. The Vendor ID is set in the Vendor ID Register within the local management register block."]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Device ID \\[DID\\]
Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn did(&self) -> DidR {
        DidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Vendor ID and Device ID Device ID assigned by the manufacturer of the device. On power-up, the core sets it to the value defined in the RTL file reg_defaults.h. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_vendor_id_and_device_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcVendorIdAndDeviceIdSpec;
impl crate::RegisterSpec for PcieRcVendorIdAndDeviceIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_vendor_id_and_device_id::R`](R) reader structure"]
impl crate::Readable for PcieRcVendorIdAndDeviceIdSpec {}
#[doc = "`reset()` method sets PCIE_RC_VENDOR_ID_AND_DEVICE_ID to value 0x0100_17cd"]
impl crate::Resettable for PcieRcVendorIdAndDeviceIdSpec {
    const RESET_VALUE: u32 = 0x0100_17cd;
}

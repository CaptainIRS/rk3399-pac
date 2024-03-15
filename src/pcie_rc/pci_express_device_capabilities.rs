#[doc = "Register `PCI_EXPRESS_DEVICE_CAPABILITIES` reader"]
pub type R = crate::R<PciExpressDeviceCapabilitiesSpec>;
#[doc = "Field `MP` reader - Max Payload Size \\[MP\\]
Specifies maximum payload size supported by the device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type MpR = crate::FieldReader;
#[doc = "Field `PFS` reader - Phantom Functions Supported \\[PFS\\]
This field is used to extend the tag field by combining unused Function bits with the tag bits. This field is hardwired to 00 to disable this feature."]
pub type PfsR = crate::FieldReader;
#[doc = "Field `ETFS` reader - Extended Tag Field Supported \\[ETFS\\]
hard coded to zero ."]
pub type EtfsR = crate::BitReader;
#[doc = "Field `AL0L` reader - Acceptable L0S Latency \\[AL0L\\]
Specifies acceptable latency that the Endpoint can tolerate while transitioning from L0S to L0. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type Al0lR = crate::FieldReader;
#[doc = "Field `AL1L` reader - Acceptable L1 Latency \\[AL1L\\]
Specifies acceptable latency that the Endpoint can tolerate while transitioning from L1 to L0. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type Al1lR = crate::FieldReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader;
#[doc = "Field `RER` reader - Role- Based Error Reporting \\[RER\\]
Enables role-based error reporting. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RerR = crate::BitReader;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader;
#[doc = "Field `CSP` reader - Captured Slot Power Limit Value \\[CSP\\]
Specifies upper limit on power supplied by slot. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type CspR = crate::FieldReader;
#[doc = "Field `CPLS` reader - Captured Power Limit Scale \\[CPLS\\]
Specifies the scale used by Slot Power Limit Value. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type CplsR = crate::FieldReader;
#[doc = "Field `FLRC` reader - Function level reset capability \\[FLRC\\]
A value of 1b indicates the Function supports the optional Function Level Reset mechanism"]
pub type FlrcR = crate::BitReader;
#[doc = "Field `R5` reader - Reserved \\[R5\\]
Reserved"]
pub type R5R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Max Payload Size \\[MP\\]
Specifies maximum payload size supported by the device. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn mp(&self) -> MpR {
        MpR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Phantom Functions Supported \\[PFS\\]
This field is used to extend the tag field by combining unused Function bits with the tag bits. This field is hardwired to 00 to disable this feature."]
    #[inline(always)]
    pub fn pfs(&self) -> PfsR {
        PfsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Extended Tag Field Supported \\[ETFS\\]
hard coded to zero ."]
    #[inline(always)]
    pub fn etfs(&self) -> EtfsR {
        EtfsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Acceptable L0S Latency \\[AL0L\\]
Specifies acceptable latency that the Endpoint can tolerate while transitioning from L0S to L0. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn al0l(&self) -> Al0lR {
        Al0lR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Acceptable L1 Latency \\[AL1L\\]
Specifies acceptable latency that the Endpoint can tolerate while transitioning from L1 to L0. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn al1l(&self) -> Al1lR {
        Al1lR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Reserved \\[R3\\]
Reserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Role- Based Error Reporting \\[RER\\]
Enables role-based error reporting. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn rer(&self) -> RerR {
        RerR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:25 - Captured Slot Power Limit Value \\[CSP\\]
Specifies upper limit on power supplied by slot. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn csp(&self) -> CspR {
        CspR::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:27 - Captured Power Limit Scale \\[CPLS\\]
Specifies the scale used by Slot Power Limit Value. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn cpls(&self) -> CplsR {
        CplsR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Function level reset capability \\[FLRC\\]
A value of 1b indicates the Function supports the optional Function Level Reset mechanism"]
    #[inline(always)]
    pub fn flrc(&self) -> FlrcR {
        FlrcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Reserved \\[R5\\]
Reserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "PCI Express Device Capabilities Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciExpressDeviceCapabilitiesSpec;
impl crate::RegisterSpec for PciExpressDeviceCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pci_express_device_capabilities::R`](R) reader structure"]
impl crate::Readable for PciExpressDeviceCapabilitiesSpec {}
#[doc = "`reset()` method sets PCI_EXPRESS_DEVICE_CAPABILITIES to value 0x8001"]
impl crate::Resettable for PciExpressDeviceCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x8001;
}

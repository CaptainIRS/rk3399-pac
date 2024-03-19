#[doc = "Register `PCIE_VF_PCI_EXPRESS_CAPABILITY_LIST` reader"]
pub type R = crate::R<PcieVfPciExpressCapabilityListSpec>;
#[doc = "Field `CID` reader - Capability ID \\[CID\\]
Specifies Capability ID assigned by PCI SIG for this structure. This field is hardwired to 10 hex."]
pub type CidR = crate::FieldReader;
#[doc = "Field `NCP` reader - Next Capability Pointer \\[NCP\\]
Points to the next PCI capability structure. Set to 0 because this is the last capability structure."]
pub type NcpR = crate::FieldReader;
#[doc = "Field `CV` reader - Capability Version \\[CV\\]
Identifies the version number of the capability structure. The value depends on the value of the strap input PCIE_GENERATION_SEL If PCIE_GENERATION_SEL indicates Gen 2 or later generations, then the value is 2 else 1."]
pub type CvR = crate::FieldReader;
#[doc = "Field `DT` reader - Device Type \\[DT\\]
Indicates the type of device implementing this Function. This field is hardwired to 0 in the EP mode."]
pub type DtR = crate::FieldReader;
#[doc = "Field `SS` reader - Slot Status \\[SS\\]
Set to 1 when the link connected to a slot. Hardwired to 0."]
pub type SsR = crate::BitReader;
#[doc = "Field `IMN` reader - Interrupt Message Number \\[IMN\\]
Identifies the MSI or MSI-X interrupt vector for the interrupt message generated corresponding to the status bits in the Slot Status Register, Root Status Register, or this capability structure. This field must be defined based on the chosen interrupt mode - MSI or MSI-X. This field is hardwired to 0."]
pub type ImnR = crate::FieldReader;
#[doc = "Field `TRS` reader - TCS Routing Supported \\[TRS\\]
When set to 1, this bit indicates that the device supports routing of Trusted Configuration Requests. Not valid for Endpoints. Hardwired to 0."]
pub type TrsR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID\\]
Specifies Capability ID assigned by PCI SIG for this structure. This field is hardwired to 10 hex."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Next Capability Pointer \\[NCP\\]
Points to the next PCI capability structure. Set to 0 because this is the last capability structure."]
    #[inline(always)]
    pub fn ncp(&self) -> NcpR {
        NcpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Capability Version \\[CV\\]
Identifies the version number of the capability structure. The value depends on the value of the strap input PCIE_GENERATION_SEL If PCIE_GENERATION_SEL indicates Gen 2 or later generations, then the value is 2 else 1."]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Device Type \\[DT\\]
Indicates the type of device implementing this Function. This field is hardwired to 0 in the EP mode."]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Slot Status \\[SS\\]
Set to 1 when the link connected to a slot. Hardwired to 0."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Interrupt Message Number \\[IMN\\]
Identifies the MSI or MSI-X interrupt vector for the interrupt message generated corresponding to the status bits in the Slot Status Register, Root Status Register, or this capability structure. This field must be defined based on the chosen interrupt mode - MSI or MSI-X. This field is hardwired to 0."]
    #[inline(always)]
    pub fn imn(&self) -> ImnR {
        ImnR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - TCS Routing Supported \\[TRS\\]
When set to 1, this bit indicates that the device supports routing of Trusted Configuration Requests. Not valid for Endpoints. Hardwired to 0."]
    #[inline(always)]
    pub fn trs(&self) -> TrsR {
        TrsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "PCI Express Capability List Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_pci_express_capability_list::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfPciExpressCapabilityListSpec;
impl crate::RegisterSpec for PcieVfPciExpressCapabilityListSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_pci_express_capability_list::R`](R) reader structure"]
impl crate::Readable for PcieVfPciExpressCapabilityListSpec {}
#[doc = "`reset()` method sets PCIE_VF_PCI_EXPRESS_CAPABILITY_LIST to value 0x0002_0010"]
impl crate::Resettable for PcieVfPciExpressCapabilityListSpec {
    const RESET_VALUE: u32 = 0x0002_0010;
}

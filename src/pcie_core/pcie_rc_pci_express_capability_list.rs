#[doc = "Register `PCIE_RC_PCI_EXPRESS_CAPABILITY_LIST` reader"]
pub type R = crate::R<PcieRcPciExpressCapabilityListSpec>;
#[doc = "Field `CID` reader - Capability ID \\[CID\\]\n\nSpecifies Capability ID assigned by\n\nPCI SIG for this structure. This field\n\nis hardwired to 10 hex."]
pub type CidR = crate::FieldReader;
#[doc = "Field `NCP` reader - Next Capability Pointer \\[NCP\\]\n\nPoints to the next PCI capability\n\nstructure. Set to 0 because this is\n\nthe last capability structure."]
pub type NcpR = crate::FieldReader;
#[doc = "Field `PCV` reader - Capability Version \\[PCV\\]\n\nIdentifies the version number of the\n\ncapability structure. The value\n\ndepends on the value of the strap\n\ninput PCIE_GENERATION_SEL\n\nIf PCIE_GENERATION_SEL indicates\n\nGen 2 or later generations, then the\n\nvalue is 2 else 1. Can be modified\n\nusing local management interface\n\nafter asserting input signal\n\nMGMT_TYPE1_CONFIG_REG_ACCESS\n\nhigh."]
pub type PcvR = crate::FieldReader;
#[doc = "Field `DT` reader - Device Type \\[DT\\]\n\nIndicates the type of device\n\nimplementing this Function. This field\n\nis hardwired to 4 in the RP mode."]
pub type DtR = crate::FieldReader;
#[doc = "Field `SI` reader - Slot Implemented \\[SI\\]\n\nWhen Set, this bit indicates that the\n\nLink associated with this Port is\n\nconnected to a slot"]
pub type SiR = crate::BitReader;
#[doc = "Field `IMN` reader - Interrupt Message Number \\[IMN\\]\n\nIdentifies the MSI or MSI-X interrupt\n\nvector for the interrupt message\n\ngenerated corresponding to the\n\nstatus bits in the Slot Status\n\nRegister, Root Status Register, or this\n\ncapability structure. This field must\n\nbe defined based on the chosen\n\ninterrupt mode - MSI or MSI-X. This\n\nfield is hardwired to 0."]
pub type ImnR = crate::FieldReader;
#[doc = "Field `TRS` reader - TCS Routing Supported \\[TRS\\]\n\nWhen set to 1, this bit indicates that\n\nthe device supports routing of\n\nTrusted Configuration Requests. Not\n\nvalid for Endpoints. Hardwired to 0."]
pub type TrsR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID\\]\n\nSpecifies Capability ID assigned by\n\nPCI SIG for this structure. This field\n\nis hardwired to 10 hex."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Next Capability Pointer \\[NCP\\]\n\nPoints to the next PCI capability\n\nstructure. Set to 0 because this is\n\nthe last capability structure."]
    #[inline(always)]
    pub fn ncp(&self) -> NcpR {
        NcpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Capability Version \\[PCV\\]\n\nIdentifies the version number of the\n\ncapability structure. The value\n\ndepends on the value of the strap\n\ninput PCIE_GENERATION_SEL\n\nIf PCIE_GENERATION_SEL indicates\n\nGen 2 or later generations, then the\n\nvalue is 2 else 1. Can be modified\n\nusing local management interface\n\nafter asserting input signal\n\nMGMT_TYPE1_CONFIG_REG_ACCESS\n\nhigh."]
    #[inline(always)]
    pub fn pcv(&self) -> PcvR {
        PcvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Device Type \\[DT\\]\n\nIndicates the type of device\n\nimplementing this Function. This field\n\nis hardwired to 4 in the RP mode."]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Slot Implemented \\[SI\\]\n\nWhen Set, this bit indicates that the\n\nLink associated with this Port is\n\nconnected to a slot"]
    #[inline(always)]
    pub fn si(&self) -> SiR {
        SiR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Interrupt Message Number \\[IMN\\]\n\nIdentifies the MSI or MSI-X interrupt\n\nvector for the interrupt message\n\ngenerated corresponding to the\n\nstatus bits in the Slot Status\n\nRegister, Root Status Register, or this\n\ncapability structure. This field must\n\nbe defined based on the chosen\n\ninterrupt mode - MSI or MSI-X. This\n\nfield is hardwired to 0."]
    #[inline(always)]
    pub fn imn(&self) -> ImnR {
        ImnR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - TCS Routing Supported \\[TRS\\]\n\nWhen set to 1, this bit indicates that\n\nthe device supports routing of\n\nTrusted Configuration Requests. Not\n\nvalid for Endpoints. Hardwired to 0."]
    #[inline(always)]
    pub fn trs(&self) -> TrsR {
        TrsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "PCI Express Capability List Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_pci_express_capability_list::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcPciExpressCapabilityListSpec;
impl crate::RegisterSpec for PcieRcPciExpressCapabilityListSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_pci_express_capability_list::R`](R) reader structure"]
impl crate::Readable for PcieRcPciExpressCapabilityListSpec {}
#[doc = "`reset()` method sets PCIE_RC_PCI_EXPRESS_CAPABILITY_LIST to value 0x0142_0010"]
impl crate::Resettable for PcieRcPciExpressCapabilityListSpec {
    const RESET_VALUE: u32 = 0x0142_0010;
}

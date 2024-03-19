#[doc = "Register `PCIE_VF_POWER_MANAGEMENT_CAPABILITIES` reader"]
pub type R = crate::R<PcieVfPowerManagementCapabilitiesSpec>;
#[doc = "Field `CID` reader - Capability ID \\[CID\\]\n\nIdentifies that the capability\n\nstructure is for Power Management.\n\nThis field is set by default to 01 hex.\n\nIt can be re-written independently\n\nfor each Function from the local\n\nmanagement bus."]
pub type CidR = crate::FieldReader;
#[doc = "Field `CP` reader - Capabilities Pointer \\[CP\\]\n\nContains pointer to the next PCI\n\nCapability Structure. The core sets it\n\nto the value defined in the RTL file\n\nreg_defaults.h. By default, this\n\npoints to the MSI Capability\n\nStructure. This field can be re-\n\nwritten independently for each\n\nFunction from the local\n\nmanagement bus."]
pub type CpR = crate::FieldReader;
#[doc = "Field `VID` reader - Version ID \\[VID\\]\n\nIndicates the version of the PCI Bus\n\nPower Management Specifications\n\nthat the Function implements. This\n\nfield is set by default to 011 (Version\n\n1.2). It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
pub type VidR = crate::FieldReader;
#[doc = "Field `PC` reader - PME Clock \\[PC\\]\n\nNot applicable to PCI Express. This\n\nbit is hardwired to 0."]
pub type PcR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::BitReader;
#[doc = "Field `DSI` reader - Device Specific Initialization Bit \\[DSI\\]\n\nThis bit, when set, indicates that the\n\ndevice requires additional\n\nconfiguration steps beyond setting\n\nup its PCI configuration space, to\n\nbring it to the D0active state from\n\nthe D0uninitialized state. This bit is\n\nhardwired to 0."]
pub type DsiR = crate::BitReader;
#[doc = "Field `MCRAPS` reader - Max Current Required from Aux Power Supply \\[MCRAPS\\]\n\nSpecifies the maximum current\n\ndrawn by the device from the aux\n\npower source in the D3cold state.\n\nThis field is not implemented in\n\ndevices not supporting PME\n\nnotification when in the D3cold\n\nstate, and is therefore hardwired to\n\n0."]
pub type McrapsR = crate::FieldReader;
#[doc = "Field `D1S` reader - D1 Support \\[D1S\\]\n\nSet if the Function supports the D1\n\npower state. This bit can be modified\n\nfrom the local management bus by\n\nwriting into Function 0. All other\n\nFunctions assume the value set in\n\nFunction 0s Power Management\n\nCapabilities Register."]
pub type D1sR = crate::BitReader;
#[doc = "Field `D2S` reader - D2 Support \\[D2S\\]\n\nSet if the Function supports the D2\n\npower state. Currently hardwired to\n\n0."]
pub type D2sR = crate::BitReader;
#[doc = "Field `PSD0S` reader - PME Support for D0 State \\[PSD0S\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D0 state. This bit is set\n\nto 1 by default, but can be modified\n\nfrom the local management bus by\n\nwriting into Function 0. All other\n\nFunctions assume the value set in\n\nFunction 0s Power Management\n\nCapabilities Register."]
pub type Psd0sR = crate::BitReader;
#[doc = "Field `PSD1S` reader - PME Support for D1 State \\[PSD1S\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D1 state. This bit is set\n\nto 1 by default, but can be modified\n\nfrom the local management bus by\n\nwriting into Function 0. All other\n\nFunctions assume the value set in\n\nFunction 0s Power Management\n\nCapabilities Register."]
pub type Psd1sR = crate::BitReader;
#[doc = "Field `PSD2S` reader - PME Support for D2 State \\[PSD2S\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D2 state. This bit is\n\nhardwired to 0 because D2 state is\n\nnot supported."]
pub type Psd2sR = crate::BitReader;
#[doc = "Field `PSDHS` reader - PME Support for D3(hot) State \\[PSDHS\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D3hot state. This bit is\n\nset to 1 by default, but can be\n\nmodified from the local management\n\nbus by writing into Function 0. All\n\nother Functions assume the value\n\nset in Function 0s Power\n\nManagement Capabilities Register."]
pub type PsdhsR = crate::BitReader;
#[doc = "Field `PSDCS` reader - PME Support for D3(cold) State \\[PSDCS\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D3cold state. Because\n\nthe device does not have aux power,\n\nthis bit is hardwired to 0."]
pub type PsdcsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID\\]\n\nIdentifies that the capability\n\nstructure is for Power Management.\n\nThis field is set by default to 01 hex.\n\nIt can be re-written independently\n\nfor each Function from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Capabilities Pointer \\[CP\\]\n\nContains pointer to the next PCI\n\nCapability Structure. The core sets it\n\nto the value defined in the RTL file\n\nreg_defaults.h. By default, this\n\npoints to the MSI Capability\n\nStructure. This field can be re-\n\nwritten independently for each\n\nFunction from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Version ID \\[VID\\]\n\nIndicates the version of the PCI Bus\n\nPower Management Specifications\n\nthat the Function implements. This\n\nfield is set by default to 011 (Version\n\n1.2). It can be re-written\n\nindependently for each Function\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - PME Clock \\[PC\\]\n\nNot applicable to PCI Express. This\n\nbit is hardwired to 0."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Device Specific Initialization Bit \\[DSI\\]\n\nThis bit, when set, indicates that the\n\ndevice requires additional\n\nconfiguration steps beyond setting\n\nup its PCI configuration space, to\n\nbring it to the D0active state from\n\nthe D0uninitialized state. This bit is\n\nhardwired to 0."]
    #[inline(always)]
    pub fn dsi(&self) -> DsiR {
        DsiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:24 - Max Current Required from Aux Power Supply \\[MCRAPS\\]\n\nSpecifies the maximum current\n\ndrawn by the device from the aux\n\npower source in the D3cold state.\n\nThis field is not implemented in\n\ndevices not supporting PME\n\nnotification when in the D3cold\n\nstate, and is therefore hardwired to\n\n0."]
    #[inline(always)]
    pub fn mcraps(&self) -> McrapsR {
        McrapsR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - D1 Support \\[D1S\\]\n\nSet if the Function supports the D1\n\npower state. This bit can be modified\n\nfrom the local management bus by\n\nwriting into Function 0. All other\n\nFunctions assume the value set in\n\nFunction 0s Power Management\n\nCapabilities Register."]
    #[inline(always)]
    pub fn d1s(&self) -> D1sR {
        D1sR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - D2 Support \\[D2S\\]\n\nSet if the Function supports the D2\n\npower state. Currently hardwired to\n\n0."]
    #[inline(always)]
    pub fn d2s(&self) -> D2sR {
        D2sR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PME Support for D0 State \\[PSD0S\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D0 state. This bit is set\n\nto 1 by default, but can be modified\n\nfrom the local management bus by\n\nwriting into Function 0. All other\n\nFunctions assume the value set in\n\nFunction 0s Power Management\n\nCapabilities Register."]
    #[inline(always)]
    pub fn psd0s(&self) -> Psd0sR {
        Psd0sR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PME Support for D1 State \\[PSD1S\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D1 state. This bit is set\n\nto 1 by default, but can be modified\n\nfrom the local management bus by\n\nwriting into Function 0. All other\n\nFunctions assume the value set in\n\nFunction 0s Power Management\n\nCapabilities Register."]
    #[inline(always)]
    pub fn psd1s(&self) -> Psd1sR {
        Psd1sR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PME Support for D2 State \\[PSD2S\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D2 state. This bit is\n\nhardwired to 0 because D2 state is\n\nnot supported."]
    #[inline(always)]
    pub fn psd2s(&self) -> Psd2sR {
        Psd2sR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PME Support for D3(hot) State \\[PSDHS\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D3hot state. This bit is\n\nset to 1 by default, but can be\n\nmodified from the local management\n\nbus by writing into Function 0. All\n\nother Functions assume the value\n\nset in Function 0s Power\n\nManagement Capabilities Register."]
    #[inline(always)]
    pub fn psdhs(&self) -> PsdhsR {
        PsdhsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PME Support for D3(cold) State \\[PSDCS\\]\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D3cold state. Because\n\nthe device does not have aux power,\n\nthis bit is hardwired to 0."]
    #[inline(always)]
    pub fn psdcs(&self) -> PsdcsR {
        PsdcsR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Power Management Capabilities Register\n\nIndicates whether the Function is\n\ncapable of sending PME messages\n\nwhen in the D3cold state. Because\n\nthe device does not have aux power,\n\nthis bit is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_power_management_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfPowerManagementCapabilitiesSpec;
impl crate::RegisterSpec for PcieVfPowerManagementCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_power_management_capabilities::R`](R) reader structure"]
impl crate::Readable for PcieVfPowerManagementCapabilitiesSpec {}
#[doc = "`reset()` method sets PCIE_VF_POWER_MANAGEMENT_CAPABILITIES to value 0x5a03_9001"]
impl crate::Resettable for PcieVfPowerManagementCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x5a03_9001;
}

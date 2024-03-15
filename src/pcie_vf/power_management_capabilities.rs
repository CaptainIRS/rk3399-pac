#[doc = "Register `POWER_MANAGEMENT_CAPABILITIES` reader"]
pub type R = crate::R<PowerManagementCapabilitiesSpec>;
#[doc = "Field `CID` reader - Capability ID \\[CID\\]
Identifies that the capability structure is for Power Management. This field is set by default to 01 hex. It can be re-written independently for each Function from the local management bus."]
pub type CidR = crate::FieldReader;
#[doc = "Field `CP` reader - Capabilities Pointer \\[CP\\]
Contains pointer to the next PCI Capability Structure. The core sets it to the value defined in the RTL file reg_defaults.h. By default, this points to the MSI Capability Structure. This field can be re- written independently for each Function from the local management bus."]
pub type CpR = crate::FieldReader;
#[doc = "Field `VID` reader - Version ID \\[VID\\]
Indicates the version of the PCI Bus Power Management Specifications that the Function implements. This field is set by default to 011 (Version 1.2). It can be re-written independently for each Function from the local management bus."]
pub type VidR = crate::FieldReader;
#[doc = "Field `PC` reader - PME Clock \\[PC\\]
Not applicable to PCI Express. This bit is hardwired to 0."]
pub type PcR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::BitReader;
#[doc = "Field `DSI` reader - Device Specific Initialization Bit \\[DSI\\]
This bit, when set, indicates that the device requires additional configuration steps beyond setting up its PCI configuration space, to bring it to the D0active state from the D0uninitialized state. This bit is hardwired to 0."]
pub type DsiR = crate::BitReader;
#[doc = "Field `MCRAPS` reader - Max Current Required from Aux Power Supply \\[MCRAPS\\]
Specifies the maximum current drawn by the device from the aux power source in the D3cold state. This field is not implemented in devices not supporting PME notification when in the D3cold state, and is therefore hardwired to 0."]
pub type McrapsR = crate::FieldReader;
#[doc = "Field `D1S` reader - D1 Support \\[D1S\\]
Set if the Function supports the D1 power state. This bit can be modified from the local management bus by writing into Function 0. All other Functions assume the value set in Function 0s Power Management Capabilities Register."]
pub type D1sR = crate::BitReader;
#[doc = "Field `D2S` reader - D2 Support \\[D2S\\]
Set if the Function supports the D2 power state. Currently hardwired to 0."]
pub type D2sR = crate::BitReader;
#[doc = "Field `PSD0S` reader - PME Support for D0 State \\[PSD0S\\]
Indicates whether the Function is capable of sending PME messages when in the D0 state. This bit is set to 1 by default, but can be modified from the local management bus by writing into Function 0. All other Functions assume the value set in Function 0s Power Management Capabilities Register."]
pub type Psd0sR = crate::BitReader;
#[doc = "Field `PSD1S` reader - PME Support for D1 State \\[PSD1S\\]
Indicates whether the Function is capable of sending PME messages when in the D1 state. This bit is set to 1 by default, but can be modified from the local management bus by writing into Function 0. All other Functions assume the value set in Function 0s Power Management Capabilities Register."]
pub type Psd1sR = crate::BitReader;
#[doc = "Field `PSD2S` reader - PME Support for D2 State \\[PSD2S\\]
Indicates whether the Function is capable of sending PME messages when in the D2 state. This bit is hardwired to 0 because D2 state is not supported."]
pub type Psd2sR = crate::BitReader;
#[doc = "Field `PSDHS` reader - PME Support for D3(hot) State \\[PSDHS\\]
Indicates whether the Function is capable of sending PME messages when in the D3hot state. This bit is set to 1 by default, but can be modified from the local management bus by writing into Function 0. All other Functions assume the value set in Function 0s Power Management Capabilities Register."]
pub type PsdhsR = crate::BitReader;
#[doc = "Field `PSDCS` reader - PME Support for D3(cold) State \\[PSDCS\\]
Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0."]
pub type PsdcsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID\\]
Identifies that the capability structure is for Power Management. This field is set by default to 01 hex. It can be re-written independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Capabilities Pointer \\[CP\\]
Contains pointer to the next PCI Capability Structure. The core sets it to the value defined in the RTL file reg_defaults.h. By default, this points to the MSI Capability Structure. This field can be re- written independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Version ID \\[VID\\]
Indicates the version of the PCI Bus Power Management Specifications that the Function implements. This field is set by default to 011 (Version 1.2). It can be re-written independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - PME Clock \\[PC\\]
Not applicable to PCI Express. This bit is hardwired to 0."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Device Specific Initialization Bit \\[DSI\\]
This bit, when set, indicates that the device requires additional configuration steps beyond setting up its PCI configuration space, to bring it to the D0active state from the D0uninitialized state. This bit is hardwired to 0."]
    #[inline(always)]
    pub fn dsi(&self) -> DsiR {
        DsiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:24 - Max Current Required from Aux Power Supply \\[MCRAPS\\]
Specifies the maximum current drawn by the device from the aux power source in the D3cold state. This field is not implemented in devices not supporting PME notification when in the D3cold state, and is therefore hardwired to 0."]
    #[inline(always)]
    pub fn mcraps(&self) -> McrapsR {
        McrapsR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - D1 Support \\[D1S\\]
Set if the Function supports the D1 power state. This bit can be modified from the local management bus by writing into Function 0. All other Functions assume the value set in Function 0s Power Management Capabilities Register."]
    #[inline(always)]
    pub fn d1s(&self) -> D1sR {
        D1sR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - D2 Support \\[D2S\\]
Set if the Function supports the D2 power state. Currently hardwired to 0."]
    #[inline(always)]
    pub fn d2s(&self) -> D2sR {
        D2sR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PME Support for D0 State \\[PSD0S\\]
Indicates whether the Function is capable of sending PME messages when in the D0 state. This bit is set to 1 by default, but can be modified from the local management bus by writing into Function 0. All other Functions assume the value set in Function 0s Power Management Capabilities Register."]
    #[inline(always)]
    pub fn psd0s(&self) -> Psd0sR {
        Psd0sR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PME Support for D1 State \\[PSD1S\\]
Indicates whether the Function is capable of sending PME messages when in the D1 state. This bit is set to 1 by default, but can be modified from the local management bus by writing into Function 0. All other Functions assume the value set in Function 0s Power Management Capabilities Register."]
    #[inline(always)]
    pub fn psd1s(&self) -> Psd1sR {
        Psd1sR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PME Support for D2 State \\[PSD2S\\]
Indicates whether the Function is capable of sending PME messages when in the D2 state. This bit is hardwired to 0 because D2 state is not supported."]
    #[inline(always)]
    pub fn psd2s(&self) -> Psd2sR {
        Psd2sR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PME Support for D3(hot) State \\[PSDHS\\]
Indicates whether the Function is capable of sending PME messages when in the D3hot state. This bit is set to 1 by default, but can be modified from the local management bus by writing into Function 0. All other Functions assume the value set in Function 0s Power Management Capabilities Register."]
    #[inline(always)]
    pub fn psdhs(&self) -> PsdhsR {
        PsdhsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PME Support for D3(cold) State \\[PSDCS\\]
Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0."]
    #[inline(always)]
    pub fn psdcs(&self) -> PsdcsR {
        PsdcsR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Power Management Capabilities Register Indicates whether the Function is capable of sending PME messages when in the D3cold state. Because the device does not have aux power, this bit is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_management_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerManagementCapabilitiesSpec;
impl crate::RegisterSpec for PowerManagementCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_management_capabilities::R`](R) reader structure"]
impl crate::Readable for PowerManagementCapabilitiesSpec {}
#[doc = "`reset()` method sets POWER_MANAGEMENT_CAPABILITIES to value 0x5a03_9001"]
impl crate::Resettable for PowerManagementCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x5a03_9001;
}

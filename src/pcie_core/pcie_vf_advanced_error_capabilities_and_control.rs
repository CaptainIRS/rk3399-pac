#[doc = "Register `PCIE_VF_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL` reader"]
pub type R = crate::R<PcieVfAdvancedErrorCapabilitiesAndControlSpec>;
#[doc = "Field `FER` reader - First Error Pointer \\[FER\\]\n\nThis is a 5-bit pointer to the bit\n\nposition in the Uncorrectable Error\n\nStatus Register corresponding to the\n\nerror that was detected first. When\n\nthere are multiple bits set in the\n\nUncorrectable Error Status Register,\n\nthis field informs the software which\n\nerror was observed first. To prevent\n\nthe field from being overwritten\n\nbefore software was able to read it,\n\nthis field is not updated while the\n\nstatus bit pointed by it in the\n\nUncorrectable Error Status Register\n\nremains set. After the software\n\nclears this status bit, a subsequent\n\nerror condition that sets any bit in\n\nthe Uncorrectable Error Status\n\nRegister will update the First Error\n\nPointer. Any uncorrectable error\n\ntype, including the special cases\n\nwhere the error is reported using an\n\nERR_COR message, will set the First\n\nError Pointer (assuming the software\n\nhas reset the error pointed by it in\n\nthe Uncorrectable Error Status\n\nRegister). STICKY."]
pub type FerR = crate::FieldReader;
#[doc = "Field `EGC` reader - ECRC Generation Capability \\[EGC\\]\n\nThis read-only bit indicates to the\n\nsoftware that the device is capable\n\nof generating ECRC in packets\n\ntransmitted on the link. This bit is\n\nhardwired to 0. The setting of the\n\ncorresponding bit in the Advanced\n\nError Capabilities and Control\n\nRegister of PF 0 applies to all Virtual\n\nFunctions."]
pub type EgcR = crate::BitReader;
#[doc = "Field `EEG` reader - Enable ECRC Generation \\[EEG\\]\n\nEnables the ECRC generation on the\n\ntransmit side of the core. This bit is\n\nhardwired to 0. The setting of the\n\ncorresponding bit in the Advanced\n\nError Capabilities and Control\n\nRegister of PF0 applies to all Virtual\n\nFunctions."]
pub type EegR = crate::BitReader;
#[doc = "Field `ECCAP` reader - ECRC Check Capability \\[ECCAP\\]\n\nThis read-only bit indicates to the\n\nsoftware that the device is capable\n\nof checking ECRC in packets received\n\nfrom the link. This bit is hardwired\n\nto0. This setting of the\n\ncorresponding bit in the Advanced\n\nError Capabilities and Control\n\nRegister of PF 0 applies to all Virtual\n\nFunctions."]
pub type EccapR = crate::BitReader;
#[doc = "Field `ECC` reader - Enable ECRC Check \\[ECC\\]\n\nSetting this bit enables ECRC\n\nchecking on the receive side of the\n\ncore. This bit is hardwired to 0. The\n\nsetting of the corresponding bit in\n\nthe Advanced Error Capabilities and\n\nControl Register of PF 0 applies to all\n\nVirtual Functions."]
pub type EccR = crate::BitReader;
#[doc = "Field `MHRC` reader - Multiple Header Recording Capable \\[MHRC\\]\n\nThis bit is set when the Function has\n\nthe capability to log more than one\n\nerror header in its Header Log\n\nRegisters. It is hardwired to 0."]
pub type MhrcR = crate::BitReader;
#[doc = "Field `MHRE` reader - Multiple Header Recording Enable \\[MHRE\\]\n\nSetting this bit enables the Function\n\nto log multiple error headers in its\n\nHeader Log Registers. It is hardwired\n\nto 0"]
pub type MhreR = crate::BitReader;
#[doc = "Field `R18` reader - Reserved \\[R18\\]\n\nReserved"]
pub type R18R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - First Error Pointer \\[FER\\]\n\nThis is a 5-bit pointer to the bit\n\nposition in the Uncorrectable Error\n\nStatus Register corresponding to the\n\nerror that was detected first. When\n\nthere are multiple bits set in the\n\nUncorrectable Error Status Register,\n\nthis field informs the software which\n\nerror was observed first. To prevent\n\nthe field from being overwritten\n\nbefore software was able to read it,\n\nthis field is not updated while the\n\nstatus bit pointed by it in the\n\nUncorrectable Error Status Register\n\nremains set. After the software\n\nclears this status bit, a subsequent\n\nerror condition that sets any bit in\n\nthe Uncorrectable Error Status\n\nRegister will update the First Error\n\nPointer. Any uncorrectable error\n\ntype, including the special cases\n\nwhere the error is reported using an\n\nERR_COR message, will set the First\n\nError Pointer (assuming the software\n\nhas reset the error pointed by it in\n\nthe Uncorrectable Error Status\n\nRegister). STICKY."]
    #[inline(always)]
    pub fn fer(&self) -> FerR {
        FerR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - ECRC Generation Capability \\[EGC\\]\n\nThis read-only bit indicates to the\n\nsoftware that the device is capable\n\nof generating ECRC in packets\n\ntransmitted on the link. This bit is\n\nhardwired to 0. The setting of the\n\ncorresponding bit in the Advanced\n\nError Capabilities and Control\n\nRegister of PF 0 applies to all Virtual\n\nFunctions."]
    #[inline(always)]
    pub fn egc(&self) -> EgcR {
        EgcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable ECRC Generation \\[EEG\\]\n\nEnables the ECRC generation on the\n\ntransmit side of the core. This bit is\n\nhardwired to 0. The setting of the\n\ncorresponding bit in the Advanced\n\nError Capabilities and Control\n\nRegister of PF0 applies to all Virtual\n\nFunctions."]
    #[inline(always)]
    pub fn eeg(&self) -> EegR {
        EegR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ECRC Check Capability \\[ECCAP\\]\n\nThis read-only bit indicates to the\n\nsoftware that the device is capable\n\nof checking ECRC in packets received\n\nfrom the link. This bit is hardwired\n\nto0. This setting of the\n\ncorresponding bit in the Advanced\n\nError Capabilities and Control\n\nRegister of PF 0 applies to all Virtual\n\nFunctions."]
    #[inline(always)]
    pub fn eccap(&self) -> EccapR {
        EccapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable ECRC Check \\[ECC\\]\n\nSetting this bit enables ECRC\n\nchecking on the receive side of the\n\ncore. This bit is hardwired to 0. The\n\nsetting of the corresponding bit in\n\nthe Advanced Error Capabilities and\n\nControl Register of PF 0 applies to all\n\nVirtual Functions."]
    #[inline(always)]
    pub fn ecc(&self) -> EccR {
        EccR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multiple Header Recording Capable \\[MHRC\\]\n\nThis bit is set when the Function has\n\nthe capability to log more than one\n\nerror header in its Header Log\n\nRegisters. It is hardwired to 0."]
    #[inline(always)]
    pub fn mhrc(&self) -> MhrcR {
        MhrcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multiple Header Recording Enable \\[MHRE\\]\n\nSetting this bit enables the Function\n\nto log multiple error headers in its\n\nHeader Log Registers. It is hardwired\n\nto 0"]
    #[inline(always)]
    pub fn mhre(&self) -> MhreR {
        MhreR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - Reserved \\[R18\\]\n\nReserved"]
    #[inline(always)]
    pub fn r18(&self) -> R18R {
        R18R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
#[doc = "Advanced Error Capabilities and Control Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_advanced_error_capabilities_and_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfAdvancedErrorCapabilitiesAndControlSpec;
impl crate::RegisterSpec for PcieVfAdvancedErrorCapabilitiesAndControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_advanced_error_capabilities_and_control::R`](R) reader structure"]
impl crate::Readable for PcieVfAdvancedErrorCapabilitiesAndControlSpec {}
#[doc = "`reset()` method sets PCIE_VF_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL to value 0"]
impl crate::Resettable for PcieVfAdvancedErrorCapabilitiesAndControlSpec {
    const RESET_VALUE: u32 = 0;
}

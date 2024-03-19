#[doc = "Register `PCIE_RC_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL` reader"]
pub type R = crate::R<PcieRcAdvancedErrorCapabilitiesAndControlSpec>;
#[doc = "Register `PCIE_RC_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL` writer"]
pub type W = crate::W<PcieRcAdvancedErrorCapabilitiesAndControlSpec>;
#[doc = "Field `FEP` reader - First Error Pointer \\[FEP\\]\n\nThis is a 5-bit pointer to the bit\n\nposition in the Uncorrectable Error\n\nStatus Register corresponding to the\n\nerror that was detected first. When\n\nthere are multiple bits set in the\n\nUncorrectable Error Status Register,\n\nthis field informs the software which\n\nerror was observed first. To prevent\n\nthe field from being overwritten\n\nbefore the software is able to read it,\n\nthis field is not updated while the\n\nstatus bit it points to in the\n\nUncorrectable Error Status Register\n\nremains set. After the software\n\nclears this status bit, a subsequent\n\nerror condition that sets any bit in\n\nthe Uncorrectable Error Status\n\nRegister will update the First Error\n\nPointer. Any uncorrectable error\n\ntype, including the special cases\n\nwhere the error is reported using an\n\nERR_COR message, will set the First\n\nError Pointer (assuming the software\n\nhas reset the error pointed by it in\n\nthe Uncorrectable Error Status\n\nRegister). STICKY."]
pub type FepR = crate::FieldReader;
#[doc = "Field `EGC` reader - ECRC Generation Capability \\[EGC\\]\n\nThis read-only bit indicates to the\n\nsoftware that the device is capable\n\nof generating ECRC in packets\n\ntransmitted on the link."]
pub type EgcR = crate::BitReader;
#[doc = "Field `EEG` reader - Enable ECRC Generation \\[EEG\\]\n\nSetting this bit enables the ECRC\n\ngeneration on the transmit side of\n\nthe core. This bit is writable from the\n\nlocal management bus. STICKY."]
pub type EegR = crate::BitReader;
#[doc = "Field `EEG` writer - Enable ECRC Generation \\[EEG\\]\n\nSetting this bit enables the ECRC\n\ngeneration on the transmit side of\n\nthe core. This bit is writable from the\n\nlocal management bus. STICKY."]
pub type EegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC` reader - ECRC Check Capability \\[ECC\\]\n\nThis read-only bit indicates to the\n\nsoftware that the device is capable\n\nof checking ECRC in packets received\n\nfrom the link."]
pub type EccR = crate::BitReader;
#[doc = "Field `EEC` reader - Enable ECRC Check \\[EEC\\]\n\nSetting this bit enables ECRC\n\nchecking on the receive side of the\n\ncore. This bit is writable from the\n\nlocal management bus. STICKY."]
pub type EecR = crate::BitReader;
#[doc = "Field `EEC` writer - Enable ECRC Check \\[EEC\\]\n\nSetting this bit enables ECRC\n\nchecking on the receive side of the\n\ncore. This bit is writable from the\n\nlocal management bus. STICKY."]
pub type EecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MHRC` reader - Multiple Header Recording Capable \\[MHRC\\]\n\nThis bit is set when the RC has the\n\ncapability to log more than one error\n\nheader in its Header Log Registers.\n\nIt is hardwired to 0."]
pub type MhrcR = crate::BitReader;
#[doc = "Field `MHRE` reader - Multiple Header Recording Enable \\[MHRE\\]\n\nSetting this bit enables the RC to log\n\nmultiple error headers in its Header\n\nLog Registers. It is hardwired to 0."]
pub type MhreR = crate::BitReader;
#[doc = "Field `R43` reader - Reserved \\[R43\\]\n\nReserved"]
pub type R43R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - First Error Pointer \\[FEP\\]\n\nThis is a 5-bit pointer to the bit\n\nposition in the Uncorrectable Error\n\nStatus Register corresponding to the\n\nerror that was detected first. When\n\nthere are multiple bits set in the\n\nUncorrectable Error Status Register,\n\nthis field informs the software which\n\nerror was observed first. To prevent\n\nthe field from being overwritten\n\nbefore the software is able to read it,\n\nthis field is not updated while the\n\nstatus bit it points to in the\n\nUncorrectable Error Status Register\n\nremains set. After the software\n\nclears this status bit, a subsequent\n\nerror condition that sets any bit in\n\nthe Uncorrectable Error Status\n\nRegister will update the First Error\n\nPointer. Any uncorrectable error\n\ntype, including the special cases\n\nwhere the error is reported using an\n\nERR_COR message, will set the First\n\nError Pointer (assuming the software\n\nhas reset the error pointed by it in\n\nthe Uncorrectable Error Status\n\nRegister). STICKY."]
    #[inline(always)]
    pub fn fep(&self) -> FepR {
        FepR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - ECRC Generation Capability \\[EGC\\]\n\nThis read-only bit indicates to the\n\nsoftware that the device is capable\n\nof generating ECRC in packets\n\ntransmitted on the link."]
    #[inline(always)]
    pub fn egc(&self) -> EgcR {
        EgcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable ECRC Generation \\[EEG\\]\n\nSetting this bit enables the ECRC\n\ngeneration on the transmit side of\n\nthe core. This bit is writable from the\n\nlocal management bus. STICKY."]
    #[inline(always)]
    pub fn eeg(&self) -> EegR {
        EegR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ECRC Check Capability \\[ECC\\]\n\nThis read-only bit indicates to the\n\nsoftware that the device is capable\n\nof checking ECRC in packets received\n\nfrom the link."]
    #[inline(always)]
    pub fn ecc(&self) -> EccR {
        EccR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable ECRC Check \\[EEC\\]\n\nSetting this bit enables ECRC\n\nchecking on the receive side of the\n\ncore. This bit is writable from the\n\nlocal management bus. STICKY."]
    #[inline(always)]
    pub fn eec(&self) -> EecR {
        EecR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multiple Header Recording Capable \\[MHRC\\]\n\nThis bit is set when the RC has the\n\ncapability to log more than one error\n\nheader in its Header Log Registers.\n\nIt is hardwired to 0."]
    #[inline(always)]
    pub fn mhrc(&self) -> MhrcR {
        MhrcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multiple Header Recording Enable \\[MHRE\\]\n\nSetting this bit enables the RC to log\n\nmultiple error headers in its Header\n\nLog Registers. It is hardwired to 0."]
    #[inline(always)]
    pub fn mhre(&self) -> MhreR {
        MhreR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - Reserved \\[R43\\]\n\nReserved"]
    #[inline(always)]
    pub fn r43(&self) -> R43R {
        R43R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 6 - Enable ECRC Generation \\[EEG\\]\n\nSetting this bit enables the ECRC\n\ngeneration on the transmit side of\n\nthe core. This bit is writable from the\n\nlocal management bus. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn eeg(&mut self) -> EegW<PcieRcAdvancedErrorCapabilitiesAndControlSpec> {
        EegW::new(self, 6)
    }
    #[doc = "Bit 8 - Enable ECRC Check \\[EEC\\]\n\nSetting this bit enables ECRC\n\nchecking on the receive side of the\n\ncore. This bit is writable from the\n\nlocal management bus. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn eec(&mut self) -> EecW<PcieRcAdvancedErrorCapabilitiesAndControlSpec> {
        EecW::new(self, 8)
    }
}
#[doc = "Advanced Error Capabilities and Control Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_advanced_error_capabilities_and_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_advanced_error_capabilities_and_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcAdvancedErrorCapabilitiesAndControlSpec;
impl crate::RegisterSpec for PcieRcAdvancedErrorCapabilitiesAndControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_advanced_error_capabilities_and_control::R`](R) reader structure"]
impl crate::Readable for PcieRcAdvancedErrorCapabilitiesAndControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_advanced_error_capabilities_and_control::W`](W) writer structure"]
impl crate::Writable for PcieRcAdvancedErrorCapabilitiesAndControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_ADVANCED_ERROR_CAPABILITIES_AND_CONTROL to value 0xa0"]
impl crate::Resettable for PcieRcAdvancedErrorCapabilitiesAndControlSpec {
    const RESET_VALUE: u32 = 0xa0;
}

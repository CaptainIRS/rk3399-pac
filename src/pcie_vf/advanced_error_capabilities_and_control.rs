#[doc = "Register `ADVANCED_ERROR_CAPABILITIES_AND_CONTROL` reader"]
pub type R = crate::R<AdvancedErrorCapabilitiesAndControlSpec>;
#[doc = "Field `FER` reader - First Error Pointer \\[FER\\]
This is a 5-bit pointer to the bit position in the Uncorrectable Error Status Register corresponding to the error that was detected first. When there are multiple bits set in the Uncorrectable Error Status Register, this field informs the software which error was observed first. To prevent the field from being overwritten before software was able to read it, this field is not updated while the status bit pointed by it in the Uncorrectable Error Status Register remains set. After the software clears this status bit, a subsequent error condition that sets any bit in the Uncorrectable Error Status Register will update the First Error Pointer. Any uncorrectable error type, including the special cases where the error is reported using an ERR_COR message, will set the First Error Pointer (assuming the software has reset the error pointed by it in the Uncorrectable Error Status Register). STICKY."]
pub type FerR = crate::FieldReader;
#[doc = "Field `EGC` reader - ECRC Generation Capability \\[EGC\\]
This read-only bit indicates to the software that the device is capable of generating ECRC in packets transmitted on the link. This bit is hardwired to 0. The setting of the corresponding bit in the Advanced Error Capabilities and Control Register of PF 0 applies to all Virtual Functions."]
pub type EgcR = crate::BitReader;
#[doc = "Field `EEG` reader - Enable ECRC Generation \\[EEG\\]
Enables the ECRC generation on the transmit side of the core. This bit is hardwired to 0. The setting of the corresponding bit in the Advanced Error Capabilities and Control Register of PF0 applies to all Virtual Functions."]
pub type EegR = crate::BitReader;
#[doc = "Field `ECCAP` reader - ECRC Check Capability \\[ECCAP\\]
This read-only bit indicates to the software that the device is capable of checking ECRC in packets received from the link. This bit is hardwired to0. This setting of the corresponding bit in the Advanced Error Capabilities and Control Register of PF 0 applies to all Virtual Functions."]
pub type EccapR = crate::BitReader;
#[doc = "Field `ECC` reader - Enable ECRC Check \\[ECC\\]
Setting this bit enables ECRC checking on the receive side of the core. This bit is hardwired to 0. The setting of the corresponding bit in the Advanced Error Capabilities and Control Register of PF 0 applies to all Virtual Functions."]
pub type EccR = crate::BitReader;
#[doc = "Field `MHRC` reader - Multiple Header Recording Capable \\[MHRC\\]
This bit is set when the Function has the capability to log more than one error header in its Header Log Registers. It is hardwired to 0."]
pub type MhrcR = crate::BitReader;
#[doc = "Field `MHRE` reader - Multiple Header Recording Enable \\[MHRE\\]
Setting this bit enables the Function to log multiple error headers in its Header Log Registers. It is hardwired to 0"]
pub type MhreR = crate::BitReader;
#[doc = "Field `R18` reader - Reserved \\[R18\\]
Reserved"]
pub type R18R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - First Error Pointer \\[FER\\]
This is a 5-bit pointer to the bit position in the Uncorrectable Error Status Register corresponding to the error that was detected first. When there are multiple bits set in the Uncorrectable Error Status Register, this field informs the software which error was observed first. To prevent the field from being overwritten before software was able to read it, this field is not updated while the status bit pointed by it in the Uncorrectable Error Status Register remains set. After the software clears this status bit, a subsequent error condition that sets any bit in the Uncorrectable Error Status Register will update the First Error Pointer. Any uncorrectable error type, including the special cases where the error is reported using an ERR_COR message, will set the First Error Pointer (assuming the software has reset the error pointed by it in the Uncorrectable Error Status Register). STICKY."]
    #[inline(always)]
    pub fn fer(&self) -> FerR {
        FerR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - ECRC Generation Capability \\[EGC\\]
This read-only bit indicates to the software that the device is capable of generating ECRC in packets transmitted on the link. This bit is hardwired to 0. The setting of the corresponding bit in the Advanced Error Capabilities and Control Register of PF 0 applies to all Virtual Functions."]
    #[inline(always)]
    pub fn egc(&self) -> EgcR {
        EgcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable ECRC Generation \\[EEG\\]
Enables the ECRC generation on the transmit side of the core. This bit is hardwired to 0. The setting of the corresponding bit in the Advanced Error Capabilities and Control Register of PF0 applies to all Virtual Functions."]
    #[inline(always)]
    pub fn eeg(&self) -> EegR {
        EegR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ECRC Check Capability \\[ECCAP\\]
This read-only bit indicates to the software that the device is capable of checking ECRC in packets received from the link. This bit is hardwired to0. This setting of the corresponding bit in the Advanced Error Capabilities and Control Register of PF 0 applies to all Virtual Functions."]
    #[inline(always)]
    pub fn eccap(&self) -> EccapR {
        EccapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable ECRC Check \\[ECC\\]
Setting this bit enables ECRC checking on the receive side of the core. This bit is hardwired to 0. The setting of the corresponding bit in the Advanced Error Capabilities and Control Register of PF 0 applies to all Virtual Functions."]
    #[inline(always)]
    pub fn ecc(&self) -> EccR {
        EccR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multiple Header Recording Capable \\[MHRC\\]
This bit is set when the Function has the capability to log more than one error header in its Header Log Registers. It is hardwired to 0."]
    #[inline(always)]
    pub fn mhrc(&self) -> MhrcR {
        MhrcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multiple Header Recording Enable \\[MHRE\\]
Setting this bit enables the Function to log multiple error headers in its Header Log Registers. It is hardwired to 0"]
    #[inline(always)]
    pub fn mhre(&self) -> MhreR {
        MhreR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - Reserved \\[R18\\]
Reserved"]
    #[inline(always)]
    pub fn r18(&self) -> R18R {
        R18R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
#[doc = "Advanced Error Capabilities and Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`advanced_error_capabilities_and_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdvancedErrorCapabilitiesAndControlSpec;
impl crate::RegisterSpec for AdvancedErrorCapabilitiesAndControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`advanced_error_capabilities_and_control::R`](R) reader structure"]
impl crate::Readable for AdvancedErrorCapabilitiesAndControlSpec {}
#[doc = "`reset()` method sets ADVANCED_ERROR_CAPABILITIES_AND_CONTROL to value 0"]
impl crate::Resettable for AdvancedErrorCapabilitiesAndControlSpec {
    const RESET_VALUE: u32 = 0;
}

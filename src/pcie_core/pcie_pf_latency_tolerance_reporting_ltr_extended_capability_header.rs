#[doc = "Register `PCIE_PF_LATENCY_TOLERANCE_REPORTING_LTR_EXTENDED_CAPABILITY_HEADER` reader"]
pub type R = crate::R<PciePfLatencyToleranceReportingLtrExtendedCapabilityHeaderSpec>;
#[doc = "Field `PECID` reader - PCI Express Extended Capability ID \\[PECID\\]
This field is hardwired to the Capability ID assigned by PCI SIG to the Latency Tolerance Reporting Capability (0018 hex)."]
pub type PecidR = crate::FieldReader<u16>;
#[doc = "Field `CV` reader - Capability Version \\[CV\\]
Specifies the SIG assigned value for the version of the capability structure. This field is set by default to 1, but can be modified from the local management bus."]
pub type CvR = crate::FieldReader;
#[doc = "Field `NCO` reader - Next Capability Offset \\[NCO\\]
Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
pub type NcoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PCI Express Extended Capability ID \\[PECID\\]
This field is hardwired to the Capability ID assigned by PCI SIG to the Latency Tolerance Reporting Capability (0018 hex)."]
    #[inline(always)]
    pub fn pecid(&self) -> PecidR {
        PecidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Capability Version \\[CV\\]
Specifies the SIG assigned value for the version of the capability structure. This field is set by default to 1, but can be modified from the local management bus."]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Next Capability Offset \\[NCO\\]
Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set."]
    #[inline(always)]
    pub fn nco(&self) -> NcoR {
        NcoR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "Latency Tolerance Reporting (LTR) Extended Capability Header Register Indicates offset to the next PCI Express capability structure. The default next pointer value is dynamic and is dependent on whether the strap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_latency_tolerance_reporting_ltr_extended_capability_header::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfLatencyToleranceReportingLtrExtendedCapabilityHeaderSpec;
impl crate::RegisterSpec for PciePfLatencyToleranceReportingLtrExtendedCapabilityHeaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_latency_tolerance_reporting_ltr_extended_capability_header::R`](R) reader structure"]
impl crate::Readable for PciePfLatencyToleranceReportingLtrExtendedCapabilityHeaderSpec {}
#[doc = "`reset()` method sets PCIE_PF_LATENCY_TOLERANCE_REPORTING_LTR_EXTENDED_CAPABILITY_HEADER to value 0x1c01_0018"]
impl crate::Resettable for PciePfLatencyToleranceReportingLtrExtendedCapabilityHeaderSpec {
    const RESET_VALUE: u32 = 0x1c01_0018;
}

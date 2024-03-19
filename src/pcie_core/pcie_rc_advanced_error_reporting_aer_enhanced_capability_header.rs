#[doc = "Register `PCIE_RC_ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER` reader"]
pub type R = crate::R<PcieRcAdvancedErrorReportingAerEnhancedCapabilityHeaderSpec>;
#[doc = "Field `PECID` reader - PCI Express Extended Capability ID \\[PECID\\]\n\nThis field is hardwired to the\n\nCapability ID assigned by PCI SIG to\n\nthe PCI Express AER Extended\n\nCapability Structure (0001 hex)."]
pub type PecidR = crate::FieldReader<u16>;
#[doc = "Field `CV` reader - Capability Version \\[CV\\]\n\nSpecifies the SIG assigned value for\n\nthe version of the capability\n\nstructure. This field is set by default\n\nto 4'h2."]
pub type CvR = crate::FieldReader;
#[doc = "Field `NCO` reader - Next Capability Offset \\[NCO\\]\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set."]
pub type NcoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PCI Express Extended Capability ID \\[PECID\\]\n\nThis field is hardwired to the\n\nCapability ID assigned by PCI SIG to\n\nthe PCI Express AER Extended\n\nCapability Structure (0001 hex)."]
    #[inline(always)]
    pub fn pecid(&self) -> PecidR {
        PecidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Capability Version \\[CV\\]\n\nSpecifies the SIG assigned value for\n\nthe version of the capability\n\nstructure. This field is set by default\n\nto 4'h2."]
    #[inline(always)]
    pub fn cv(&self) -> CvR {
        CvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Next Capability Offset \\[NCO\\]\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set."]
    #[inline(always)]
    pub fn nco(&self) -> NcoR {
        NcoR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "Advanced Error Reporting (AER) Enhanced Capability Header Register\n\nIndicates offset to the next PCI\n\nExpress capability structure. The\n\ndefault next pointer value is dynamic\n\nand is dependent on whether the\n\nstrap or LMI bits are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_advanced_error_reporting_aer_enhanced_capability_header::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcAdvancedErrorReportingAerEnhancedCapabilityHeaderSpec;
impl crate::RegisterSpec for PcieRcAdvancedErrorReportingAerEnhancedCapabilityHeaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_advanced_error_reporting_aer_enhanced_capability_header::R`](R) reader structure"]
impl crate::Readable for PcieRcAdvancedErrorReportingAerEnhancedCapabilityHeaderSpec {}
#[doc = "`reset()` method sets PCIE_RC_ADVANCED_ERROR_REPORTING_AER_ENHANCED_CAPABILITY_HEADER to value 0x2742_0001"]
impl crate::Resettable for PcieRcAdvancedErrorReportingAerEnhancedCapabilityHeaderSpec {
    const RESET_VALUE: u32 = 0x2742_0001;
}

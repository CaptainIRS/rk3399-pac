#[doc = "Register `PCIE_PF_SR_IOV_CAPABILITIES` reader"]
pub type R = crate::R<PciePfSrIovCapabilitiesSpec>;
#[doc = "Field `VFMC` reader - VF Migration Capable \\[VFMC\\]
Set when the core supports VF migration. Hardwired to 0."]
pub type VfmcR = crate::BitReader;
#[doc = "Field `ACHP` reader - ARI Capable Hierarchy Preserved \\[ACHP\\]
A 1 in this bit position indicates that the ARI Capable Hierarchy bit in the SR-IOV Control Register is preserved across certain power state transitions (see the PCI-SIG Single Root IO Virtualization and Sharing Specifications, Version 1.1, Section 3.3.3.5 for details). This bit is set to 1 by default, but can be modified from the local management bus."]
pub type AchpR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - VF Migration Capable \\[VFMC\\]
Set when the core supports VF migration. Hardwired to 0."]
    #[inline(always)]
    pub fn vfmc(&self) -> VfmcR {
        VfmcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ARI Capable Hierarchy Preserved \\[ACHP\\]
A 1 in this bit position indicates that the ARI Capable Hierarchy bit in the SR-IOV Control Register is preserved across certain power state transitions (see the PCI-SIG Single Root IO Virtualization and Sharing Specifications, Version 1.1, Section 3.3.3.5 for details). This bit is set to 1 by default, but can be modified from the local management bus."]
    #[inline(always)]
    pub fn achp(&self) -> AchpR {
        AchpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[doc = "SR-IOV Capabilities Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_sr_iov_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfSrIovCapabilitiesSpec;
impl crate::RegisterSpec for PciePfSrIovCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_sr_iov_capabilities::R`](R) reader structure"]
impl crate::Readable for PciePfSrIovCapabilitiesSpec {}
#[doc = "`reset()` method sets PCIE_PF_SR_IOV_CAPABILITIES to value 0x02"]
impl crate::Resettable for PciePfSrIovCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x02;
}

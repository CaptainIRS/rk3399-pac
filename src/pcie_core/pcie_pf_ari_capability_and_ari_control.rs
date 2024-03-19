#[doc = "Register `PCIE_PF_ARI_CAPABILITY_AND_ARI_CONTROL` reader"]
pub type R = crate::R<PciePfAriCapabilityAndAriControlSpec>;
#[doc = "Field `MFGC` reader - MFVC Function Groups Capability \\[MFGC\\]
Set when device supports arbitration at the Function Group-level. This field is hardwired to 0."]
pub type MfgcR = crate::BitReader;
#[doc = "Field `AFGC` reader - ACS Function Groups Capability \\[AFGC\\]
Relevant only when ACS Capability is supported. This field is hardwired to 0."]
pub type AfgcR = crate::BitReader;
#[doc = "Field `NF` reader - Next Function \\[NF\\]
Points to the next Physical Function in the device. This field is set by default to point to the next Physical Function, 0 for last Function. It can be rewritten from the local management bus."]
pub type NfR = crate::FieldReader;
#[doc = "Field `ACR` reader - ARI Control Register \\[ACR\\]
ARI Control Register not implemented in this core. This field is hardwired to 0."]
pub type AcrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - MFVC Function Groups Capability \\[MFGC\\]
Set when device supports arbitration at the Function Group-level. This field is hardwired to 0."]
    #[inline(always)]
    pub fn mfgc(&self) -> MfgcR {
        MfgcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACS Function Groups Capability \\[AFGC\\]
Relevant only when ACS Capability is supported. This field is hardwired to 0."]
    #[inline(always)]
    pub fn afgc(&self) -> AfgcR {
        AfgcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Next Function \\[NF\\]
Points to the next Physical Function in the device. This field is set by default to point to the next Physical Function, 0 for last Function. It can be rewritten from the local management bus."]
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - ARI Control Register \\[ACR\\]
ARI Control Register not implemented in this core. This field is hardwired to 0."]
    #[inline(always)]
    pub fn acr(&self) -> AcrR {
        AcrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ARI Capability Register and ARI Control Register ARI Control Register not implemented in this core. This field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_ari_capability_and_ari_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfAriCapabilityAndAriControlSpec;
impl crate::RegisterSpec for PciePfAriCapabilityAndAriControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_ari_capability_and_ari_control::R`](R) reader structure"]
impl crate::Readable for PciePfAriCapabilityAndAriControlSpec {}
#[doc = "`reset()` method sets PCIE_PF_ARI_CAPABILITY_AND_ARI_CONTROL to value 0"]
impl crate::Resettable for PciePfAriCapabilityAndAriControlSpec {
    const RESET_VALUE: u32 = 0;
}

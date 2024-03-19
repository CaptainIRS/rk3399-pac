#[doc = "Register `PCIE_RC_MSI_X_CONTROL` reader"]
pub type R = crate::R<PcieRcMsiXControlSpec>;
#[doc = "Register `PCIE_RC_MSI_X_CONTROL` writer"]
pub type W = crate::W<PcieRcMsiXControlSpec>;
#[doc = "Field `CID` reader - Capability ID \\[CID\\]
Identifies that the capability structure is for MSI-X. This field is set by default to 11 hex. It can be rewritten independently for each Function from the local management bus."]
pub type CidR = crate::FieldReader;
#[doc = "Field `CP` reader - Capabilities Pointer \\[CP\\]
Contains pointer to the next PCI Capability Structure. This is set to point to the PCI Express Capability Structure at 30 hex. This can be rewritten independently for each Function from the local management bus."]
pub type CpR = crate::FieldReader;
#[doc = "Field `MSIXTS` reader - MSI-X Table Size \\[MSIXTS\\]
Specifies the size of the MSI-X Table, that is, the number of interrupt vectors defined for the Function. The programmed value is 1 minus the size of the table (that is, this field is set to 0 if the table size is 1.). It can be re- written independently for each Function from the local management bus."]
pub type MsixtsR = crate::FieldReader<u16>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `FM` reader - Function Mask \\[FM\\]
This bit serves as a global mask to all the interrupt conditions associated with this Function. When this bit is set, the core will not send out MSI-X messages from this Function. This field can also be written from the local management bus."]
pub type FmR = crate::BitReader;
#[doc = "Field `FM` writer - Function Mask \\[FM\\]
This bit serves as a global mask to all the interrupt conditions associated with this Function. When this bit is set, the core will not send out MSI-X messages from this Function. This field can also be written from the local management bus."]
pub type FmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIXE` reader - MSI-X Enable \\[MSIXE\\]
Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
pub type MsixeR = crate::BitReader;
#[doc = "Field `MSIXE` writer - MSI-X Enable \\[MSIXE\\]
Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
pub type MsixeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID\\]
Identifies that the capability structure is for MSI-X. This field is set by default to 11 hex. It can be rewritten independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Capabilities Pointer \\[CP\\]
Contains pointer to the next PCI Capability Structure. This is set to point to the PCI Express Capability Structure at 30 hex. This can be rewritten independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:26 - MSI-X Table Size \\[MSIXTS\\]
Specifies the size of the MSI-X Table, that is, the number of interrupt vectors defined for the Function. The programmed value is 1 minus the size of the table (that is, this field is set to 0 if the table size is 1.). It can be re- written independently for each Function from the local management bus."]
    #[inline(always)]
    pub fn msixts(&self) -> MsixtsR {
        MsixtsR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:29 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - Function Mask \\[FM\\]
This bit serves as a global mask to all the interrupt conditions associated with this Function. When this bit is set, the core will not send out MSI-X messages from this Function. This field can also be written from the local management bus."]
    #[inline(always)]
    pub fn fm(&self) -> FmR {
        FmR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MSI-X Enable \\[MSIXE\\]
Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
    #[inline(always)]
    pub fn msixe(&self) -> MsixeR {
        MsixeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Function Mask \\[FM\\]
This bit serves as a global mask to all the interrupt conditions associated with this Function. When this bit is set, the core will not send out MSI-X messages from this Function. This field can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn fm(&mut self) -> FmW<PcieRcMsiXControlSpec> {
        FmW::new(self, 30)
    }
    #[doc = "Bit 31 - MSI-X Enable \\[MSIXE\\]
Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn msixe(&mut self) -> MsixeW<PcieRcMsiXControlSpec> {
        MsixeW::new(self, 31)
    }
}
#[doc = "MSI-X Control Register Set by the configuration program to enable the MSI-X feature. This field can also be written from the local management bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_x_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_x_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcMsiXControlSpec;
impl crate::RegisterSpec for PcieRcMsiXControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_msi_x_control::R`](R) reader structure"]
impl crate::Readable for PcieRcMsiXControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_msi_x_control::W`](W) writer structure"]
impl crate::Writable for PcieRcMsiXControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_MSI_X_CONTROL to value 0xc011"]
impl crate::Resettable for PcieRcMsiXControlSpec {
    const RESET_VALUE: u32 = 0xc011;
}

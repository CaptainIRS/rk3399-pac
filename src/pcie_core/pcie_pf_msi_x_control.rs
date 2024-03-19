#[doc = "Register `PCIE_PF_MSI_X_CONTROL` reader"]
pub type R = crate::R<PciePfMsiXControlSpec>;
#[doc = "Register `PCIE_PF_MSI_X_CONTROL` writer"]
pub type W = crate::W<PciePfMsiXControlSpec>;
#[doc = "Field `CID` reader - Capability ID \\[CID\\]\n\nIdentifies that the capability\n\nstructure is for MSI-X. This field is\n\nset by default to 11 hex. It can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus."]
pub type CidR = crate::FieldReader;
#[doc = "Field `CP` reader - Capabilities Pointer \\[CP\\]\n\nContains pointer to the next PCI\n\nCapability Structure. This is set to\n\npoint to the PCI Express Capability\n\nStructure at 30 hex. This can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus."]
pub type CpR = crate::FieldReader;
#[doc = "Field `MSIXTS` reader - MSI-X Table Size \\[MSIXTS\\]\n\nSpecifies the size of the MSI-X Table,\n\nthat is, the number of interrupt\n\nvectors definedfor the Function. The\n\nprogrammed value is 1 minus the\n\nsize of the table (that is, this field is\n\nset to 0 if the table size is 1.). It can\n\nbe re- written independently for each\n\nFunction from the local management\n\nbus."]
pub type MsixtsR = crate::FieldReader<u16>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `FM` reader - Function Mask \\[FM\\]\n\nThis bit serves as a global mask to\n\nall the interrupt conditions\n\nassociated with this Function. When\n\nthis bit is set, the core will not send\n\nout MSI-X messages from this\n\nFunction. This field can also be\n\nwritten from the local management\n\nbus."]
pub type FmR = crate::BitReader;
#[doc = "Field `FM` writer - Function Mask \\[FM\\]\n\nThis bit serves as a global mask to\n\nall the interrupt conditions\n\nassociated with this Function. When\n\nthis bit is set, the core will not send\n\nout MSI-X messages from this\n\nFunction. This field can also be\n\nwritten from the local management\n\nbus."]
pub type FmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIXE` reader - MSI-X Enable \\[MSIXE\\]\n\nSet by the configuration program to\n\nenable the MSI-X feature. This field\n\ncan also be written from the local\n\nmanagement bus."]
pub type MsixeR = crate::BitReader;
#[doc = "Field `MSIXE` writer - MSI-X Enable \\[MSIXE\\]\n\nSet by the configuration program to\n\nenable the MSI-X feature. This field\n\ncan also be written from the local\n\nmanagement bus."]
pub type MsixeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID\\]\n\nIdentifies that the capability\n\nstructure is for MSI-X. This field is\n\nset by default to 11 hex. It can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Capabilities Pointer \\[CP\\]\n\nContains pointer to the next PCI\n\nCapability Structure. This is set to\n\npoint to the PCI Express Capability\n\nStructure at 30 hex. This can be\n\nrewritten independently for each\n\nFunction from the local management\n\nbus."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:26 - MSI-X Table Size \\[MSIXTS\\]\n\nSpecifies the size of the MSI-X Table,\n\nthat is, the number of interrupt\n\nvectors definedfor the Function. The\n\nprogrammed value is 1 minus the\n\nsize of the table (that is, this field is\n\nset to 0 if the table size is 1.). It can\n\nbe re- written independently for each\n\nFunction from the local management\n\nbus."]
    #[inline(always)]
    pub fn msixts(&self) -> MsixtsR {
        MsixtsR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:29 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - Function Mask \\[FM\\]\n\nThis bit serves as a global mask to\n\nall the interrupt conditions\n\nassociated with this Function. When\n\nthis bit is set, the core will not send\n\nout MSI-X messages from this\n\nFunction. This field can also be\n\nwritten from the local management\n\nbus."]
    #[inline(always)]
    pub fn fm(&self) -> FmR {
        FmR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MSI-X Enable \\[MSIXE\\]\n\nSet by the configuration program to\n\nenable the MSI-X feature. This field\n\ncan also be written from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn msixe(&self) -> MsixeR {
        MsixeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Function Mask \\[FM\\]\n\nThis bit serves as a global mask to\n\nall the interrupt conditions\n\nassociated with this Function. When\n\nthis bit is set, the core will not send\n\nout MSI-X messages from this\n\nFunction. This field can also be\n\nwritten from the local management\n\nbus."]
    #[inline(always)]
    #[must_use]
    pub fn fm(&mut self) -> FmW<PciePfMsiXControlSpec> {
        FmW::new(self, 30)
    }
    #[doc = "Bit 31 - MSI-X Enable \\[MSIXE\\]\n\nSet by the configuration program to\n\nenable the MSI-X feature. This field\n\ncan also be written from the local\n\nmanagement bus."]
    #[inline(always)]
    #[must_use]
    pub fn msixe(&mut self) -> MsixeW<PciePfMsiXControlSpec> {
        MsixeW::new(self, 31)
    }
}
#[doc = "MSI-X Control Register\n\nSet by the configuration program to\n\nenable the MSI-X feature. This field\n\ncan also be written from the local\n\nmanagement bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_x_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_x_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfMsiXControlSpec;
impl crate::RegisterSpec for PciePfMsiXControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_msi_x_control::R`](R) reader structure"]
impl crate::Readable for PciePfMsiXControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_msi_x_control::W`](W) writer structure"]
impl crate::Writable for PciePfMsiXControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_MSI_X_CONTROL to value 0xc011"]
impl crate::Resettable for PciePfMsiXControlSpec {
    const RESET_VALUE: u32 = 0xc011;
}

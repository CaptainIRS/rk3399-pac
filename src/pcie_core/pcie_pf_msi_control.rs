#[doc = "Register `PCIE_PF_MSI_CONTROL` reader"]
pub type R = crate::R<PciePfMsiControlSpec>;
#[doc = "Register `PCIE_PF_MSI_CONTROL` writer"]
pub type W = crate::W<PciePfMsiControlSpec>;
#[doc = "Field `CID1` reader - Capability ID \\[CID1\\]\n\nSpecifies that the capability structure is\n\nfor MSI. Hardwired to 05 hex."]
pub type Cid1R = crate::FieldReader;
#[doc = "Field `CP1` reader - Capabilities Pointer \\[CP1\\]\n\nPointer to the next PCI Capability\n\nStructure. This can be modified from\n\nthe local management bus. This field can\n\nbe written from the local management\n\nbus."]
pub type Cp1R = crate::FieldReader;
#[doc = "Field `ME` reader - MSI Enable \\[ME\\]\n\nSet by the configuration program to\n\nenable the MSI feature. This field can\n\nalso be written from the local\n\nmanagement bus."]
pub type MeR = crate::BitReader;
#[doc = "Field `ME` writer - MSI Enable \\[ME\\]\n\nSet by the configuration program to\n\nenable the MSI feature. This field can\n\nalso be written from the local\n\nmanagement bus."]
pub type MeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMC` reader - Multiple Message Capable \\[MMC\\]\n\nEncodes the number of distinct messages\n\nthat the core is capable of generating for\n\nthis Function (000 = 1, 001 = 2, 010 =\n\n4, 011 = 8, 100 = 16, 101= 32). Thus,\n\nthis field defines the number of the\n\ninterrupt vectors for this Function. The\n\ncore allows up to 32 distinct messages,\n\nbut the setting of this field must be based\n\non the number of interrupt inputs of the\n\ncore that are actually used by the client.\n\nFor example, if the client logic uses 8 of\n\nthe 32 distinct MSI interrupt inputs of the\n\ncore for this Function, then the value of\n\nthis field must be set to 011. This field\n\ncan be written from the local\n\nmanagement bus."]
pub type MmcR = crate::FieldReader;
#[doc = "Field `MME` reader - Multiple Message Enable \\[MME\\]\n\nEncodes the number of distinct messages\n\nthat the core is programmed to generate\n\nfor this Function (000 = 1, 001 = 2, 010\n\n= 4, 011 = 8, 100 = 16, 101= 32). This\n\nsetting must be based on the number of\n\ninterrupt inputs of the core that are\n\nactually used by this Function. This\n\nfield can be written from the local\n\nmanagement bus."]
pub type MmeR = crate::FieldReader;
#[doc = "Field `MME` writer - Multiple Message Enable \\[MME\\]\n\nEncodes the number of distinct messages\n\nthat the core is programmed to generate\n\nfor this Function (000 = 1, 001 = 2, 010\n\n= 4, 011 = 8, 100 = 16, 101= 32). This\n\nsetting must be based on the number of\n\ninterrupt inputs of the core that are\n\nactually used by this Function. This\n\nfield can be written from the local\n\nmanagement bus."]
pub type MmeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BAC64` reader - 64-Bit Address Capable \\[BAC64\\]\n\nSet to 1 to indicate that the device is\n\ncapable of generating 64-bit addresses\n\nfor MSI messages. Can be modified using\n\nlocal management interface"]
pub type Bac64R = crate::BitReader;
#[doc = "Field `MC` reader - MSI masking capable \\[MC\\]\n\ncan be modified using local management\n\ninterface"]
pub type McR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID1\\]\n\nSpecifies that the capability structure is\n\nfor MSI. Hardwired to 05 hex."]
    #[inline(always)]
    pub fn cid1(&self) -> Cid1R {
        Cid1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Capabilities Pointer \\[CP1\\]\n\nPointer to the next PCI Capability\n\nStructure. This can be modified from\n\nthe local management bus. This field can\n\nbe written from the local management\n\nbus."]
    #[inline(always)]
    pub fn cp1(&self) -> Cp1R {
        Cp1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - MSI Enable \\[ME\\]\n\nSet by the configuration program to\n\nenable the MSI feature. This field can\n\nalso be written from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn me(&self) -> MeR {
        MeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Multiple Message Capable \\[MMC\\]\n\nEncodes the number of distinct messages\n\nthat the core is capable of generating for\n\nthis Function (000 = 1, 001 = 2, 010 =\n\n4, 011 = 8, 100 = 16, 101= 32). Thus,\n\nthis field defines the number of the\n\ninterrupt vectors for this Function. The\n\ncore allows up to 32 distinct messages,\n\nbut the setting of this field must be based\n\non the number of interrupt inputs of the\n\ncore that are actually used by the client.\n\nFor example, if the client logic uses 8 of\n\nthe 32 distinct MSI interrupt inputs of the\n\ncore for this Function, then the value of\n\nthis field must be set to 011. This field\n\ncan be written from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Multiple Message Enable \\[MME\\]\n\nEncodes the number of distinct messages\n\nthat the core is programmed to generate\n\nfor this Function (000 = 1, 001 = 2, 010\n\n= 4, 011 = 8, 100 = 16, 101= 32). This\n\nsetting must be based on the number of\n\ninterrupt inputs of the core that are\n\nactually used by this Function. This\n\nfield can be written from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - 64-Bit Address Capable \\[BAC64\\]\n\nSet to 1 to indicate that the device is\n\ncapable of generating 64-bit addresses\n\nfor MSI messages. Can be modified using\n\nlocal management interface"]
    #[inline(always)]
    pub fn bac64(&self) -> Bac64R {
        Bac64R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MSI masking capable \\[MC\\]\n\ncan be modified using local management\n\ninterface"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - MSI Enable \\[ME\\]\n\nSet by the configuration program to\n\nenable the MSI feature. This field can\n\nalso be written from the local\n\nmanagement bus."]
    #[inline(always)]
    #[must_use]
    pub fn me(&mut self) -> MeW<PciePfMsiControlSpec> {
        MeW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Multiple Message Enable \\[MME\\]\n\nEncodes the number of distinct messages\n\nthat the core is programmed to generate\n\nfor this Function (000 = 1, 001 = 2, 010\n\n= 4, 011 = 8, 100 = 16, 101= 32). This\n\nsetting must be based on the number of\n\ninterrupt inputs of the core that are\n\nactually used by this Function. This\n\nfield can be written from the local\n\nmanagement bus."]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MmeW<PciePfMsiControlSpec> {
        MmeW::new(self, 20)
    }
}
#[doc = "MSI Control Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfMsiControlSpec;
impl crate::RegisterSpec for PciePfMsiControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_msi_control::R`](R) reader structure"]
impl crate::Readable for PciePfMsiControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_msi_control::W`](W) writer structure"]
impl crate::Writable for PciePfMsiControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_MSI_CONTROL to value 0x0180_b005"]
impl crate::Resettable for PciePfMsiControlSpec {
    const RESET_VALUE: u32 = 0x0180_b005;
}

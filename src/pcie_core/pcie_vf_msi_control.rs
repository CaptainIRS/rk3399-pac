#[doc = "Register `PCIE_VF_MSI_CONTROL` reader"]
pub type R = crate::R<PcieVfMsiControlSpec>;
#[doc = "Register `PCIE_VF_MSI_CONTROL` writer"]
pub type W = crate::W<PcieVfMsiControlSpec>;
#[doc = "Field `CID` reader - Capability ID \\[CID\\]\n\nSpecifies that the capability structure is\n\nfor MSI. Hardwired to 05 hex."]
pub type CidR = crate::FieldReader;
#[doc = "Field `CP` reader - Capabilities Pointer \\[CP\\]\n\nPointer to the next PCI Capability\n\nStructure. The value read from this\n\nread-only field is the corresponding\n\npointer in the MSI Capability Structure of\n\nthe Physical Function this VF is attached\n\nto. The setting is common across all the\n\nVirtual Functions."]
pub type CpR = crate::FieldReader;
#[doc = "Field `ME` reader - MSI Enable \\[ME\\]\n\nSet by the configuration program to\n\nenable the MSI feature. This field can\n\nalso be written from the local\n\nmanagement bus."]
pub type MeR = crate::BitReader;
#[doc = "Field `ME` writer - MSI Enable \\[ME\\]\n\nSet by the configuration program to\n\nenable the MSI feature. This field can\n\nalso be written from the local\n\nmanagement bus."]
pub type MeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMC` reader - Multiple Message Capable \\[MMC\\]\n\nEncodes the number of distinct messages\n\nthat the core is capable of generating for\n\nthis Function (000 = 1, 001 = 2, 010 = 4,\n\n011 = 8, 100 = 16, 101= 32). Thus, this\n\nfield defines the number of the interrupt\n\nvectors for this Function. The core allows\n\nup to 32 distinct messages, but the\n\nsetting of this field must be based on the\n\nnumber of interrupt inputs of the core\n\nthat are actually used by the client. For\n\nexample, if the client logic uses 8 of the\n\n32 distinct MSI interrupt inputs of the\n\ncore for this Function, then the value of\n\nthis field must be set to 011. This field\n\ncan be written from the local\n\nmanagement bus."]
pub type MmcR = crate::FieldReader;
#[doc = "Field `MME` reader - Multiple Message Enable \\[MME\\]\n\nEncodes the number of distinct messages\n\nthat the core is programmed to generate\n\nfor this Function (000 = 1, 001 = 2, 010\n\n= 4, 011 = 8, 100 = 16, 101= 32). This\n\nsetting must be based on the number of\n\ninterrupt inputs of the core that are\n\nactually used"]
pub type MmeR = crate::FieldReader;
#[doc = "Field `MME` writer - Multiple Message Enable \\[MME\\]\n\nEncodes the number of distinct messages\n\nthat the core is programmed to generate\n\nfor this Function (000 = 1, 001 = 2, 010\n\n= 4, 011 = 8, 100 = 16, 101= 32). This\n\nsetting must be based on the number of\n\ninterrupt inputs of the core that are\n\nactually used"]
pub type MmeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AC64` reader - 64-Bit Address Capable \\[AC64\\]\n\nSet to 1 to indicate that the device is\n\ncapable of generating 64-bit addresses\n\nfor MSI messages."]
pub type Ac64R = crate::BitReader;
#[doc = "Field `MC` reader - MSI masking capable \\[MC\\]\n\ncan be modified using localmanagement interface"]
pub type McR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID\\]\n\nSpecifies that the capability structure is\n\nfor MSI. Hardwired to 05 hex."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Capabilities Pointer \\[CP\\]\n\nPointer to the next PCI Capability\n\nStructure. The value read from this\n\nread-only field is the corresponding\n\npointer in the MSI Capability Structure of\n\nthe Physical Function this VF is attached\n\nto. The setting is common across all the\n\nVirtual Functions."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - MSI Enable \\[ME\\]\n\nSet by the configuration program to\n\nenable the MSI feature. This field can\n\nalso be written from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn me(&self) -> MeR {
        MeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Multiple Message Capable \\[MMC\\]\n\nEncodes the number of distinct messages\n\nthat the core is capable of generating for\n\nthis Function (000 = 1, 001 = 2, 010 = 4,\n\n011 = 8, 100 = 16, 101= 32). Thus, this\n\nfield defines the number of the interrupt\n\nvectors for this Function. The core allows\n\nup to 32 distinct messages, but the\n\nsetting of this field must be based on the\n\nnumber of interrupt inputs of the core\n\nthat are actually used by the client. For\n\nexample, if the client logic uses 8 of the\n\n32 distinct MSI interrupt inputs of the\n\ncore for this Function, then the value of\n\nthis field must be set to 011. This field\n\ncan be written from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Multiple Message Enable \\[MME\\]\n\nEncodes the number of distinct messages\n\nthat the core is programmed to generate\n\nfor this Function (000 = 1, 001 = 2, 010\n\n= 4, 011 = 8, 100 = 16, 101= 32). This\n\nsetting must be based on the number of\n\ninterrupt inputs of the core that are\n\nactually used"]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - 64-Bit Address Capable \\[AC64\\]\n\nSet to 1 to indicate that the device is\n\ncapable of generating 64-bit addresses\n\nfor MSI messages."]
    #[inline(always)]
    pub fn ac64(&self) -> Ac64R {
        Ac64R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MSI masking capable \\[MC\\]\n\ncan be modified using localmanagement interface"]
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
    pub fn me(&mut self) -> MeW<PcieVfMsiControlSpec> {
        MeW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Multiple Message Enable \\[MME\\]\n\nEncodes the number of distinct messages\n\nthat the core is programmed to generate\n\nfor this Function (000 = 1, 001 = 2, 010\n\n= 4, 011 = 8, 100 = 16, 101= 32). This\n\nsetting must be based on the number of\n\ninterrupt inputs of the core that are\n\nactually used"]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MmeW<PcieVfMsiControlSpec> {
        MmeW::new(self, 20)
    }
}
#[doc = "MSI Control Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_msi_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfMsiControlSpec;
impl crate::RegisterSpec for PcieVfMsiControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_msi_control::R`](R) reader structure"]
impl crate::Readable for PcieVfMsiControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_vf_msi_control::W`](W) writer structure"]
impl crate::Writable for PcieVfMsiControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_VF_MSI_CONTROL to value 0x0180_b005"]
impl crate::Resettable for PcieVfMsiControlSpec {
    const RESET_VALUE: u32 = 0x0180_b005;
}

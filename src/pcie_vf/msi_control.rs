#[doc = "Register `MSI_CONTROL` reader"]
pub type R = crate::R<MsiControlSpec>;
#[doc = "Register `MSI_CONTROL` writer"]
pub type W = crate::W<MsiControlSpec>;
#[doc = "Field `CID` reader - Capability ID \\[CID\\]
Specifies that the capability structure is for MSI. Hardwired to 05 hex."]
pub type CidR = crate::FieldReader;
#[doc = "Field `CP` reader - Capabilities Pointer \\[CP\\]
Pointer to the next PCI Capability Structure. The value read from this read-only field is the corresponding pointer in the MSI Capability Structure of the Physical Function this VF is attached to. The setting is common across all the Virtual Functions."]
pub type CpR = crate::FieldReader;
#[doc = "Field `ME` reader - MSI Enable \\[ME\\]
Set by the configuration program to enable the MSI feature. This field can also be written from the local management bus."]
pub type MeR = crate::BitReader;
#[doc = "Field `ME` writer - MSI Enable \\[ME\\]
Set by the configuration program to enable the MSI feature. This field can also be written from the local management bus."]
pub type MeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMC` reader - Multiple Message Capable \\[MMC\\]
Encodes the number of distinct messages that the core is capable of generating for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101= 32). Thus, this field defines the number of the interrupt vectors for this Function. The core allows up to 32 distinct messages, but the setting of this field must be based on the number of interrupt inputs of the core that are actually used by the client. For example, if the client logic uses 8 of the 32 distinct MSI interrupt inputs of the core for this Function, then the value of this field must be set to 011. This field can be written from the local management bus."]
pub type MmcR = crate::FieldReader;
#[doc = "Field `MME` reader - Multiple Message Enable \\[MME\\]
Encodes the number of distinct messages that the core is programmed to generate for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101= 32). This setting must be based on the number of interrupt inputs of the core that are actually used"]
pub type MmeR = crate::FieldReader;
#[doc = "Field `MME` writer - Multiple Message Enable \\[MME\\]
Encodes the number of distinct messages that the core is programmed to generate for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101= 32). This setting must be based on the number of interrupt inputs of the core that are actually used"]
pub type MmeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AC64` reader - 64-Bit Address Capable \\[AC64\\]
Set to 1 to indicate that the device is capable of generating 64-bit addresses for MSI messages."]
pub type Ac64R = crate::BitReader;
#[doc = "Field `MC` reader - MSI masking capable \\[MC\\]
can be modified using localmanagement interface"]
pub type McR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID\\]
Specifies that the capability structure is for MSI. Hardwired to 05 hex."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Capabilities Pointer \\[CP\\]
Pointer to the next PCI Capability Structure. The value read from this read-only field is the corresponding pointer in the MSI Capability Structure of the Physical Function this VF is attached to. The setting is common across all the Virtual Functions."]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - MSI Enable \\[ME\\]
Set by the configuration program to enable the MSI feature. This field can also be written from the local management bus."]
    #[inline(always)]
    pub fn me(&self) -> MeR {
        MeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Multiple Message Capable \\[MMC\\]
Encodes the number of distinct messages that the core is capable of generating for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101= 32). Thus, this field defines the number of the interrupt vectors for this Function. The core allows up to 32 distinct messages, but the setting of this field must be based on the number of interrupt inputs of the core that are actually used by the client. For example, if the client logic uses 8 of the 32 distinct MSI interrupt inputs of the core for this Function, then the value of this field must be set to 011. This field can be written from the local management bus."]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Multiple Message Enable \\[MME\\]
Encodes the number of distinct messages that the core is programmed to generate for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101= 32). This setting must be based on the number of interrupt inputs of the core that are actually used"]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - 64-Bit Address Capable \\[AC64\\]
Set to 1 to indicate that the device is capable of generating 64-bit addresses for MSI messages."]
    #[inline(always)]
    pub fn ac64(&self) -> Ac64R {
        Ac64R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MSI masking capable \\[MC\\]
can be modified using localmanagement interface"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - MSI Enable \\[ME\\]
Set by the configuration program to enable the MSI feature. This field can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn me(&mut self) -> MeW<MsiControlSpec> {
        MeW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Multiple Message Enable \\[MME\\]
Encodes the number of distinct messages that the core is programmed to generate for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101= 32). This setting must be based on the number of interrupt inputs of the core that are actually used"]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MmeW<MsiControlSpec> {
        MmeW::new(self, 20)
    }
}
#[doc = "MSI Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msi_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsiControlSpec;
impl crate::RegisterSpec for MsiControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msi_control::R`](R) reader structure"]
impl crate::Readable for MsiControlSpec {}
#[doc = "`write(|w| ..)` method takes [`msi_control::W`](W) writer structure"]
impl crate::Writable for MsiControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSI_CONTROL to value 0x0180_b005"]
impl crate::Resettable for MsiControlSpec {
    const RESET_VALUE: u32 = 0x0180_b005;
}

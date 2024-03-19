#[doc = "Register `PCIE_RC_MSI_CONTROL` reader"]
pub type R = crate::R<PcieRcMsiControlSpec>;
#[doc = "Register `PCIE_RC_MSI_CONTROL` writer"]
pub type W = crate::W<PcieRcMsiControlSpec>;
#[doc = "Field `CID1` reader - Capability ID \\[CID1\\]
Specifies that the capability structure is for MSI. Hardwired to 05 hex."]
pub type Cid1R = crate::FieldReader;
#[doc = "Field `CP1` reader - Capabilities Pointer \\[CP1\\]
Pointer to the next PCI Capability Structure. This can be modified from the local management bus. This field can be written from the local management bus."]
pub type Cp1R = crate::FieldReader;
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
Encodes the number of distinct messages that the core is programmed to generate for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101 = 32). This setting must be based on the number of interrupt inputs of the core that are actually used by this Function. This field can be written from the local management bus."]
pub type MmeR = crate::FieldReader;
#[doc = "Field `MME` writer - Multiple Message Enable \\[MME\\]
Encodes the number of distinct messages that the core is programmed to generate for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101 = 32). This setting must be based on the number of interrupt inputs of the core that are actually used by this Function. This field can be written from the local management bus."]
pub type MmeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BAC64` reader - 64-Bit Address Capable \\[BAC64\\]
Set to 1 to indicate that the device is capable of generating 64-bit addresses for MSI messages. Can be modified using local management interface"]
pub type Bac64R = crate::BitReader;
#[doc = "Field `MC` reader - MSI masking capable \\[MC\\]
can be modified using local management interface"]
pub type McR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Capability ID \\[CID1\\]
Specifies that the capability structure is for MSI. Hardwired to 05 hex."]
    #[inline(always)]
    pub fn cid1(&self) -> Cid1R {
        Cid1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Capabilities Pointer \\[CP1\\]
Pointer to the next PCI Capability Structure. This can be modified from the local management bus. This field can be written from the local management bus."]
    #[inline(always)]
    pub fn cp1(&self) -> Cp1R {
        Cp1R::new(((self.bits >> 8) & 0xff) as u8)
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
Encodes the number of distinct messages that the core is programmed to generate for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101 = 32). This setting must be based on the number of interrupt inputs of the core that are actually used by this Function. This field can be written from the local management bus."]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - 64-Bit Address Capable \\[BAC64\\]
Set to 1 to indicate that the device is capable of generating 64-bit addresses for MSI messages. Can be modified using local management interface"]
    #[inline(always)]
    pub fn bac64(&self) -> Bac64R {
        Bac64R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MSI masking capable \\[MC\\]
can be modified using local management interface"]
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
    pub fn me(&mut self) -> MeW<PcieRcMsiControlSpec> {
        MeW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Multiple Message Enable \\[MME\\]
Encodes the number of distinct messages that the core is programmed to generate for this Function (000 = 1, 001 = 2, 010 = 4, 011 = 8, 100 = 16, 101 = 32). This setting must be based on the number of interrupt inputs of the core that are actually used by this Function. This field can be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MmeW<PcieRcMsiControlSpec> {
        MmeW::new(self, 20)
    }
}
#[doc = "MSI Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_msi_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcMsiControlSpec;
impl crate::RegisterSpec for PcieRcMsiControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_msi_control::R`](R) reader structure"]
impl crate::Readable for PcieRcMsiControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_msi_control::W`](W) writer structure"]
impl crate::Writable for PcieRcMsiControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_MSI_CONTROL to value 0x0180_b005"]
impl crate::Resettable for PcieRcMsiControlSpec {
    const RESET_VALUE: u32 = 0x0180_b005;
}

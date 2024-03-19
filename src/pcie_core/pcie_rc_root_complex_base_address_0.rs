#[doc = "Register `PCIE_RC_ROOT_COMPLEX_BASE_ADDRESS_0` reader"]
pub type R = crate::R<PcieRcRootComplexBaseAddress0Spec>;
#[doc = "Register `PCIE_RC_ROOT_COMPLEX_BASE_ADDRESS_0` writer"]
pub type W = crate::W<PcieRcRootComplexBaseAddress0Spec>;
#[doc = "Field `MSI0` reader - BAR Type \\[MSI0\\]\n\nSpecifies whether this BAR defines a\n\nmemory address range or an I/O\n\naddress range (0 = memory, 1 =\n\nI/O). The value read in this field is\n\ndetermined by the setting of Root\n\nComplex BAR Configuration\n\nRegister."]
pub type Msi0R = crate::BitReader;
#[doc = "Field `R7` reader - Reserved \\[R7\\]\n\nThis bit is hardwired to 0 for both\n\nmemory and I/O BARs."]
pub type R7R = crate::BitReader;
#[doc = "Field `S0` reader - Size \\[S0\\]\n\nFor memory BAR: This bit reads as\n\n0 when BAR 0 is configured as a 32-\n\nbit BAR, and as 1 when configured\n\nas a 64-bit BAR. For IO BAR: This is\n\nbit 3 of the base address. The value\n\nread in this field is determined by\n\nthe setting of Root Complex BAR\n\nConfiguration Register."]
pub type S0R = crate::BitReader;
#[doc = "Field `P0` reader - Prefetchability \\[P0\\]\n\nFor memory BAR: This bit reads as\n\n1 when BAR 0 is configured as a\n\nprefetchable BAR, and as 0 when\n\nconfigured as a non-prefetchable\n\nBAR. For IO BAR: This is bit 3 of the\n\nbase address. The value read in this\n\nfield is determined by the setting of\n\nRoot Complex BAR Configuration\n\nRegister."]
pub type P0R = crate::BitReader;
#[doc = "Field `BAMR0` reader - Base Address - RO part \\[BAMR0\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in Root Complex BAR\n\nConfiguration Register. All other bits\n\nare not writeable, and are read as\n\n0's."]
pub type Bamr0R = crate::FieldReader<u32>;
#[doc = "Field `BAMRW` reader - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in Root Complex BAR\n\nConfiguration Register. All other bits\n\nare not writeable, and are read as\n\n0's."]
pub type BamrwR = crate::FieldReader<u16>;
#[doc = "Field `BAMRW` writer - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in Root Complex BAR\n\nConfiguration Register. All other bits\n\nare not writeable, and are read as\n\n0's."]
pub type BamrwW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - BAR Type \\[MSI0\\]\n\nSpecifies whether this BAR defines a\n\nmemory address range or an I/O\n\naddress range (0 = memory, 1 =\n\nI/O). The value read in this field is\n\ndetermined by the setting of Root\n\nComplex BAR Configuration\n\nRegister."]
    #[inline(always)]
    pub fn msi0(&self) -> Msi0R {
        Msi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved \\[R7\\]\n\nThis bit is hardwired to 0 for both\n\nmemory and I/O BARs."]
    #[inline(always)]
    pub fn r7(&self) -> R7R {
        R7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Size \\[S0\\]\n\nFor memory BAR: This bit reads as\n\n0 when BAR 0 is configured as a 32-\n\nbit BAR, and as 1 when configured\n\nas a 64-bit BAR. For IO BAR: This is\n\nbit 3 of the base address. The value\n\nread in this field is determined by\n\nthe setting of Root Complex BAR\n\nConfiguration Register."]
    #[inline(always)]
    pub fn s0(&self) -> S0R {
        S0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Prefetchability \\[P0\\]\n\nFor memory BAR: This bit reads as\n\n1 when BAR 0 is configured as a\n\nprefetchable BAR, and as 0 when\n\nconfigured as a non-prefetchable\n\nBAR. For IO BAR: This is bit 3 of the\n\nbase address. The value read in this\n\nfield is determined by the setting of\n\nRoot Complex BAR Configuration\n\nRegister."]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:21 - Base Address - RO part \\[BAMR0\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in Root Complex BAR\n\nConfiguration Register. All other bits\n\nare not writeable, and are read as\n\n0's."]
    #[inline(always)]
    pub fn bamr0(&self) -> Bamr0R {
        Bamr0R::new((self.bits >> 4) & 0x0003_ffff)
    }
    #[doc = "Bits 22:31 - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in Root Complex BAR\n\nConfiguration Register. All other bits\n\nare not writeable, and are read as\n\n0's."]
    #[inline(always)]
    pub fn bamrw(&self) -> BamrwR {
        BamrwR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31 - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in Root Complex BAR\n\nConfiguration Register. All other bits\n\nare not writeable, and are read as\n\n0's."]
    #[inline(always)]
    #[must_use]
    pub fn bamrw(&mut self) -> BamrwW<PcieRcRootComplexBaseAddress0Spec> {
        BamrwW::new(self, 22)
    }
}
#[doc = "Root Complex Base Address Register 0\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in Root Complex BAR\n\nConfiguration Register. All other bits\n\nare not writeable, and are read as\n\n0's.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_root_complex_base_address_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_root_complex_base_address_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcRootComplexBaseAddress0Spec;
impl crate::RegisterSpec for PcieRcRootComplexBaseAddress0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_root_complex_base_address_0::R`](R) reader structure"]
impl crate::Readable for PcieRcRootComplexBaseAddress0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_root_complex_base_address_0::W`](W) writer structure"]
impl crate::Writable for PcieRcRootComplexBaseAddress0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_ROOT_COMPLEX_BASE_ADDRESS_0 to value 0x04"]
impl crate::Resettable for PcieRcRootComplexBaseAddress0Spec {
    const RESET_VALUE: u32 = 0x04;
}

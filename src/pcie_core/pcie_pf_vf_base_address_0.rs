#[doc = "Register `PCIE_PF_VF_BASE_ADDRESS_0` reader"]
pub type R = crate::R<PciePfVfBaseAddress0Spec>;
#[doc = "Register `PCIE_PF_VF_BASE_ADDRESS_0` writer"]
pub type W = crate::W<PciePfVfBaseAddress0Spec>;
#[doc = "Field `MSI` reader - Memory Space Indicator \\[MSI\\]\n\nSpecifies whether this BAR defines a\n\nmemory address range or an I/O\n\naddress range (0 = memory, 1 =\n\nI/O). The value read in this field is\n\ndetermined by the setting of BAR\n\nConfiguration Registers of the\n\nassociated Physical Function"]
pub type MsiR = crate::BitReader;
#[doc = "Field `R7` reader - Reserved \\[R7\\]\n\nThis bit is hardwired to 0 for both\n\nmemory and I/O BARs."]
pub type R7R = crate::BitReader;
#[doc = "Field `S0` reader - Size \\[S0\\]\n\nWhen the BAR is used to define a\n\nmemory address range, this field\n\nindicates whether the address\n\nrange is 32-bit or 64-bit (0 = 32-\n\nbit, 1 = 64 bit).\n\nFor 64-bit address ranges, the value\n\nin BAR 1 is treated as a continuation\n\nof the base address in BAR 0. The\n\nvalue read in this field is determined\n\nby the setting of BAR Configuration\n\nRegisters of the associated Physical\n\nFunction."]
pub type S0R = crate::BitReader;
#[doc = "Field `P0` reader - Prefetchability \\[P0\\]\n\nWhen the BAR is used to define a\n\nmemory address range, this field\n\ndeclares whether data from the\n\naddress range is prefetchable (0 =\n\nnon- prefetchable, 1 =\n\nprefetchable). The value read in this\n\nfield is determined by the setting of\n\nBAR\n\nConfiguration Registers of the\n\nassociated Physical Function"]
pub type P0R = crate::BitReader;
#[doc = "Field `R8` reader - Reserved \\[R8\\]\n\nThese bits are hardwired to 0"]
pub type R8R = crate::FieldReader;
#[doc = "Field `BAMR0` reader - Base Address - RO part \\[BAMR0\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in BAR Configuration\n\nRegisters of the associated Physical\n\nFunction. All other bits are not\n\nwriteable, and are read as 0's."]
pub type Bamr0R = crate::FieldReader<u16>;
#[doc = "Field `BAMRW` reader - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in BAR Configuration\n\nRegisters of the associated Physical\n\nFunction."]
pub type BamrwR = crate::FieldReader<u16>;
#[doc = "Field `BAMRW` writer - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in BAR Configuration\n\nRegisters of the associated Physical\n\nFunction."]
pub type BamrwW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Memory Space Indicator \\[MSI\\]\n\nSpecifies whether this BAR defines a\n\nmemory address range or an I/O\n\naddress range (0 = memory, 1 =\n\nI/O). The value read in this field is\n\ndetermined by the setting of BAR\n\nConfiguration Registers of the\n\nassociated Physical Function"]
    #[inline(always)]
    pub fn msi(&self) -> MsiR {
        MsiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved \\[R7\\]\n\nThis bit is hardwired to 0 for both\n\nmemory and I/O BARs."]
    #[inline(always)]
    pub fn r7(&self) -> R7R {
        R7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Size \\[S0\\]\n\nWhen the BAR is used to define a\n\nmemory address range, this field\n\nindicates whether the address\n\nrange is 32-bit or 64-bit (0 = 32-\n\nbit, 1 = 64 bit).\n\nFor 64-bit address ranges, the value\n\nin BAR 1 is treated as a continuation\n\nof the base address in BAR 0. The\n\nvalue read in this field is determined\n\nby the setting of BAR Configuration\n\nRegisters of the associated Physical\n\nFunction."]
    #[inline(always)]
    pub fn s0(&self) -> S0R {
        S0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Prefetchability \\[P0\\]\n\nWhen the BAR is used to define a\n\nmemory address range, this field\n\ndeclares whether data from the\n\naddress range is prefetchable (0 =\n\nnon- prefetchable, 1 =\n\nprefetchable). The value read in this\n\nfield is determined by the setting of\n\nBAR\n\nConfiguration Registers of the\n\nassociated Physical Function"]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved \\[R8\\]\n\nThese bits are hardwired to 0"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:21 - Base Address - RO part \\[BAMR0\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in BAR Configuration\n\nRegisters of the associated Physical\n\nFunction. All other bits are not\n\nwriteable, and are read as 0's."]
    #[inline(always)]
    pub fn bamr0(&self) -> Bamr0R {
        Bamr0R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:31 - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in BAR Configuration\n\nRegisters of the associated Physical\n\nFunction."]
    #[inline(always)]
    pub fn bamrw(&self) -> BamrwR {
        BamrwR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31 - Base Address - RW part \\[BAMRW\\]\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in BAR Configuration\n\nRegisters of the associated Physical\n\nFunction."]
    #[inline(always)]
    #[must_use]
    pub fn bamrw(&mut self) -> BamrwW<PciePfVfBaseAddress0Spec> {
        BamrwW::new(self, 22)
    }
}
#[doc = "VF Base Address Register 0\n\nThis field defines the base address\n\nof the memory address range. The\n\nnumber of implemented bits in this\n\nfield determines the BAR aperture\n\nconfigured in BAR Configuration\n\nRegisters of the associated Physical\n\nFunction.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_vf_base_address_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_vf_base_address_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfVfBaseAddress0Spec;
impl crate::RegisterSpec for PciePfVfBaseAddress0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_vf_base_address_0::R`](R) reader structure"]
impl crate::Readable for PciePfVfBaseAddress0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_vf_base_address_0::W`](W) writer structure"]
impl crate::Writable for PciePfVfBaseAddress0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_VF_BASE_ADDRESS_0 to value 0x04"]
impl crate::Resettable for PciePfVfBaseAddress0Spec {
    const RESET_VALUE: u32 = 0x04;
}

#[doc = "Register `PCIE_PF_MSI_MASK` reader"]
pub type R = crate::R<PciePfMsiMaskSpec>;
#[doc = "Register `PCIE_PF_MSI_MASK` writer"]
pub type W = crate::W<PciePfMsiMaskSpec>;
#[doc = "Field `MM` reader - MSI Mask \\[MM\\]
Mask bits for MSI interrupts. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts forthe Function, which determines the number of valid mask bits. Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of the MSI Mask field also changes correspondingly"]
pub type MmR = crate::BitReader;
#[doc = "Field `MM` writer - MSI Mask \\[MM\\]
Mask bits for MSI interrupts. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts forthe Function, which determines the number of valid mask bits. Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of the MSI Mask field also changes correspondingly"]
pub type MmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - MSI Mask \\[MM\\]
Mask bits for MSI interrupts. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts forthe Function, which determines the number of valid mask bits. Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of the MSI Mask field also changes correspondingly"]
    #[inline(always)]
    pub fn mm(&self) -> MmR {
        MmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved \\[R0\\]
Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - MSI Mask \\[MM\\]
Mask bits for MSI interrupts. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts forthe Function, which determines the number of valid mask bits. Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of the MSI Mask field also changes correspondingly"]
    #[inline(always)]
    #[must_use]
    pub fn mm(&mut self) -> MmW<PciePfMsiMaskSpec> {
        MmW::new(self, 0)
    }
}
#[doc = "MSI Mask Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfMsiMaskSpec;
impl crate::RegisterSpec for PciePfMsiMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_msi_mask::R`](R) reader structure"]
impl crate::Readable for PciePfMsiMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_msi_mask::W`](W) writer structure"]
impl crate::Writable for PciePfMsiMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_MSI_MASK to value 0"]
impl crate::Resettable for PciePfMsiMaskSpec {
    const RESET_VALUE: u32 = 0;
}

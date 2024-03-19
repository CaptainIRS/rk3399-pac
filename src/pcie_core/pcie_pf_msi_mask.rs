#[doc = "Register `PCIE_PF_MSI_MASK` reader"]
pub type R = crate::R<PciePfMsiMaskSpec>;
#[doc = "Register `PCIE_PF_MSI_MASK` writer"]
pub type W = crate::W<PciePfMsiMaskSpec>;
#[doc = "Field `MM` reader - MSI Mask \\[MM\\]\n\nMask bits for MSI interrupts. The\n\nMultiple Message Capable field of the\n\nMSI Control Register specifies the\n\nnumber of distinct interrupts forthe\n\nFunction, which determines the\n\nnumber of valid mask bits. Please\n\nnote that if the Multiple Message\n\nCapable field is changed from the\n\nlocal management APB bus, then the\n\nwidth of the MSI Mask field also\n\nchanges correspondingly"]
pub type MmR = crate::BitReader;
#[doc = "Field `MM` writer - MSI Mask \\[MM\\]\n\nMask bits for MSI interrupts. The\n\nMultiple Message Capable field of the\n\nMSI Control Register specifies the\n\nnumber of distinct interrupts forthe\n\nFunction, which determines the\n\nnumber of valid mask bits. Please\n\nnote that if the Multiple Message\n\nCapable field is changed from the\n\nlocal management APB bus, then the\n\nwidth of the MSI Mask field also\n\nchanges correspondingly"]
pub type MmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nPlease note that if the Multiple\n\nMessage Capable field is changed\n\nfrom the local management APB bus,\n\nthen the width of this field also\n\nchanges correspondingly"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - MSI Mask \\[MM\\]\n\nMask bits for MSI interrupts. The\n\nMultiple Message Capable field of the\n\nMSI Control Register specifies the\n\nnumber of distinct interrupts forthe\n\nFunction, which determines the\n\nnumber of valid mask bits. Please\n\nnote that if the Multiple Message\n\nCapable field is changed from the\n\nlocal management APB bus, then the\n\nwidth of the MSI Mask field also\n\nchanges correspondingly"]
    #[inline(always)]
    pub fn mm(&self) -> MmR {
        MmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved \\[R0\\]\n\nPlease note that if the Multiple\n\nMessage Capable field is changed\n\nfrom the local management APB bus,\n\nthen the width of this field also\n\nchanges correspondingly"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - MSI Mask \\[MM\\]\n\nMask bits for MSI interrupts. The\n\nMultiple Message Capable field of the\n\nMSI Control Register specifies the\n\nnumber of distinct interrupts forthe\n\nFunction, which determines the\n\nnumber of valid mask bits. Please\n\nnote that if the Multiple Message\n\nCapable field is changed from the\n\nlocal management APB bus, then the\n\nwidth of the MSI Mask field also\n\nchanges correspondingly"]
    #[inline(always)]
    #[must_use]
    pub fn mm(&mut self) -> MmW<PciePfMsiMaskSpec> {
        MmW::new(self, 0)
    }
}
#[doc = "MSI Mask Register\n\nPlease note that if the Multiple\n\nMessage Capable field is changed\n\nfrom the local management APB bus,\n\nthen the width of this field also\n\nchanges correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_msi_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

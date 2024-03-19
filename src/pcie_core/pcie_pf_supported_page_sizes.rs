#[doc = "Register `PCIE_PF_SUPPORTED_PAGE_SIZES` reader"]
pub type R = crate::R<PciePfSupportedPageSizesSpec>;
#[doc = "Field `PS` reader - Page Sizes \\[PS\\]
Page sizes supported by the device (one bit for each page size). The core implements only bits 15:0 of this register. The default value of this field is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus."]
pub type PsR = crate::FieldReader<u16>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Page Sizes \\[PS\\]
Page sizes supported by the device (one bit for each page size). The core implements only bits 15:0 of this register. The default value of this field is specified in reg_defaults.h, but can be re- written independently for each PF from the local management bus."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Supported Page Sizes Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_supported_page_sizes::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfSupportedPageSizesSpec;
impl crate::RegisterSpec for PciePfSupportedPageSizesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_supported_page_sizes::R`](R) reader structure"]
impl crate::Readable for PciePfSupportedPageSizesSpec {}
#[doc = "`reset()` method sets PCIE_PF_SUPPORTED_PAGE_SIZES to value 0x0553"]
impl crate::Resettable for PciePfSupportedPageSizesSpec {
    const RESET_VALUE: u32 = 0x0553;
}

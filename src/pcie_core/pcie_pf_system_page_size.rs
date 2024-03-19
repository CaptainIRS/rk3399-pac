#[doc = "Register `PCIE_PF_SYSTEM_PAGE_SIZE` reader"]
pub type R = crate::R<PciePfSystemPageSizeSpec>;
#[doc = "Register `PCIE_PF_SYSTEM_PAGE_SIZE` writer"]
pub type W = crate::W<PciePfSystemPageSizeSpec>;
#[doc = "Field `SPS` reader - System Page Size \\[SPS\\]\n\nThis field must be programmed by\n\nsoftware to the current page size in\n\nuse. The core implements only bits\n\n15:0 of this register. This field can\n\nalso be written from the local\n\nmanagement bus."]
pub type SpsR = crate::FieldReader<u16>;
#[doc = "Field `SPS` writer - System Page Size \\[SPS\\]\n\nThis field must be programmed by\n\nsoftware to the current page size in\n\nuse. The core implements only bits\n\n15:0 of this register. This field can\n\nalso be written from the local\n\nmanagement bus."]
pub type SpsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - System Page Size \\[SPS\\]\n\nThis field must be programmed by\n\nsoftware to the current page size in\n\nuse. The core implements only bits\n\n15:0 of this register. This field can\n\nalso be written from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - System Page Size \\[SPS\\]\n\nThis field must be programmed by\n\nsoftware to the current page size in\n\nuse. The core implements only bits\n\n15:0 of this register. This field can\n\nalso be written from the local\n\nmanagement bus."]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SpsW<PciePfSystemPageSizeSpec> {
        SpsW::new(self, 0)
    }
}
#[doc = "System Page Size Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_system_page_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_system_page_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfSystemPageSizeSpec;
impl crate::RegisterSpec for PciePfSystemPageSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_system_page_size::R`](R) reader structure"]
impl crate::Readable for PciePfSystemPageSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_system_page_size::W`](W) writer structure"]
impl crate::Writable for PciePfSystemPageSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_SYSTEM_PAGE_SIZE to value 0x01"]
impl crate::Resettable for PciePfSystemPageSizeSpec {
    const RESET_VALUE: u32 = 0x01;
}

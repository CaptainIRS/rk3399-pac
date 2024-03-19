#[doc = "Register `PCIE_PF_RESIZABLE_BAR_CAPABILITY_2` reader"]
pub type R = crate::R<PciePfResizableBarCapability2Spec>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `A1M` reader - Aperture 1M \\[A1M\\]
Indicates that the BAR aperture can be set to 1M."]
pub type A1mR = crate::BitReader;
#[doc = "Field `A2M` reader - Aperture 2M \\[A2M\\]
Indicates that the BAR aperture can be set to 2M."]
pub type A2mR = crate::BitReader;
#[doc = "Field `A4M` reader - Aperture 4M \\[A4M\\]
Indicates that the BAR aperture can be set to 4M."]
pub type A4mR = crate::BitReader;
#[doc = "Field `A8M` reader - Aperture 8M \\[A8M\\]
Indicates that the BAR aperture can be set to 8M."]
pub type A8mR = crate::BitReader;
#[doc = "Field `A16M` reader - Aperture 16M \\[A16M\\]
Indicates that the BAR aperture can be set to 16M."]
pub type A16mR = crate::BitReader;
#[doc = "Field `A32M` reader - Aperture 32M \\[A32M\\]
Indicates that the BAR aperture can be set to 32M."]
pub type A32mR = crate::BitReader;
#[doc = "Field `A64M` reader - Aperture 64M \\[A64M\\]
Indicates that the BAR aperture can be set to 64M."]
pub type A64mR = crate::BitReader;
#[doc = "Field `A128M` reader - Aperture 128M \\[A128M\\]
Indicates that the BAR aperture can be set to 128M."]
pub type A128mR = crate::BitReader;
#[doc = "Field `A256M` reader - Aperture 256M \\[A256M\\]
Indicates that the BAR aperture can be set to 256M."]
pub type A256mR = crate::BitReader;
#[doc = "Field `A512M` reader - Aperture 512M \\[A512M\\]
Indicates that the BAR aperture can be set to 512M."]
pub type A512mR = crate::BitReader;
#[doc = "Field `A1G` reader - Aperture 1G \\[A1G\\]
Indicates that the BAR aperture can be set to 1G."]
pub type A1gR = crate::BitReader;
#[doc = "Field `A2G` reader - Aperture 2G \\[A2G\\]
Indicates that the BAR aperture can be set to 2G."]
pub type A2gR = crate::BitReader;
#[doc = "Field `A4G` reader - Aperture 4G \\[A4G\\]
Indicates that the BAR aperture can be set to 4G."]
pub type A4gR = crate::BitReader;
#[doc = "Field `A8G` reader - Aperture 8G \\[A8G\\]
Indicates that the BAR aperture can be set to 8G."]
pub type A8gR = crate::BitReader;
#[doc = "Field `A16G` reader - Aperture 16G \\[A16G\\]
Indicates that the BAR aperture can be set to 16G."]
pub type A16gR = crate::BitReader;
#[doc = "Field `A32G` reader - Aperture 32G \\[A32G\\]
Indicates that the BAR aperture can be set to 32G."]
pub type A32gR = crate::BitReader;
#[doc = "Field `A64G` reader - Aperture 64G \\[A64G\\]
Indicates that the BAR aperture can be set to 64G."]
pub type A64gR = crate::BitReader;
#[doc = "Field `A128G` reader - Aperture 128G \\[A128G\\]
Indicates that the BAR aperture can be set to 128G."]
pub type A128gR = crate::BitReader;
#[doc = "Field `A256G` reader - Aperture 256G \\[A256G\\]
Indicates that the BAR aperture can be set to 256G."]
pub type A256gR = crate::BitReader;
#[doc = "Field `A512G` reader - Aperture 512G \\[A512G\\]
Indicates that the BAR aperture can be set to 512G."]
pub type A512gR = crate::BitReader;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Aperture 1M \\[A1M\\]
Indicates that the BAR aperture can be set to 1M."]
    #[inline(always)]
    pub fn a1m(&self) -> A1mR {
        A1mR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Aperture 2M \\[A2M\\]
Indicates that the BAR aperture can be set to 2M."]
    #[inline(always)]
    pub fn a2m(&self) -> A2mR {
        A2mR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Aperture 4M \\[A4M\\]
Indicates that the BAR aperture can be set to 4M."]
    #[inline(always)]
    pub fn a4m(&self) -> A4mR {
        A4mR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Aperture 8M \\[A8M\\]
Indicates that the BAR aperture can be set to 8M."]
    #[inline(always)]
    pub fn a8m(&self) -> A8mR {
        A8mR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Aperture 16M \\[A16M\\]
Indicates that the BAR aperture can be set to 16M."]
    #[inline(always)]
    pub fn a16m(&self) -> A16mR {
        A16mR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Aperture 32M \\[A32M\\]
Indicates that the BAR aperture can be set to 32M."]
    #[inline(always)]
    pub fn a32m(&self) -> A32mR {
        A32mR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Aperture 64M \\[A64M\\]
Indicates that the BAR aperture can be set to 64M."]
    #[inline(always)]
    pub fn a64m(&self) -> A64mR {
        A64mR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Aperture 128M \\[A128M\\]
Indicates that the BAR aperture can be set to 128M."]
    #[inline(always)]
    pub fn a128m(&self) -> A128mR {
        A128mR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Aperture 256M \\[A256M\\]
Indicates that the BAR aperture can be set to 256M."]
    #[inline(always)]
    pub fn a256m(&self) -> A256mR {
        A256mR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Aperture 512M \\[A512M\\]
Indicates that the BAR aperture can be set to 512M."]
    #[inline(always)]
    pub fn a512m(&self) -> A512mR {
        A512mR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Aperture 1G \\[A1G\\]
Indicates that the BAR aperture can be set to 1G."]
    #[inline(always)]
    pub fn a1g(&self) -> A1gR {
        A1gR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Aperture 2G \\[A2G\\]
Indicates that the BAR aperture can be set to 2G."]
    #[inline(always)]
    pub fn a2g(&self) -> A2gR {
        A2gR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Aperture 4G \\[A4G\\]
Indicates that the BAR aperture can be set to 4G."]
    #[inline(always)]
    pub fn a4g(&self) -> A4gR {
        A4gR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Aperture 8G \\[A8G\\]
Indicates that the BAR aperture can be set to 8G."]
    #[inline(always)]
    pub fn a8g(&self) -> A8gR {
        A8gR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Aperture 16G \\[A16G\\]
Indicates that the BAR aperture can be set to 16G."]
    #[inline(always)]
    pub fn a16g(&self) -> A16gR {
        A16gR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Aperture 32G \\[A32G\\]
Indicates that the BAR aperture can be set to 32G."]
    #[inline(always)]
    pub fn a32g(&self) -> A32gR {
        A32gR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Aperture 64G \\[A64G\\]
Indicates that the BAR aperture can be set to 64G."]
    #[inline(always)]
    pub fn a64g(&self) -> A64gR {
        A64gR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Aperture 128G \\[A128G\\]
Indicates that the BAR aperture can be set to 128G."]
    #[inline(always)]
    pub fn a128g(&self) -> A128gR {
        A128gR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Aperture 256G \\[A256G\\]
Indicates that the BAR aperture can be set to 256G."]
    #[inline(always)]
    pub fn a256g(&self) -> A256gR {
        A256gR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Aperture 512G \\[A512G\\]
Indicates that the BAR aperture can be set to 512G."]
    #[inline(always)]
    pub fn a512g(&self) -> A512gR {
        A512gR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Reserved \\[R1\\]
Reserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Resizable BAR Capability Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_capability_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfResizableBarCapability2Spec;
impl crate::RegisterSpec for PciePfResizableBarCapability2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_resizable_bar_capability_2::R`](R) reader structure"]
impl crate::Readable for PciePfResizableBarCapability2Spec {}
#[doc = "`reset()` method sets PCIE_PF_RESIZABLE_BAR_CAPABILITY_2 to value 0"]
impl crate::Resettable for PciePfResizableBarCapability2Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PCIE_PF_RESIZABLE_BAR_CONTROL_2` reader"]
pub type R = crate::R<PciePfResizableBarControl2Spec>;
#[doc = "Field `BARI` reader - BAR Index \\[BARI\\]\n\nSpecifies the index of the BAR controlled by\n\nthis register. This field can be modified\n\nindependently for each PF from the local\n\nmanagement bus."]
pub type BariR = crate::FieldReader;
#[doc = "Field `R2` reader - Reserved \\[R2\\]\n\nReserved"]
pub type R2R = crate::FieldReader;
#[doc = "Field `RBARC` reader - Resizable BAR Count \\[RBARC\\]\n\nSpecifies the number of BARs that can be\n\nconfigured through the Resizable BAR\n\nCapability Structure for this PF. This field\n\ncan be modified independently for each PF\n\nfrom the local management bus."]
pub type RbarcR = crate::FieldReader;
#[doc = "Field `BARS` reader - BAR Size \\[BARS\\]\n\nWhen the Resizable BAR Capability is\n\nenabled for the Physical Function, this field\n\ncontrols the BAR aperture for the first BAR\n\nof the PF (0 = 1M, 1 = 2M, ... , 12 = 4G).\n\nThis field can be modified independently for\n\neach PF from the local management bus."]
pub type BarsR = crate::FieldReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]\n\nReserved"]
pub type R3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - BAR Index \\[BARI\\]\n\nSpecifies the index of the BAR controlled by\n\nthis register. This field can be modified\n\nindependently for each PF from the local\n\nmanagement bus."]
    #[inline(always)]
    pub fn bari(&self) -> BariR {
        BariR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Reserved \\[R2\\]\n\nReserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Resizable BAR Count \\[RBARC\\]\n\nSpecifies the number of BARs that can be\n\nconfigured through the Resizable BAR\n\nCapability Structure for this PF. This field\n\ncan be modified independently for each PF\n\nfrom the local management bus."]
    #[inline(always)]
    pub fn rbarc(&self) -> RbarcR {
        RbarcR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - BAR Size \\[BARS\\]\n\nWhen the Resizable BAR Capability is\n\nenabled for the Physical Function, this field\n\ncontrols the BAR aperture for the first BAR\n\nof the PF (0 = 1M, 1 = 2M, ... , 12 = 4G).\n\nThis field can be modified independently for\n\neach PF from the local management bus."]
    #[inline(always)]
    pub fn bars(&self) -> BarsR {
        BarsR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:31 - Reserved \\[R3\\]\n\nReserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[doc = "Resizable BAR Control Register 2\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_control_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfResizableBarControl2Spec;
impl crate::RegisterSpec for PciePfResizableBarControl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_resizable_bar_control_2::R`](R) reader structure"]
impl crate::Readable for PciePfResizableBarControl2Spec {}
#[doc = "`reset()` method sets PCIE_PF_RESIZABLE_BAR_CONTROL_2 to value 0"]
impl crate::Resettable for PciePfResizableBarControl2Spec {
    const RESET_VALUE: u32 = 0;
}

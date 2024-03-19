#[doc = "Register `PCIE_PF_RESIZABLE_BAR_CONTROL_3` reader"]
pub type R = crate::R<PciePfResizableBarControl3Spec>;
#[doc = "Field `BARI` reader - BAR Index \\[BARI\\]
Specifies the index of the BAR controlled by this register. This field can be modified independently for each PF from the local management bus."]
pub type BariR = crate::FieldReader;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::FieldReader;
#[doc = "Field `RBARC` reader - Resizable BAR Count \\[RBARC\\]
Specifies the number of BARs that can be configured through the Resizable BAR Capability Structure for this PF. This field can be modified independently for each PF from the local management bus."]
pub type RbarcR = crate::FieldReader;
#[doc = "Field `BARS` reader - BAR Size \\[BARS\\]
When the Resizable BAR Capability is enabled for the Physical Function, this field controls the BAR aperture for the first BAR of the PF (0 = 1M, 1 = 2M, ... , 12 = 4G). This field can be modified independently for each PF from the local management bus."]
pub type BarsR = crate::FieldReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - BAR Index \\[BARI\\]
Specifies the index of the BAR controlled by this register. This field can be modified independently for each PF from the local management bus."]
    #[inline(always)]
    pub fn bari(&self) -> BariR {
        BariR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Reserved \\[R2\\]
Reserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Resizable BAR Count \\[RBARC\\]
Specifies the number of BARs that can be configured through the Resizable BAR Capability Structure for this PF. This field can be modified independently for each PF from the local management bus."]
    #[inline(always)]
    pub fn rbarc(&self) -> RbarcR {
        RbarcR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - BAR Size \\[BARS\\]
When the Resizable BAR Capability is enabled for the Physical Function, this field controls the BAR aperture for the first BAR of the PF (0 = 1M, 1 = 2M, ... , 12 = 4G). This field can be modified independently for each PF from the local management bus."]
    #[inline(always)]
    pub fn bars(&self) -> BarsR {
        BarsR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:31 - Reserved \\[R3\\]
Reserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[doc = "Resizable BAR Control Register 3 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_resizable_bar_control_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfResizableBarControl3Spec;
impl crate::RegisterSpec for PciePfResizableBarControl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_resizable_bar_control_3::R`](R) reader structure"]
impl crate::Readable for PciePfResizableBarControl3Spec {}
#[doc = "`reset()` method sets PCIE_PF_RESIZABLE_BAR_CONTROL_3 to value 0"]
impl crate::Resettable for PciePfResizableBarControl3Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PCIE_PF_MSI_PENDING_BITS` reader"]
pub type R = crate::R<PciePfMsiPendingBitsSpec>;
#[doc = "Field `MP` reader - MSI Pending Bits \\[MP\\]\n\nPending bits for MSI interrupts. This\n\nfield can be written from the APB\n\ninterface to reflect the current\n\npending status. The Multiple\n\nMessage Capable field of the MSI\n\nControl Register specifies the\n\nnumber of distinct interrupts for the\n\nFunction, which determines the\n\nnumber of valid pending bits. Please\n\nnote that if the Multiple Message\n\nCapable field is changed from the\n\nlocal management APB bus, then the\n\nwidth of the MSI Pending Bits field\n\nalso changes correspondingly"]
pub type MpR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nPlease note that if the Multiple\n\nMessage Capable field is changed\n\nfrom the local management APB bus,\n\nthen the width of this field also\n\nchanges correspondingly"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - MSI Pending Bits \\[MP\\]\n\nPending bits for MSI interrupts. This\n\nfield can be written from the APB\n\ninterface to reflect the current\n\npending status. The Multiple\n\nMessage Capable field of the MSI\n\nControl Register specifies the\n\nnumber of distinct interrupts for the\n\nFunction, which determines the\n\nnumber of valid pending bits. Please\n\nnote that if the Multiple Message\n\nCapable field is changed from the\n\nlocal management APB bus, then the\n\nwidth of the MSI Pending Bits field\n\nalso changes correspondingly"]
    #[inline(always)]
    pub fn mp(&self) -> MpR {
        MpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved \\[R0\\]\n\nPlease note that if the Multiple\n\nMessage Capable field is changed\n\nfrom the local management APB bus,\n\nthen the width of this field also\n\nchanges correspondingly"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[doc = "MSI Pending Bits Register\n\nPlease note that if the Multiple\n\nMessage Capable field is changed\n\nfrom the local management APB bus,\n\nthen the width of this field also\n\nchanges correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_pending_bits::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfMsiPendingBitsSpec;
impl crate::RegisterSpec for PciePfMsiPendingBitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_msi_pending_bits::R`](R) reader structure"]
impl crate::Readable for PciePfMsiPendingBitsSpec {}
#[doc = "`reset()` method sets PCIE_PF_MSI_PENDING_BITS to value 0"]
impl crate::Resettable for PciePfMsiPendingBitsSpec {
    const RESET_VALUE: u32 = 0;
}

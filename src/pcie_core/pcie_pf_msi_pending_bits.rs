#[doc = "Register `PCIE_PF_MSI_PENDING_BITS` reader"]
pub type R = crate::R<PciePfMsiPendingBitsSpec>;
#[doc = "Field `MP` reader - MSI Pending Bits \\[MP\\]
Pending bits for MSI interrupts. This field can be written from the APB interface to reflect the current pending status. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts for the Function, which determines the number of valid pending bits. Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of the MSI Pending Bits field also changes correspondingly"]
pub type MpR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - MSI Pending Bits \\[MP\\]
Pending bits for MSI interrupts. This field can be written from the APB interface to reflect the current pending status. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts for the Function, which determines the number of valid pending bits. Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of the MSI Pending Bits field also changes correspondingly"]
    #[inline(always)]
    pub fn mp(&self) -> MpR {
        MpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved \\[R0\\]
Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[doc = "MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_pending_bits::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

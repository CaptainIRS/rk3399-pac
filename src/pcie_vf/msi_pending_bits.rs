#[doc = "Register `MSI_PENDING_BITS` reader"]
pub type R = crate::R<MsiPendingBitsSpec>;
#[doc = "Field `MP` reader - MSI Pending Bits \\[MP\\]
Pending bits for MSI interrupts. This register contains the MSI pending interrupt bits, one for each of the interrupt levels. This field can be written from the local management APB bus. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts for the Function, which determines the number of valid pending bits. Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of the MSI Pending Bits field also changes correspondingly"]
pub type MpR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - MSI Pending Bits \\[MP\\]
Pending bits for MSI interrupts. This register contains the MSI pending interrupt bits, one for each of the interrupt levels. This field can be written from the local management APB bus. The Multiple Message Capable field of the MSI Control Register specifies the number of distinct interrupts for the Function, which determines the number of valid pending bits. Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of the MSI Pending Bits field also changes correspondingly"]
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
#[doc = "MSI Pending Bits Register Please note that if the Multiple Message Capable field is changed from the local management APB bus, then the width of this field also changes correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msi_pending_bits::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsiPendingBitsSpec;
impl crate::RegisterSpec for MsiPendingBitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msi_pending_bits::R`](R) reader structure"]
impl crate::Readable for MsiPendingBitsSpec {}
#[doc = "`reset()` method sets MSI_PENDING_BITS to value 0"]
impl crate::Resettable for MsiPendingBitsSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PCIE_VF_MSI_PENDING_BITS` reader"]
pub type R = crate::R<PcieVfMsiPendingBitsSpec>;
#[doc = "Field `MP` reader - MSI Pending Bits \\[MP\\]\n\nPending bits for MSI interrupts. This\n\nregister contains the MSI pending\n\ninterrupt bits, one for each of the\n\ninterrupt levels. This field can be\n\nwritten from the local management\n\nAPB bus. The Multiple Message\n\nCapable field of the MSI Control\n\nRegister specifies the number of\n\ndistinct interrupts for the Function,\n\nwhich determines the number of\n\nvalid pending bits. Please note that if\n\nthe Multiple Message Capable field is\n\nchanged from the local management\n\nAPB bus, then the width of the MSI\n\nPending Bits field also changes\n\ncorrespondingly"]
pub type MpR = crate::BitReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nPlease note that if the Multiple\n\nMessage Capable field is changed\n\nfrom the local management APB bus,\n\nthen the width of this field also\n\nchanges correspondingly"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - MSI Pending Bits \\[MP\\]\n\nPending bits for MSI interrupts. This\n\nregister contains the MSI pending\n\ninterrupt bits, one for each of the\n\ninterrupt levels. This field can be\n\nwritten from the local management\n\nAPB bus. The Multiple Message\n\nCapable field of the MSI Control\n\nRegister specifies the number of\n\ndistinct interrupts for the Function,\n\nwhich determines the number of\n\nvalid pending bits. Please note that if\n\nthe Multiple Message Capable field is\n\nchanged from the local management\n\nAPB bus, then the width of the MSI\n\nPending Bits field also changes\n\ncorrespondingly"]
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
#[doc = "MSI Pending Bits Register\n\nPlease note that if the Multiple\n\nMessage Capable field is changed\n\nfrom the local management APB bus,\n\nthen the width of this field also\n\nchanges correspondingly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_pending_bits::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfMsiPendingBitsSpec;
impl crate::RegisterSpec for PcieVfMsiPendingBitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_msi_pending_bits::R`](R) reader structure"]
impl crate::Readable for PcieVfMsiPendingBitsSpec {}
#[doc = "`reset()` method sets PCIE_VF_MSI_PENDING_BITS to value 0"]
impl crate::Resettable for PcieVfMsiPendingBitsSpec {
    const RESET_VALUE: u32 = 0;
}

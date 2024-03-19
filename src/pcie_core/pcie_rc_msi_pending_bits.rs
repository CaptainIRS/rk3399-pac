#[doc = "Register `PCIE_RC_MSI_PENDING_BITS` reader"]
pub type R = crate::R<PcieRcMsiPendingBitsSpec>;
#[doc = "Field `MP` reader - MSI Pending Bits \\[MP\\]
Pending bits for MSI interrupts. This field can be written from the APB interface to refelct the current pending status."]
pub type MpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MSI Pending Bits \\[MP\\]
Pending bits for MSI interrupts. This field can be written from the APB interface to refelct the current pending status."]
    #[inline(always)]
    pub fn mp(&self) -> MpR {
        MpR::new((self.bits & 1) != 0)
    }
}
#[doc = "MSI Pending Bits Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_msi_pending_bits::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcMsiPendingBitsSpec;
impl crate::RegisterSpec for PcieRcMsiPendingBitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_msi_pending_bits::R`](R) reader structure"]
impl crate::Readable for PcieRcMsiPendingBitsSpec {}
#[doc = "`reset()` method sets PCIE_RC_MSI_PENDING_BITS to value 0"]
impl crate::Resettable for PcieRcMsiPendingBitsSpec {
    const RESET_VALUE: u32 = 0;
}

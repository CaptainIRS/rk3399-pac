#[doc = "Register `PCIE_PF_MSI_X_PENDING_INTERRUPT` reader"]
pub type R = crate::R<PciePfMsiXPendingInterruptSpec>;
#[doc = "Field `BARI1` reader - BAR Indicator Register \\[BARI1\\]
Identifies the BAR corresponding to the memory address range where the PBA Structure is located (000 = BAR 0, 001 = BAR 1, ... , 101 = BAR 5). The value programmed must be the same as the BAR Indicator configured in the MSI-X Table Offset Register.Identifies the BAR corresponding to the memory address range where the PBA Structure is located (000 = BAR 0, 001 = BAR1, ... , 101 = BAR 5). The value programmed must be the same as the BAR Indicator configured in the MSI-X Table Offset Register."]
pub type Bari1R = crate::FieldReader;
#[doc = "Field `PBAO` reader - PBA Offset \\[PBAO\\]
Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub type PbaoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - BAR Indicator Register \\[BARI1\\]
Identifies the BAR corresponding to the memory address range where the PBA Structure is located (000 = BAR 0, 001 = BAR 1, ... , 101 = BAR 5). The value programmed must be the same as the BAR Indicator configured in the MSI-X Table Offset Register.Identifies the BAR corresponding to the memory address range where the PBA Structure is located (000 = BAR 0, 001 = BAR1, ... , 101 = BAR 5). The value programmed must be the same as the BAR Indicator configured in the MSI-X Table Offset Register."]
    #[inline(always)]
    pub fn bari1(&self) -> Bari1R {
        Bari1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - PBA Offset \\[PBAO\\]
Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub fn pbao(&self) -> PbaoR {
        PbaoR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[doc = "MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_x_pending_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfMsiXPendingInterruptSpec;
impl crate::RegisterSpec for PciePfMsiXPendingInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_msi_x_pending_interrupt::R`](R) reader structure"]
impl crate::Readable for PciePfMsiXPendingInterruptSpec {}
#[doc = "`reset()` method sets PCIE_PF_MSI_X_PENDING_INTERRUPT to value 0x08"]
impl crate::Resettable for PciePfMsiXPendingInterruptSpec {
    const RESET_VALUE: u32 = 0x08;
}

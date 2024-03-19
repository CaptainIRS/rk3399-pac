#[doc = "Register `PCIE_VF_MSI_X_PENDING_INTERRUPT` reader"]
pub type R = crate::R<PcieVfMsiXPendingInterruptSpec>;
#[doc = "Field `BARI` reader - BAR Indicator Register \\[BARI\\]
Identifies the BAR corresponding to the memory address range where the PBA Structure is located (000 = BAR 0, 001 = BAR 1, ... , 101 = BAR 5). The value programmed must be the same as the BAR Indicator configured in the MSI-X TableOffset Register.Identifies the BAR corresponding to the memory address range where the PBA Structure is located (000 = BAR 0, 001 = BAR1, ... , 101 = BAR 5). The value programmed must be the same as the BAR Indicator configured in the MSI-X Table Offset Register."]
pub type BariR = crate::FieldReader;
#[doc = "Field `PO` reader - PBA Offset \\[PO\\]
Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub type PoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - BAR Indicator Register \\[BARI\\]
Identifies the BAR corresponding to the memory address range where the PBA Structure is located (000 = BAR 0, 001 = BAR 1, ... , 101 = BAR 5). The value programmed must be the same as the BAR Indicator configured in the MSI-X TableOffset Register.Identifies the BAR corresponding to the memory address range where the PBA Structure is located (000 = BAR 0, 001 = BAR1, ... , 101 = BAR 5). The value programmed must be the same as the BAR Indicator configured in the MSI-X Table Offset Register."]
    #[inline(always)]
    pub fn bari(&self) -> BariR {
        BariR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - PBA Offset \\[PO\\]
Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub fn po(&self) -> PoR {
        PoR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[doc = "MSI-X Pending Interrupt Register Offset of the memory address where the PBA is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_x_pending_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfMsiXPendingInterruptSpec;
impl crate::RegisterSpec for PcieVfMsiXPendingInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_msi_x_pending_interrupt::R`](R) reader structure"]
impl crate::Readable for PcieVfMsiXPendingInterruptSpec {}
#[doc = "`reset()` method sets PCIE_VF_MSI_X_PENDING_INTERRUPT to value 0x08"]
impl crate::Resettable for PcieVfMsiXPendingInterruptSpec {
    const RESET_VALUE: u32 = 0x08;
}

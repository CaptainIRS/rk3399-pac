#[doc = "Register `PCIE_VF_MSI_X_PENDING_INTERRUPT` reader"]
pub type R = crate::R<PcieVfMsiXPendingInterruptSpec>;
#[doc = "Field `BARI` reader - BAR Indicator Register \\[BARI\\]\n\nIdentifies the BAR corresponding to\n\nthe memory address range where\n\nthe PBA Structure is located (000 =\n\nBAR 0, 001 = BAR 1, ... , 101 = BAR\n\n5).\n\nThe value programmed must be the\n\nsame as the BAR Indicator\n\nconfigured in the MSI-X TableOffset\n\nRegister.Identifies the BAR\n\ncorresponding to the memory\n\naddress range where the PBA\n\nStructure is located (000 = BAR 0,\n\n001 = BAR1, ... , 101 = BAR 5).\n\nThe value programmed must be the\n\nsame as the BAR Indicator\n\nconfigured in the MSI-X Table Offset\n\nRegister."]
pub type BariR = crate::FieldReader;
#[doc = "Field `PO` reader - PBA Offset \\[PO\\]\n\nOffset of the memory address where\n\nthe PBA is located, relative to the\n\nselected BAR. The three least\n\nsignificant bits of the address are\n\nomitted, as the addresses are\n\nQWORD aligned."]
pub type PoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - BAR Indicator Register \\[BARI\\]\n\nIdentifies the BAR corresponding to\n\nthe memory address range where\n\nthe PBA Structure is located (000 =\n\nBAR 0, 001 = BAR 1, ... , 101 = BAR\n\n5).\n\nThe value programmed must be the\n\nsame as the BAR Indicator\n\nconfigured in the MSI-X TableOffset\n\nRegister.Identifies the BAR\n\ncorresponding to the memory\n\naddress range where the PBA\n\nStructure is located (000 = BAR 0,\n\n001 = BAR1, ... , 101 = BAR 5).\n\nThe value programmed must be the\n\nsame as the BAR Indicator\n\nconfigured in the MSI-X Table Offset\n\nRegister."]
    #[inline(always)]
    pub fn bari(&self) -> BariR {
        BariR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - PBA Offset \\[PO\\]\n\nOffset of the memory address where\n\nthe PBA is located, relative to the\n\nselected BAR. The three least\n\nsignificant bits of the address are\n\nomitted, as the addresses are\n\nQWORD aligned."]
    #[inline(always)]
    pub fn po(&self) -> PoR {
        PoR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[doc = "MSI-X Pending Interrupt Register\n\nOffset of the memory address where\n\nthe PBA is located, relative to the\n\nselected BAR. The three least\n\nsignificant bits of the address are\n\nomitted, as the addresses are\n\nQWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_x_pending_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

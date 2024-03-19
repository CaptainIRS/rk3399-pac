#[doc = "Register `PCIE_PF_MSI_X_PENDING_INTERRUPT` reader"]
pub type R = crate::R<PciePfMsiXPendingInterruptSpec>;
#[doc = "Field `BARI1` reader - BAR Indicator Register \\[BARI1\\]\n\nIdentifies the BAR corresponding to\n\nthe memory address range where\n\nthe PBA Structure is located (000 =\n\nBAR 0, 001 = BAR 1, ... , 101 = BAR\n\n5). The value programmed must be\n\nthe same as the BAR Indicator\n\nconfigured in the MSI-X Table Offset\n\nRegister.Identifies the BAR\n\ncorresponding to the memory\n\naddress range where the PBA\n\nStructure is located (000 = BAR 0,\n\n001 = BAR1, ... , 101 = BAR 5). The\n\nvalue programmed must be the\n\nsame as the BAR Indicator\n\nconfigured in the MSI-X Table Offset\n\nRegister."]
pub type Bari1R = crate::FieldReader;
#[doc = "Field `PBAO` reader - PBA Offset \\[PBAO\\]\n\nOffset of the memory address where\n\nthe PBA is located, relative to the\n\nselected BAR. The three least\n\nsignificant bits of the address are\n\nomitted, as the addresses are\n\nQWORD aligned."]
pub type PbaoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - BAR Indicator Register \\[BARI1\\]\n\nIdentifies the BAR corresponding to\n\nthe memory address range where\n\nthe PBA Structure is located (000 =\n\nBAR 0, 001 = BAR 1, ... , 101 = BAR\n\n5). The value programmed must be\n\nthe same as the BAR Indicator\n\nconfigured in the MSI-X Table Offset\n\nRegister.Identifies the BAR\n\ncorresponding to the memory\n\naddress range where the PBA\n\nStructure is located (000 = BAR 0,\n\n001 = BAR1, ... , 101 = BAR 5). The\n\nvalue programmed must be the\n\nsame as the BAR Indicator\n\nconfigured in the MSI-X Table Offset\n\nRegister."]
    #[inline(always)]
    pub fn bari1(&self) -> Bari1R {
        Bari1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - PBA Offset \\[PBAO\\]\n\nOffset of the memory address where\n\nthe PBA is located, relative to the\n\nselected BAR. The three least\n\nsignificant bits of the address are\n\nomitted, as the addresses are\n\nQWORD aligned."]
    #[inline(always)]
    pub fn pbao(&self) -> PbaoR {
        PbaoR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[doc = "MSI-X Pending Interrupt Register\n\nOffset of the memory address where\n\nthe PBA is located, relative to the\n\nselected BAR. The three least\n\nsignificant bits of the address are\n\nomitted, as the addresses are\n\nQWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_msi_x_pending_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

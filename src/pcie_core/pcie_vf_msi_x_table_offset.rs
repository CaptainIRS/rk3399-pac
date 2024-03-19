#[doc = "Register `PCIE_VF_MSI_X_TABLE_OFFSET` reader"]
pub type R = crate::R<PcieVfMsiXTableOffsetSpec>;
#[doc = "Field `BARI` reader - BAR Indicator Register \\[BARI\\]
Identifies the BAR corresponding to the memory address range where the MSI-X Table is located (000 = BAR 0, 001 = BAR 1, ... , 101 = BAR 5)."]
pub type BariR = crate::FieldReader;
#[doc = "Field `TO` reader - Table Offset \\[TO\\]
Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
pub type ToR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - BAR Indicator Register \\[BARI\\]
Identifies the BAR corresponding to the memory address range where the MSI-X Table is located (000 = BAR 0, 001 = BAR 1, ... , 101 = BAR 5)."]
    #[inline(always)]
    pub fn bari(&self) -> BariR {
        BariR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - Table Offset \\[TO\\]
Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned."]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[doc = "MSI-X Table Offset Register Offset of the memory address where the MSI- X Table is located, relative to the selected BAR. The three least significant bits of the address are omitted, as the addresses are QWORD aligned.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_msi_x_table_offset::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfMsiXTableOffsetSpec;
impl crate::RegisterSpec for PcieVfMsiXTableOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_msi_x_table_offset::R`](R) reader structure"]
impl crate::Readable for PcieVfMsiXTableOffsetSpec {}
#[doc = "`reset()` method sets PCIE_VF_MSI_X_TABLE_OFFSET to value 0"]
impl crate::Resettable for PcieVfMsiXTableOffsetSpec {
    const RESET_VALUE: u32 = 0;
}

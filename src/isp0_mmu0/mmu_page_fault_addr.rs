#[doc = "Register `MMU_PAGE_FAULT_ADDR` reader"]
pub type R = crate::R<MmuPageFaultAddrSpec>;
#[doc = "Field `MMU_PAGE_FAULT_ADDR` reader - address of last page fault"]
pub type MmuPageFaultAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - address of last page fault"]
    #[inline(always)]
    pub fn mmu_page_fault_addr(&self) -> MmuPageFaultAddrR {
        MmuPageFaultAddrR::new(self.bits)
    }
}
#[doc = "MMU logic address of last page fault register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_page_fault_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuPageFaultAddrSpec;
impl crate::RegisterSpec for MmuPageFaultAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_page_fault_addr::R`](R) reader structure"]
impl crate::Readable for MmuPageFaultAddrSpec {}
#[doc = "`reset()` method sets MMU_PAGE_FAULT_ADDR to value 0"]
impl crate::Resettable for MmuPageFaultAddrSpec {
    const RESET_VALUE: u32 = 0;
}

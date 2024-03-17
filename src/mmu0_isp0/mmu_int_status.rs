#[doc = "Register `MMU_INT_STATUS` reader"]
pub type R = crate::R<MmuIntStatusSpec>;
#[doc = "Register `MMU_INT_STATUS` writer"]
pub type W = crate::W<MmuIntStatusSpec>;
#[doc = "Field `PAGE_FAULT` reader - page fault interrupt"]
pub type PageFaultR = crate::BitReader;
#[doc = "Field `READ_BUS_ERROR` reader - read bus error interrupt"]
pub type ReadBusErrorR = crate::BitReader;
#[doc = "Field `READ_BUS_ERROR` writer - read bus error interrupt"]
pub type ReadBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - page fault interrupt"]
    #[inline(always)]
    pub fn page_fault(&self) -> PageFaultR {
        PageFaultR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - read bus error interrupt"]
    #[inline(always)]
    pub fn read_bus_error(&self) -> ReadBusErrorR {
        ReadBusErrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - read bus error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn read_bus_error(&mut self) -> ReadBusErrorW<MmuIntStatusSpec> {
        ReadBusErrorW::new(self, 1)
    }
}
#[doc = "MMU interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_int_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_int_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuIntStatusSpec;
impl crate::RegisterSpec for MmuIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_int_status::R`](R) reader structure"]
impl crate::Readable for MmuIntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mmu_int_status::W`](W) writer structure"]
impl crate::Writable for MmuIntStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMU_INT_STATUS to value 0"]
impl crate::Resettable for MmuIntStatusSpec {
    const RESET_VALUE: u32 = 0;
}

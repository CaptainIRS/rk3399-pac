#[doc = "Register `INT_MASK` reader"]
pub type R = crate::R<IntMaskSpec>;
#[doc = "Register `INT_MASK` writer"]
pub type W = crate::W<IntMaskSpec>;
#[doc = "Field `PAGE_FAULT_INT_EN` reader - page fault interrupt enable"]
pub type PageFaultIntEnR = crate::BitReader;
#[doc = "Field `PAGE_FAULT_INT_EN` writer - page fault interrupt enable"]
pub type PageFaultIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_BUS_ERROR_INT_EN` reader - read bus error interrupt enable"]
pub type ReadBusErrorIntEnR = crate::BitReader;
#[doc = "Field `READ_BUS_ERROR_INT_EN` writer - read bus error interrupt enable"]
pub type ReadBusErrorIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - page fault interrupt enable"]
    #[inline(always)]
    pub fn page_fault_int_en(&self) -> PageFaultIntEnR {
        PageFaultIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - read bus error interrupt enable"]
    #[inline(always)]
    pub fn read_bus_error_int_en(&self) -> ReadBusErrorIntEnR {
        ReadBusErrorIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - page fault interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn page_fault_int_en(&mut self) -> PageFaultIntEnW<IntMaskSpec> {
        PageFaultIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - read bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn read_bus_error_int_en(&mut self) -> ReadBusErrorIntEnW<IntMaskSpec> {
        ReadBusErrorIntEnW::new(self, 1)
    }
}
#[doc = "MMU interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMaskSpec;
impl crate::RegisterSpec for IntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_mask::R`](R) reader structure"]
impl crate::Readable for IntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_mask::W`](W) writer structure"]
impl crate::Writable for IntMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_MASK to value 0"]
impl crate::Resettable for IntMaskSpec {
    const RESET_VALUE: u32 = 0;
}

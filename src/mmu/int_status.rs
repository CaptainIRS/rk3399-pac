#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Register `INT_STATUS` writer"]
pub type W = crate::W<IntStatusSpec>;
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
    pub fn read_bus_error(&mut self) -> ReadBusErrorW<IntStatusSpec> {
        ReadBusErrorW::new(self, 1)
    }
}
#[doc = "MMU interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`int_status::W`](W) writer structure"]
impl crate::Writable for IntStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {
    const RESET_VALUE: u32 = 0;
}

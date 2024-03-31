#[doc = "Register `INT_CLEAR` reader"]
pub type R = crate::R<IntClearSpec>;
#[doc = "Register `INT_CLEAR` writer"]
pub type W = crate::W<IntClearSpec>;
#[doc = "Field `PAGE_FAULT_CLEAR` reader - page fault interrupt clear, write 1 to this\n\nregister can clear page fault interrupt."]
pub type PageFaultClearR = crate::BitReader;
#[doc = "Field `PAGE_FAULT_CLEAR` writer - page fault interrupt clear, write 1 to this\n\nregister can clear page fault interrupt."]
pub type PageFaultClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_BUS_ERROR_CLEAR` reader - read bus error interrupt clear. write 1 to this\n\nregister can clear read bus error interrupt."]
pub type ReadBusErrorClearR = crate::BitReader;
#[doc = "Field `READ_BUS_ERROR_CLEAR` writer - read bus error interrupt clear. write 1 to this\n\nregister can clear read bus error interrupt."]
pub type ReadBusErrorClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - page fault interrupt clear, write 1 to this\n\nregister can clear page fault interrupt."]
    #[inline(always)]
    pub fn page_fault_clear(&self) -> PageFaultClearR {
        PageFaultClearR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - read bus error interrupt clear. write 1 to this\n\nregister can clear read bus error interrupt."]
    #[inline(always)]
    pub fn read_bus_error_clear(&self) -> ReadBusErrorClearR {
        ReadBusErrorClearR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - page fault interrupt clear, write 1 to this\n\nregister can clear page fault interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn page_fault_clear(&mut self) -> PageFaultClearW<IntClearSpec> {
        PageFaultClearW::new(self, 0)
    }
    #[doc = "Bit 1 - read bus error interrupt clear. write 1 to this\n\nregister can clear read bus error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn read_bus_error_clear(&mut self) -> ReadBusErrorClearW<IntClearSpec> {
        ReadBusErrorClearW::new(self, 1)
    }
}
#[doc = "MMU interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClearSpec;
impl crate::RegisterSpec for IntClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_clear::R`](R) reader structure"]
impl crate::Readable for IntClearSpec {}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for IntClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0"]
impl crate::Resettable for IntClearSpec {
    const RESET_VALUE: u32 = 0;
}

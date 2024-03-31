#[doc = "Register `FIFO_BASE` reader"]
pub type R = crate::R<FifoBaseSpec>;
#[doc = "Register `FIFO_BASE` writer"]
pub type W = crate::W<FifoBaseSpec>;
#[doc = "Field `FIFO_BASE_ADDR` reader - FIFO base address"]
pub type FifoBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `FIFO_BASE_ADDR` writer - FIFO base address"]
pub type FifoBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO base address"]
    #[inline(always)]
    pub fn fifo_base_addr(&self) -> FifoBaseAddrR {
        FifoBaseAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO base address"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_base_addr(&mut self) -> FifoBaseAddrW<FifoBaseSpec> {
        FifoBaseAddrW::new(self, 0)
    }
}
#[doc = "FIFO base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoBaseSpec;
impl crate::RegisterSpec for FifoBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_base::R`](R) reader structure"]
impl crate::Readable for FifoBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_base::W`](W) writer structure"]
impl crate::Writable for FifoBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_BASE to value 0"]
impl crate::Resettable for FifoBaseSpec {
    const RESET_VALUE: u32 = 0;
}

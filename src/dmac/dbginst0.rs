#[doc = "Register `DBGINST0` writer"]
pub type W = crate::W<Dbginst0Spec>;
#[doc = "Field `DBGINST0_BITS_5` writer - The debug thread encoding is as follows:\n\n0 = DMA manager thread\n\n1 = DMA channel."]
pub type Dbginst0Bits5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGINST0_BITS_3` writer - DMA channel number:\n\nb000 = DMA channel 0\n\nb001 = DMA channel 1\n\nb010 = DMA channel 2\n\n...\n\nb111 = DMA channel 7"]
pub type Dbginst0Bits3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBGINST0_BITS_1` writer - Instruction byte 0"]
pub type Dbginst0Bits1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBGINST0_BITS_0` writer - Instruction byte 1"]
pub type Dbginst0Bits0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bit 0 - The debug thread encoding is as follows:\n\n0 = DMA manager thread\n\n1 = DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn dbginst0_bits_5(&mut self) -> Dbginst0Bits5W<Dbginst0Spec> {
        Dbginst0Bits5W::new(self, 0)
    }
    #[doc = "Bits 8:10 - DMA channel number:\n\nb000 = DMA channel 0\n\nb001 = DMA channel 1\n\nb010 = DMA channel 2\n\n...\n\nb111 = DMA channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn dbginst0_bits_3(&mut self) -> Dbginst0Bits3W<Dbginst0Spec> {
        Dbginst0Bits3W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Instruction byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn dbginst0_bits_1(&mut self) -> Dbginst0Bits1W<Dbginst0Spec> {
        Dbginst0Bits1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Instruction byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn dbginst0_bits_0(&mut self) -> Dbginst0Bits0W<Dbginst0Spec> {
        Dbginst0Bits0W::new(self, 24)
    }
}
#[doc = "Debug Instruction-0 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbginst0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbginst0Spec;
impl crate::RegisterSpec for Dbginst0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dbginst0::W`](W) writer structure"]
impl crate::Writable for Dbginst0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGINST0 to value 0"]
impl crate::Resettable for Dbginst0Spec {
    const RESET_VALUE: u32 = 0;
}

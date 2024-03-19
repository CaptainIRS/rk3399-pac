#[doc = "Register `GP_MASK` reader"]
pub type R = crate::R<GpMaskSpec>;
#[doc = "Register `GP_MASK` writer"]
pub type W = crate::W<GpMaskSpec>;
#[doc = "Field `FIFO_FULL_MASK` reader - FIFO full flag mask"]
pub type FifoFullMaskR = crate::BitReader;
#[doc = "Field `FIFO_FULL_MASK` writer - FIFO full flag mask"]
pub type FifoFullMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_EMPTY_MASK` reader - FIFO empty flag mask"]
pub type FifoEmptyMaskR = crate::BitReader;
#[doc = "Field `FIFO_EMPTY_MASK` writer - FIFO empty flag mask"]
pub type FifoEmptyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_OVERRUN_MASK` reader - FIFO overrun mask"]
pub type FifoOverrunMaskR = crate::BitReader;
#[doc = "Field `FIFO_OVERRUN_MASK` writer - FIFO overrun mask"]
pub type FifoOverrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO full flag mask"]
    #[inline(always)]
    pub fn fifo_full_mask(&self) -> FifoFullMaskR {
        FifoFullMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO empty flag mask"]
    #[inline(always)]
    pub fn fifo_empty_mask(&self) -> FifoEmptyMaskR {
        FifoEmptyMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO overrun mask"]
    #[inline(always)]
    pub fn fifo_overrun_mask(&self) -> FifoOverrunMaskR {
        FifoOverrunMaskR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO full flag mask"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_full_mask(&mut self) -> FifoFullMaskW<GpMaskSpec> {
        FifoFullMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO empty flag mask"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_empty_mask(&mut self) -> FifoEmptyMaskW<GpMaskSpec> {
        FifoEmptyMaskW::new(self, 1)
    }
    #[doc = "Bit 4 - FIFO overrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun_mask(&mut self) -> FifoOverrunMaskW<GpMaskSpec> {
        FifoOverrunMaskW::new(self, 4)
    }
}
#[doc = "Audio GPA FIFO Full and Empty Mask Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpMaskSpec;
impl crate::RegisterSpec for GpMaskSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gp_mask::R`](R) reader structure"]
impl crate::Readable for GpMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`gp_mask::W`](W) writer structure"]
impl crate::Writable for GpMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GP_MASK to value 0x10"]
impl crate::Resettable for GpMaskSpec {
    const RESET_VALUE: u8 = 0x10;
}

#[doc = "Register `AUD_INT` reader"]
pub type R = crate::R<AudIntSpec>;
#[doc = "Register `AUD_INT` writer"]
pub type W = crate::W<AudIntSpec>;
#[doc = "Field `FIFO_FULL_MASK` reader - FIFO full mask."]
pub type FifoFullMaskR = crate::BitReader;
#[doc = "Field `FIFO_FULL_MASK` writer - FIFO full mask."]
pub type FifoFullMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_EMPTY_MASK` reader - FIFO empty mask."]
pub type FifoEmptyMaskR = crate::BitReader;
#[doc = "Field `FIFO_EMPTY_MASK` writer - FIFO empty mask."]
pub type FifoEmptyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - FIFO full mask."]
    #[inline(always)]
    pub fn fifo_full_mask(&self) -> FifoFullMaskR {
        FifoFullMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO empty mask."]
    #[inline(always)]
    pub fn fifo_empty_mask(&self) -> FifoEmptyMaskR {
        FifoEmptyMaskR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - FIFO full mask."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_full_mask(&mut self) -> FifoFullMaskW<AudIntSpec> {
        FifoFullMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO empty mask."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_empty_mask(&mut self) -> FifoEmptyMaskW<AudIntSpec> {
        FifoEmptyMaskW::new(self, 3)
    }
}
#[doc = "Reserved for future use.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudIntSpec;
impl crate::RegisterSpec for AudIntSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_int::R`](R) reader structure"]
impl crate::Readable for AudIntSpec {}
#[doc = "`write(|w| ..)` method takes [`aud_int::W`](W) writer structure"]
impl crate::Writable for AudIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_INT to value 0"]
impl crate::Resettable for AudIntSpec {
    const RESET_VALUE: u8 = 0;
}

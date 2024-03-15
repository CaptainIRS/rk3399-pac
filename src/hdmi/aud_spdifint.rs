#[doc = "Register `AUD_SPDIFINT` reader"]
pub type R = crate::R<AudSpdifintSpec>;
#[doc = "Register `AUD_SPDIFINT` writer"]
pub type W = crate::W<AudSpdifintSpec>;
#[doc = "Field `SPDIF_FIFO_FULL_MASK` reader - Description: SPDIF FIFO full mask"]
pub type SpdifFifoFullMaskR = crate::BitReader;
#[doc = "Field `SPDIF_FIFO_FULL_MASK` writer - Description: SPDIF FIFO full mask"]
pub type SpdifFifoFullMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPDIF_FIFO_EMPTY_MASK` reader - Description: SPDIF FIFO empty mask"]
pub type SpdifFifoEmptyMaskR = crate::BitReader;
#[doc = "Field `SPDIF_FIFO_EMPTY_MASK` writer - Description: SPDIF FIFO empty mask"]
pub type SpdifFifoEmptyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Description: SPDIF FIFO full mask"]
    #[inline(always)]
    pub fn spdif_fifo_full_mask(&self) -> SpdifFifoFullMaskR {
        SpdifFifoFullMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Description: SPDIF FIFO empty mask"]
    #[inline(always)]
    pub fn spdif_fifo_empty_mask(&self) -> SpdifFifoEmptyMaskR {
        SpdifFifoEmptyMaskR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Description: SPDIF FIFO full mask"]
    #[inline(always)]
    #[must_use]
    pub fn spdif_fifo_full_mask(&mut self) -> SpdifFifoFullMaskW<AudSpdifintSpec> {
        SpdifFifoFullMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Description: SPDIF FIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn spdif_fifo_empty_mask(&mut self) -> SpdifFifoEmptyMaskW<AudSpdifintSpec> {
        SpdifFifoEmptyMaskW::new(self, 3)
    }
}
#[doc = "Reserved for future use.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdifint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdifint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudSpdifintSpec;
impl crate::RegisterSpec for AudSpdifintSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_spdifint::R`](R) reader structure"]
impl crate::Readable for AudSpdifintSpec {}
#[doc = "`write(|w| ..)` method takes [`aud_spdifint::W`](W) writer structure"]
impl crate::Writable for AudSpdifintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_SPDIFINT to value 0"]
impl crate::Resettable for AudSpdifintSpec {
    const RESET_VALUE: u8 = 0;
}

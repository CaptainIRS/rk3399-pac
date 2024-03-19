#[doc = "Register `IH_AS_STAT0` reader"]
pub type R = crate::R<IhAsStat0Spec>;
#[doc = "Register `IH_AS_STAT0` writer"]
pub type W = crate::W<IhAsStat0Spec>;
#[doc = "Field `AUD_FIFO_OVERFLOW` reader - Audio Sampler audio FIFO full indication."]
pub type AudFifoOverflowR = crate::BitReader;
#[doc = "Field `AUD_FIFO_OVERFLOW` writer - Audio Sampler audio FIFO full indication."]
pub type AudFifoOverflowW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUD_FIFO_UNDERFLOW` reader - Audio Sampler audio FIFO empty indication."]
pub type AudFifoUnderflowR = crate::BitReader;
#[doc = "Field `AUD_FIFO_UNDERFLOW` writer - Audio Sampler audio FIFO empty indication."]
pub type AudFifoUnderflowW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUD_FIFO_UNDERFLOW_THR` reader - Audio Sampler audio FIFO empty threshold (four\n\nsamples) indication for the legacy HBR audio\n\ninterface.\n\nFor AHB_DMA, this bit indicates that the number of\n\nsamples in the FIFO is equal to (or less) than the\n\nnumber of active audio channels.\n\nThis bit is not relevant for I2S, SPDIF, and GPA\n\ninterfaces."]
pub type AudFifoUnderflowThrR = crate::BitReader;
#[doc = "Field `AUD_FIFO_UNDERFLOW_THR` writer - Audio Sampler audio FIFO empty threshold (four\n\nsamples) indication for the legacy HBR audio\n\ninterface.\n\nFor AHB_DMA, this bit indicates that the number of\n\nsamples in the FIFO is equal to (or less) than the\n\nnumber of active audio channels.\n\nThis bit is not relevant for I2S, SPDIF, and GPA\n\ninterfaces."]
pub type AudFifoUnderflowThrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFO_OVERRUN` reader - Indicates an overrun on the audio FIFO."]
pub type FifoOverrunR = crate::BitReader;
#[doc = "Field `FIFO_OVERRUN` writer - Indicates an overrun on the audio FIFO."]
pub type FifoOverrunW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFO_UNDERRUN` reader - Indicates an underrun on the audio FIFO"]
pub type FifoUnderrunR = crate::BitReader;
#[doc = "Field `FIFO_UNDERRUN` writer - Indicates an underrun on the audio FIFO"]
pub type FifoUnderrunW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Audio Sampler audio FIFO full indication."]
    #[inline(always)]
    pub fn aud_fifo_overflow(&self) -> AudFifoOverflowR {
        AudFifoOverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Audio Sampler audio FIFO empty indication."]
    #[inline(always)]
    pub fn aud_fifo_underflow(&self) -> AudFifoUnderflowR {
        AudFifoUnderflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Audio Sampler audio FIFO empty threshold (four\n\nsamples) indication for the legacy HBR audio\n\ninterface.\n\nFor AHB_DMA, this bit indicates that the number of\n\nsamples in the FIFO is equal to (or less) than the\n\nnumber of active audio channels.\n\nThis bit is not relevant for I2S, SPDIF, and GPA\n\ninterfaces."]
    #[inline(always)]
    pub fn aud_fifo_underflow_thr(&self) -> AudFifoUnderflowThrR {
        AudFifoUnderflowThrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates an overrun on the audio FIFO."]
    #[inline(always)]
    pub fn fifo_overrun(&self) -> FifoOverrunR {
        FifoOverrunR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates an underrun on the audio FIFO"]
    #[inline(always)]
    pub fn fifo_underrun(&self) -> FifoUnderrunR {
        FifoUnderrunR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Audio Sampler audio FIFO full indication."]
    #[inline(always)]
    #[must_use]
    pub fn aud_fifo_overflow(&mut self) -> AudFifoOverflowW<IhAsStat0Spec> {
        AudFifoOverflowW::new(self, 0)
    }
    #[doc = "Bit 1 - Audio Sampler audio FIFO empty indication."]
    #[inline(always)]
    #[must_use]
    pub fn aud_fifo_underflow(&mut self) -> AudFifoUnderflowW<IhAsStat0Spec> {
        AudFifoUnderflowW::new(self, 1)
    }
    #[doc = "Bit 2 - Audio Sampler audio FIFO empty threshold (four\n\nsamples) indication for the legacy HBR audio\n\ninterface.\n\nFor AHB_DMA, this bit indicates that the number of\n\nsamples in the FIFO is equal to (or less) than the\n\nnumber of active audio channels.\n\nThis bit is not relevant for I2S, SPDIF, and GPA\n\ninterfaces."]
    #[inline(always)]
    #[must_use]
    pub fn aud_fifo_underflow_thr(&mut self) -> AudFifoUnderflowThrW<IhAsStat0Spec> {
        AudFifoUnderflowThrW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates an overrun on the audio FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun(&mut self) -> FifoOverrunW<IhAsStat0Spec> {
        FifoOverrunW::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates an underrun on the audio FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_underrun(&mut self) -> FifoUnderrunW<IhAsStat0Spec> {
        FifoUnderrunW::new(self, 4)
    }
}
#[doc = "Audio Sampler Interrupt Status Register (FIFO Threshold, Underflow and\n\nOverflow Interrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_as_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_as_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhAsStat0Spec;
impl crate::RegisterSpec for IhAsStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_as_stat0::R`](R) reader structure"]
impl crate::Readable for IhAsStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_as_stat0::W`](W) writer structure"]
impl crate::Writable for IhAsStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x1f;
}
#[doc = "`reset()` method sets IH_AS_STAT0 to value 0"]
impl crate::Resettable for IhAsStat0Spec {
    const RESET_VALUE: u8 = 0;
}

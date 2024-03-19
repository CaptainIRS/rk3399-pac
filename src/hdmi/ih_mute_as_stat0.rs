#[doc = "Register `IH_MUTE_AS_STAT0` reader"]
pub type R = crate::R<IhMuteAsStat0Spec>;
#[doc = "Register `IH_MUTE_AS_STAT0` writer"]
pub type W = crate::W<IhMuteAsStat0Spec>;
#[doc = "Field `AUD_FIFO_OVERFLOW` reader - When set to 1, mutes ih_as_stat0\\[0\\]"]
pub type AudFifoOverflowR = crate::BitReader;
#[doc = "Field `AUD_FIFO_OVERFLOW` writer - When set to 1, mutes ih_as_stat0\\[0\\]"]
pub type AudFifoOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUD_FIFO_UNDERFLOW` reader - When set to 1, mutes ih_as_stat0\\[1\\]"]
pub type AudFifoUnderflowR = crate::BitReader;
#[doc = "Field `AUD_FIFO_UNDERFLOW` writer - When set to 1, mutes ih_as_stat0\\[1\\]"]
pub type AudFifoUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUD_FIFO_UNDERFLOW_THR` reader - When set to 1, mutes ih_as_stat0\\[2\\]"]
pub type AudFifoUnderflowThrR = crate::BitReader;
#[doc = "Field `AUD_FIFO_UNDERFLOW_THR` writer - When set to 1, mutes ih_as_stat0\\[2\\]"]
pub type AudFifoUnderflowThrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_OVERRUN` reader - When set to 1, mutes ih_as_stat0\\[3\\]"]
pub type FifoOverrunR = crate::BitReader;
#[doc = "Field `FIFO_OVERRUN` writer - When set to 1, mutes ih_as_stat0\\[3\\]"]
pub type FifoOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_UNDERRUN` reader - When set to 1, mutes ih_as_stat0\\[4\\]"]
pub type FifoUnderrunR = crate::BitReader;
#[doc = "Field `FIFO_UNDERRUN` writer - When set to 1, mutes ih_as_stat0\\[4\\]"]
pub type FifoUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, mutes ih_as_stat0\\[0\\]"]
    #[inline(always)]
    pub fn aud_fifo_overflow(&self) -> AudFifoOverflowR {
        AudFifoOverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_as_stat0\\[1\\]"]
    #[inline(always)]
    pub fn aud_fifo_underflow(&self) -> AudFifoUnderflowR {
        AudFifoUnderflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_as_stat0\\[2\\]"]
    #[inline(always)]
    pub fn aud_fifo_underflow_thr(&self) -> AudFifoUnderflowThrR {
        AudFifoUnderflowThrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_as_stat0\\[3\\]"]
    #[inline(always)]
    pub fn fifo_overrun(&self) -> FifoOverrunR {
        FifoOverrunR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_as_stat0\\[4\\]"]
    #[inline(always)]
    pub fn fifo_underrun(&self) -> FifoUnderrunR {
        FifoUnderrunR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, mutes ih_as_stat0\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aud_fifo_overflow(&mut self) -> AudFifoOverflowW<IhMuteAsStat0Spec> {
        AudFifoOverflowW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_as_stat0\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aud_fifo_underflow(&mut self) -> AudFifoUnderflowW<IhMuteAsStat0Spec> {
        AudFifoUnderflowW::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_as_stat0\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aud_fifo_underflow_thr(&mut self) -> AudFifoUnderflowThrW<IhMuteAsStat0Spec> {
        AudFifoUnderflowThrW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_as_stat0\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun(&mut self) -> FifoOverrunW<IhMuteAsStat0Spec> {
        FifoOverrunW::new(self, 3)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_as_stat0\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_underrun(&mut self) -> FifoUnderrunW<IhMuteAsStat0Spec> {
        FifoUnderrunW::new(self, 4)
    }
}
#[doc = "Audio Sampler Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_as_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_as_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMuteAsStat0Spec;
impl crate::RegisterSpec for IhMuteAsStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_as_stat0::R`](R) reader structure"]
impl crate::Readable for IhMuteAsStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_as_stat0::W`](W) writer structure"]
impl crate::Writable for IhMuteAsStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_AS_STAT0 to value 0x18"]
impl crate::Resettable for IhMuteAsStat0Spec {
    const RESET_VALUE: u8 = 0x18;
}

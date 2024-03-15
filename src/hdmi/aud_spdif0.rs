#[doc = "Register `AUD_SPDIF0` reader"]
pub type R = crate::R<AudSpdif0Spec>;
#[doc = "Register `AUD_SPDIF0` writer"]
pub type W = crate::W<AudSpdif0Spec>;
#[doc = "Field `SPARE` reader - Reserved as \"spare\" bit with no associated functionality."]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - Reserved as \"spare\" bit with no associated functionality."]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SW_AUDIO_FIFO_RST` reader - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via register write command) lands in the middle of an SPDIF audio transaction, the samples become misaligned (left- right sequence lost). As a solution, for each FIFO reset, an associated SPDIF reset must be issued (writing 8'hEF to MC_SWRSTZ register)."]
pub type SwAudioFifoRstR = crate::BitReader;
#[doc = "Field `SW_AUDIO_FIFO_RST` writer - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via register write command) lands in the middle of an SPDIF audio transaction, the samples become misaligned (left- right sequence lost). As a solution, for each FIFO reset, an associated SPDIF reset must be issued (writing 8'hEF to MC_SWRSTZ register)."]
pub type SwAudioFifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via register write command) lands in the middle of an SPDIF audio transaction, the samples become misaligned (left- right sequence lost). As a solution, for each FIFO reset, an associated SPDIF reset must be issued (writing 8'hEF to MC_SWRSTZ register)."]
    #[inline(always)]
    pub fn sw_audio_fifo_rst(&self) -> SwAudioFifoRstR {
        SwAudioFifoRstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<AudSpdif0Spec> {
        SpareW::new(self, 0)
    }
    #[doc = "Bit 7 - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via register write command) lands in the middle of an SPDIF audio transaction, the samples become misaligned (left- right sequence lost). As a solution, for each FIFO reset, an associated SPDIF reset must be issued (writing 8'hEF to MC_SWRSTZ register)."]
    #[inline(always)]
    #[must_use]
    pub fn sw_audio_fifo_rst(&mut self) -> SwAudioFifoRstW<AudSpdif0Spec> {
        SwAudioFifoRstW::new(self, 7)
    }
}
#[doc = "Reserved as \"spare\" bit with no associated functionality.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdif0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdif0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudSpdif0Spec;
impl crate::RegisterSpec for AudSpdif0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_spdif0::R`](R) reader structure"]
impl crate::Readable for AudSpdif0Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_spdif0::W`](W) writer structure"]
impl crate::Writable for AudSpdif0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_SPDIF0 to value 0x0f"]
impl crate::Resettable for AudSpdif0Spec {
    const RESET_VALUE: u8 = 0x0f;
}

#[doc = "Register `GP_CONF0` reader"]
pub type R = crate::R<GpConf0Spec>;
#[doc = "Register `GP_CONF0` writer"]
pub type W = crate::W<GpConf0Spec>;
#[doc = "Field `SW_AUDIO_FIFO_RST` reader - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via register write command) lands in the middle of an GPAUD audio transaction, the samples become misaligned (left-right sequence lost). As a solution, for each FIFO reset, an associated SPDIF reset must be issued (writing 8'h7F to MC_SWRSTZ register)."]
pub type SwAudioFifoRstR = crate::BitReader;
#[doc = "Field `SW_AUDIO_FIFO_RST` writer - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via register write command) lands in the middle of an GPAUD audio transaction, the samples become misaligned (left-right sequence lost). As a solution, for each FIFO reset, an associated SPDIF reset must be issued (writing 8'h7F to MC_SWRSTZ register)."]
pub type SwAudioFifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via register write command) lands in the middle of an GPAUD audio transaction, the samples become misaligned (left-right sequence lost). As a solution, for each FIFO reset, an associated SPDIF reset must be issued (writing 8'h7F to MC_SWRSTZ register)."]
    #[inline(always)]
    pub fn sw_audio_fifo_rst(&self) -> SwAudioFifoRstR {
        SwAudioFifoRstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via register write command) lands in the middle of an GPAUD audio transaction, the samples become misaligned (left-right sequence lost). As a solution, for each FIFO reset, an associated SPDIF reset must be issued (writing 8'h7F to MC_SWRSTZ register)."]
    #[inline(always)]
    #[must_use]
    pub fn sw_audio_fifo_rst(&mut self) -> SwAudioFifoRstW<GpConf0Spec> {
        SwAudioFifoRstW::new(self, 0)
    }
}
#[doc = "Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via register write command) lands in the middle of an GPAUD audio transaction, the samples become misaligned (left-right sequence lost). As a solution, for each FIFO reset, an associated SPDIF reset must be issued (writing 8'h7F to MC_SWRSTZ register).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpConf0Spec;
impl crate::RegisterSpec for GpConf0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gp_conf0::R`](R) reader structure"]
impl crate::Readable for GpConf0Spec {}
#[doc = "`write(|w| ..)` method takes [`gp_conf0::W`](W) writer structure"]
impl crate::Writable for GpConf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GP_CONF0 to value 0"]
impl crate::Resettable for GpConf0Spec {
    const RESET_VALUE: u8 = 0;
}

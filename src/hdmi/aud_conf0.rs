#[doc = "Register `AUD_CONF0` reader"]
pub type R = crate::R<AudConf0Spec>;
#[doc = "Register `AUD_CONF0` writer"]
pub type W = crate::W<AudConf0Spec>;
#[doc = "Field `I2S_IN_EN` reader - Action I2S_in_en\\[0\\]
- I2Sdata\\[0\\]
enable I2S_in_en\\[1\\]
- I2Sdata\\[1\\]
enable I2S_in_en\\[2\\]
- I2Sdata\\[2\\]
enable I2S_in_en\\[3\\]
- I2Sdata\\[3\\]
enable"]
pub type I2sInEnR = crate::FieldReader;
#[doc = "Field `I2S_IN_EN` writer - Action I2S_in_en\\[0\\]
- I2Sdata\\[0\\]
enable I2S_in_en\\[1\\]
- I2Sdata\\[1\\]
enable I2S_in_en\\[2\\]
- I2Sdata\\[2\\]
enable I2S_in_en\\[3\\]
- I2Sdata\\[3\\]
enable"]
pub type I2sInEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPARE_1` reader - Reserved as \"spare\" bit with no associated functionality."]
pub type Spare1R = crate::BitReader;
#[doc = "Field `SPARE_1` writer - Reserved as \"spare\" bit with no associated functionality."]
pub type Spare1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_SELECT` reader - 1b: Selects I2S Audio Interface 0b: Selects the second (SPDIF/GPA) interface, in configurations with more that one audio interface (DOUBLE/GDOUBLE)"]
pub type I2sSelectR = crate::BitReader;
#[doc = "Field `I2S_SELECT` writer - 1b: Selects I2S Audio Interface 0b: Selects the second (SPDIF/GPA) interface, in configurations with more that one audio interface (DOUBLE/GDOUBLE)"]
pub type I2sSelectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE_2` reader - Reserved as \"spare\" bit with no associated functionality."]
pub type Spare2R = crate::BitReader;
#[doc = "Field `SPARE_2` writer - Reserved as \"spare\" bit with no associated functionality."]
pub type Spare2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_AUDIO_FIFO_RST` reader - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via SFR command) lands in the middle of an I2S transaction, the samples become Bits Name Attr Description misaligned (left-right sequence lost). As a solution, for each FIFO reset, an associated I2S reset must be issued (writing 8'hF7 to MC_SWRSTZ register)."]
pub type SwAudioFifoRstR = crate::BitReader;
#[doc = "Field `SW_AUDIO_FIFO_RST` writer - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via SFR command) lands in the middle of an I2S transaction, the samples become Bits Name Attr Description misaligned (left-right sequence lost). As a solution, for each FIFO reset, an associated I2S reset must be issued (writing 8'hF7 to MC_SWRSTZ register)."]
pub type SwAudioFifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Action I2S_in_en\\[0\\]
- I2Sdata\\[0\\]
enable I2S_in_en\\[1\\]
- I2Sdata\\[1\\]
enable I2S_in_en\\[2\\]
- I2Sdata\\[2\\]
enable I2S_in_en\\[3\\]
- I2Sdata\\[3\\]
enable"]
    #[inline(always)]
    pub fn i2s_in_en(&self) -> I2sInEnR {
        I2sInEnR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn spare_1(&self) -> Spare1R {
        Spare1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1b: Selects I2S Audio Interface 0b: Selects the second (SPDIF/GPA) interface, in configurations with more that one audio interface (DOUBLE/GDOUBLE)"]
    #[inline(always)]
    pub fn i2s_select(&self) -> I2sSelectR {
        I2sSelectR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn spare_2(&self) -> Spare2R {
        Spare2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via SFR command) lands in the middle of an I2S transaction, the samples become Bits Name Attr Description misaligned (left-right sequence lost). As a solution, for each FIFO reset, an associated I2S reset must be issued (writing 8'hF7 to MC_SWRSTZ register)."]
    #[inline(always)]
    pub fn sw_audio_fifo_rst(&self) -> SwAudioFifoRstR {
        SwAudioFifoRstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Action I2S_in_en\\[0\\]
- I2Sdata\\[0\\]
enable I2S_in_en\\[1\\]
- I2Sdata\\[1\\]
enable I2S_in_en\\[2\\]
- I2Sdata\\[2\\]
enable I2S_in_en\\[3\\]
- I2Sdata\\[3\\]
enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_in_en(&mut self) -> I2sInEnW<AudConf0Spec> {
        I2sInEnW::new(self, 0)
    }
    #[doc = "Bit 4 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_1(&mut self) -> Spare1W<AudConf0Spec> {
        Spare1W::new(self, 4)
    }
    #[doc = "Bit 5 - 1b: Selects I2S Audio Interface 0b: Selects the second (SPDIF/GPA) interface, in configurations with more that one audio interface (DOUBLE/GDOUBLE)"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_select(&mut self) -> I2sSelectW<AudConf0Spec> {
        I2sSelectW::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_2(&mut self) -> Spare2W<AudConf0Spec> {
        Spare2W::new(self, 6)
    }
    #[doc = "Bit 7 - Audio FIFOs software reset Writing 0b: no action taken Writing 1b: Resets all audio FIFOs Reading from this register always returns 0b. Note: If a FIFO reset request (via SFR command) lands in the middle of an I2S transaction, the samples become Bits Name Attr Description misaligned (left-right sequence lost). As a solution, for each FIFO reset, an associated I2S reset must be issued (writing 8'hF7 to MC_SWRSTZ register)."]
    #[inline(always)]
    #[must_use]
    pub fn sw_audio_fifo_rst(&mut self) -> SwAudioFifoRstW<AudConf0Spec> {
        SwAudioFifoRstW::new(self, 7)
    }
}
#[doc = "Action I2S_in_en\\[0\\]
- I2Sdata\\[0\\]
enable I2S_in_en\\[1\\]
- I2Sdata\\[1\\]
enable I2S_in_en\\[2\\]
- I2Sdata\\[2\\]
enable I2S_in_en\\[3\\]
- I2Sdata\\[3\\]
enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudConf0Spec;
impl crate::RegisterSpec for AudConf0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_conf0::R`](R) reader structure"]
impl crate::Readable for AudConf0Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_conf0::W`](W) writer structure"]
impl crate::Writable for AudConf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_CONF0 to value 0x2f"]
impl crate::Resettable for AudConf0Spec {
    const RESET_VALUE: u8 = 0x2f;
}

#[doc = "Register `AHB_DMA_CONF0` reader"]
pub type R = crate::R<AhbDmaConf0Spec>;
#[doc = "Register `AHB_DMA_CONF0` writer"]
pub type W = crate::W<AhbDmaConf0Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BurstMode {
    #[doc = "1: Forces the burst mode to be fixed beat, incremental burst mode designated by the incr_type\\[1:0\\]
signal."]
    B1 = 1,
    #[doc = "0: Normal operation is unspecified length, incremental burst. It corresponds to INCR AHB burst mode."]
    B0 = 0,
}
impl From<BurstMode> for bool {
    #[inline(always)]
    fn from(variant: BurstMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST_MODE` reader - "]
pub type BurstModeR = crate::BitReader<BurstMode>;
impl BurstModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BurstMode {
        match self.bits {
            true => BurstMode::B1,
            false => BurstMode::B0,
        }
    }
    #[doc = "Forces the burst mode to be fixed beat, incremental burst mode designated by the incr_type\\[1:0\\]
signal."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BurstMode::B1
    }
    #[doc = "Normal operation is unspecified length, incremental burst. It corresponds to INCR AHB burst mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BurstMode::B0
    }
}
#[doc = "Field `BURST_MODE` writer - "]
pub type BurstModeW<'a, REG> = crate::BitWriter<'a, REG, BurstMode>;
impl<'a, REG> BurstModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Forces the burst mode to be fixed beat, incremental burst mode designated by the incr_type\\[1:0\\]
signal."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BurstMode::B1)
    }
    #[doc = "Normal operation is unspecified length, incremental burst. It corresponds to INCR AHB burst mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BurstMode::B0)
    }
}
#[doc = "Selects the preferred burst length size\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IncrType {
    #[doc = "0: Corresponds to INCR4 fixed four beat, incremental AHB burst mode. Only valid when burst_mode is high."]
    B00 = 0,
    #[doc = "1: Corresponds to INCR8 fixed eight beat incremental AHB burst mode. Only valid when burst_mode is high."]
    B01 = 1,
    #[doc = "2: Corresponds to INCR16 fixed 16 beat incremental AHB burst mode. Only valid when burst_mode is high."]
    B10 = 2,
    #[doc = "3: Corresponds to INCR16 fixed 16 beat incremental AHB burst mode. Only valid when burst_mode is high."]
    B11 = 3,
}
impl From<IncrType> for u8 {
    #[inline(always)]
    fn from(variant: IncrType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IncrType {
    type Ux = u8;
}
#[doc = "Field `INCR_TYPE` reader - Selects the preferred burst length size"]
pub type IncrTypeR = crate::FieldReader<IncrType>;
impl IncrTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IncrType {
        match self.bits {
            0 => IncrType::B00,
            1 => IncrType::B01,
            2 => IncrType::B10,
            3 => IncrType::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Corresponds to INCR4 fixed four beat, incremental AHB burst mode. Only valid when burst_mode is high."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == IncrType::B00
    }
    #[doc = "Corresponds to INCR8 fixed eight beat incremental AHB burst mode. Only valid when burst_mode is high."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == IncrType::B01
    }
    #[doc = "Corresponds to INCR16 fixed 16 beat incremental AHB burst mode. Only valid when burst_mode is high."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == IncrType::B10
    }
    #[doc = "Corresponds to INCR16 fixed 16 beat incremental AHB burst mode. Only valid when burst_mode is high."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == IncrType::B11
    }
}
#[doc = "Field `INCR_TYPE` writer - Selects the preferred burst length size"]
pub type IncrTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, IncrType>;
impl<'a, REG> IncrTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Corresponds to INCR4 fixed four beat, incremental AHB burst mode. Only valid when burst_mode is high."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(IncrType::B00)
    }
    #[doc = "Corresponds to INCR8 fixed eight beat incremental AHB burst mode. Only valid when burst_mode is high."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(IncrType::B01)
    }
    #[doc = "Corresponds to INCR16 fixed 16 beat incremental AHB burst mode. Only valid when burst_mode is high."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(IncrType::B10)
    }
    #[doc = "Corresponds to INCR16 fixed 16 beat incremental AHB burst mode. Only valid when burst_mode is high."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(IncrType::B11)
    }
}
#[doc = "Enable request of locked burst AHB mechanism.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnableHlock {
    #[doc = "1: Enables the usage of hlock for master request to arbiter of a locked complete burst."]
    B1 = 1,
    #[doc = "0: Disables request of locked burst AHB mechanism"]
    B0 = 0,
}
impl From<EnableHlock> for bool {
    #[inline(always)]
    fn from(variant: EnableHlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE_HLOCK` reader - Enable request of locked burst AHB mechanism."]
pub type EnableHlockR = crate::BitReader<EnableHlock>;
impl EnableHlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnableHlock {
        match self.bits {
            true => EnableHlock::B1,
            false => EnableHlock::B0,
        }
    }
    #[doc = "Enables the usage of hlock for master request to arbiter of a locked complete burst."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EnableHlock::B1
    }
    #[doc = "Disables request of locked burst AHB mechanism"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EnableHlock::B0
    }
}
#[doc = "Field `ENABLE_HLOCK` writer - Enable request of locked burst AHB mechanism."]
pub type EnableHlockW<'a, REG> = crate::BitWriter<'a, REG, EnableHlock>;
impl<'a, REG> EnableHlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enables the usage of hlock for master request to arbiter of a locked complete burst."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EnableHlock::B1)
    }
    #[doc = "Disables request of locked burst AHB mechanism"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EnableHlock::B0)
    }
}
#[doc = "Field `HBR` reader - HBR packet enable\n\nThe Hdmi_tx sends the HBR packets. This bit must be\n\nenabled when transmitting non-linear audio of frequency\n\nhigher than 192 kHz. If this bit is enabled, the number of\n\nchannels configured in AHB_DMA_CONF1 is always 8."]
pub type HbrR = crate::BitReader;
#[doc = "Field `HBR` writer - HBR packet enable\n\nThe Hdmi_tx sends the HBR packets. This bit must be\n\nenabled when transmitting non-linear audio of frequency\n\nhigher than 192 kHz. If this bit is enabled, the number of\n\nchannels configured in AHB_DMA_CONF1 is always 8."]
pub type HbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSERT_PCUV` reader - Enables the insertion of PCUV data"]
pub type InsertPcuvR = crate::BitReader;
#[doc = "Field `INSERT_PCUV` writer - Enables the insertion of PCUV data"]
pub type InsertPcuvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_FIFO_RST` reader - This is the software reset bit for the audio and FIFO\n\nclear.\n\nWriting 0'b does not result in any action.\n\nWriting 1'b to this register resets all audio FIFOs.\n\nReading from this register always returns 0'b."]
pub type SwFifoRstR = crate::BitReader;
#[doc = "Field `SW_FIFO_RST` writer - This is the software reset bit for the audio and FIFO\n\nclear.\n\nWriting 0'b does not result in any action.\n\nWriting 1'b to this register resets all audio FIFOs.\n\nReading from this register always returns 0'b."]
pub type SwFifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn burst_mode(&self) -> BurstModeR {
        BurstModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Selects the preferred burst length size"]
    #[inline(always)]
    pub fn incr_type(&self) -> IncrTypeR {
        IncrTypeR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Enable request of locked burst AHB mechanism."]
    #[inline(always)]
    pub fn enable_hlock(&self) -> EnableHlockR {
        EnableHlockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HBR packet enable\n\nThe Hdmi_tx sends the HBR packets. This bit must be\n\nenabled when transmitting non-linear audio of frequency\n\nhigher than 192 kHz. If this bit is enabled, the number of\n\nchannels configured in AHB_DMA_CONF1 is always 8."]
    #[inline(always)]
    pub fn hbr(&self) -> HbrR {
        HbrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables the insertion of PCUV data"]
    #[inline(always)]
    pub fn insert_pcuv(&self) -> InsertPcuvR {
        InsertPcuvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the software reset bit for the audio and FIFO\n\nclear.\n\nWriting 0'b does not result in any action.\n\nWriting 1'b to this register resets all audio FIFOs.\n\nReading from this register always returns 0'b."]
    #[inline(always)]
    pub fn sw_fifo_rst(&self) -> SwFifoRstR {
        SwFifoRstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn burst_mode(&mut self) -> BurstModeW<AhbDmaConf0Spec> {
        BurstModeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Selects the preferred burst length size"]
    #[inline(always)]
    #[must_use]
    pub fn incr_type(&mut self) -> IncrTypeW<AhbDmaConf0Spec> {
        IncrTypeW::new(self, 1)
    }
    #[doc = "Bit 3 - Enable request of locked burst AHB mechanism."]
    #[inline(always)]
    #[must_use]
    pub fn enable_hlock(&mut self) -> EnableHlockW<AhbDmaConf0Spec> {
        EnableHlockW::new(self, 3)
    }
    #[doc = "Bit 4 - HBR packet enable\n\nThe Hdmi_tx sends the HBR packets. This bit must be\n\nenabled when transmitting non-linear audio of frequency\n\nhigher than 192 kHz. If this bit is enabled, the number of\n\nchannels configured in AHB_DMA_CONF1 is always 8."]
    #[inline(always)]
    #[must_use]
    pub fn hbr(&mut self) -> HbrW<AhbDmaConf0Spec> {
        HbrW::new(self, 4)
    }
    #[doc = "Bit 6 - Enables the insertion of PCUV data"]
    #[inline(always)]
    #[must_use]
    pub fn insert_pcuv(&mut self) -> InsertPcuvW<AhbDmaConf0Spec> {
        InsertPcuvW::new(self, 6)
    }
    #[doc = "Bit 7 - This is the software reset bit for the audio and FIFO\n\nclear.\n\nWriting 0'b does not result in any action.\n\nWriting 1'b to this register resets all audio FIFOs.\n\nReading from this register always returns 0'b."]
    #[inline(always)]
    #[must_use]
    pub fn sw_fifo_rst(&mut self) -> SwFifoRstW<AhbDmaConf0Spec> {
        SwFifoRstW::new(self, 7)
    }
}
#[doc = "Audio DMA SW FIFO reset and DMA Configuration Register 0\n\nThis register contains the software reset bit for the audio FIFOs. It also configures\n\noperating modes of the AHB master.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaConf0Spec;
impl crate::RegisterSpec for AhbDmaConf0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_conf0::R`](R) reader structure"]
impl crate::Readable for AhbDmaConf0Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_conf0::W`](W) writer structure"]
impl crate::Writable for AhbDmaConf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_CONF0 to value 0"]
impl crate::Resettable for AhbDmaConf0Spec {
    const RESET_VALUE: u8 = 0;
}

#[doc = "Register `CRU_CLKGATE_CON34` reader"]
pub type R = crate::R<CruClkgateCon34Spec>;
#[doc = "Register `CRU_CLKGATE_CON34` writer"]
pub type W = crate::W<CruClkgateCon34Spec>;
#[doc = "Field `HCLK_I2S0_EN` reader - hclk_i2s0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkI2s0EnR = crate::BitReader;
#[doc = "Field `HCLK_I2S0_EN` writer - hclk_i2s0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkI2s0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_I2S1_EN` reader - hclk_i2s1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkI2s1EnR = crate::BitReader;
#[doc = "Field `HCLK_I2S1_EN` writer - hclk_i2s1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkI2s1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_I2S2_EN` reader - hclk_i2s2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkI2s2EnR = crate::BitReader;
#[doc = "Field `HCLK_I2S2_EN` writer - hclk_i2s2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkI2s2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_SPDIF_EN` reader - hclk_spdif clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSpdifEnR = crate::BitReader;
#[doc = "Field `HCLK_SPDIF_EN` writer - hclk_spdif clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSpdifEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_SDIO_EN` reader - hclk_sdio clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSdioEnR = crate::BitReader;
#[doc = "Field `HCLK_SDIO_EN` writer - hclk_sdio clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSdioEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_SPI5_EN` reader - pclk_spi5 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi5EnR = crate::BitReader;
#[doc = "Field `PCLK_SPI5_EN` writer - pclk_spi5 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_SDIOAUDIO_NOC_EN` reader - hclk_sdioaudio_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkSdioaudioNocEnR = crate::BitReader;
#[doc = "Field `HCLK_SDIOAUDIO_NOC_EN` writer - hclk_sdioaudio_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkSdioaudioNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - hclk_i2s0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_i2s0_en(&self) -> HclkI2s0EnR {
        HclkI2s0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - hclk_i2s1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_i2s1_en(&self) -> HclkI2s1EnR {
        HclkI2s1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hclk_i2s2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_i2s2_en(&self) -> HclkI2s2EnR {
        HclkI2s2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hclk_spdif clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_spdif_en(&self) -> HclkSpdifEnR {
        HclkSpdifEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hclk_sdio clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_sdio_en(&self) -> HclkSdioEnR {
        HclkSdioEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pclk_spi5 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_spi5_en(&self) -> PclkSpi5EnR {
        PclkSpi5EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - hclk_sdioaudio_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn hclk_sdioaudio_noc_en(&self) -> HclkSdioaudioNocEnR {
        HclkSdioaudioNocEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - hclk_i2s0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_i2s0_en(&mut self) -> HclkI2s0EnW<CruClkgateCon34Spec> {
        HclkI2s0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - hclk_i2s1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_i2s1_en(&mut self) -> HclkI2s1EnW<CruClkgateCon34Spec> {
        HclkI2s1EnW::new(self, 1)
    }
    #[doc = "Bit 2 - hclk_i2s2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_i2s2_en(&mut self) -> HclkI2s2EnW<CruClkgateCon34Spec> {
        HclkI2s2EnW::new(self, 2)
    }
    #[doc = "Bit 3 - hclk_spdif clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_spdif_en(&mut self) -> HclkSpdifEnW<CruClkgateCon34Spec> {
        HclkSpdifEnW::new(self, 3)
    }
    #[doc = "Bit 4 - hclk_sdio clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_sdio_en(&mut self) -> HclkSdioEnW<CruClkgateCon34Spec> {
        HclkSdioEnW::new(self, 4)
    }
    #[doc = "Bit 5 - pclk_spi5 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi5_en(&mut self) -> PclkSpi5EnW<CruClkgateCon34Spec> {
        PclkSpi5EnW::new(self, 5)
    }
    #[doc = "Bit 6 - hclk_sdioaudio_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_sdioaudio_noc_en(&mut self) -> HclkSdioaudioNocEnW<CruClkgateCon34Spec> {
        HclkSdioaudioNocEnW::new(self, 6)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon34Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon34Spec;
impl crate::RegisterSpec for CruClkgateCon34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con34::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon34Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con34::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON34 to value 0"]
impl crate::Resettable for CruClkgateCon34Spec {
    const RESET_VALUE: u32 = 0;
}

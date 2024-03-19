#[doc = "Register `CRU_CLKGATE_CON23` reader"]
pub type R = crate::R<CruClkgateCon23Spec>;
#[doc = "Register `CRU_CLKGATE_CON23` writer"]
pub type W = crate::W<CruClkgateCon23Spec>;
#[doc = "Field `ACLK_INTMEM_EN` reader - aclk_intmem clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkIntmemEnR = crate::BitReader;
#[doc = "Field `ACLK_INTMEM_EN` writer - aclk_intmem clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkIntmemEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_TZMA_EN` reader - aclk_tzma clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkTzmaEnR = crate::BitReader;
#[doc = "Field `ACLK_TZMA_EN` writer - aclk_tzma clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkTzmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_INTMEM0_EN` reader - clk_intmem0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem0EnR = crate::BitReader;
#[doc = "Field `CLK_INTMEM0_EN` writer - clk_intmem0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_INTMEM1_EN` reader - clk_intmem1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem1EnR = crate::BitReader;
#[doc = "Field `CLK_INTMEM1_EN` writer - clk_intmem1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_INTMEM2_EN` reader - clk_intmem2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem2EnR = crate::BitReader;
#[doc = "Field `CLK_INTMEM2_EN` writer - clk_intmem2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_INTMEM3_EN` reader - clk_intmem3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem3EnR = crate::BitReader;
#[doc = "Field `CLK_INTMEM3_EN` writer - clk_intmem3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_INTMEM4_EN` reader - clk_intmem4 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem4EnR = crate::BitReader;
#[doc = "Field `CLK_INTMEM4_EN` writer - clk_intmem4 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_INTMEM5_EN` reader - clk_intmem5 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem5EnR = crate::BitReader;
#[doc = "Field `CLK_INTMEM5_EN` writer - clk_intmem5 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIntmem5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_DCF_EN` reader - aclk_dcf clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkDcfEnR = crate::BitReader;
#[doc = "Field `ACLK_DCF_EN` writer - aclk_dcf clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkDcfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_DCF_EN` reader - pclk_dcf clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkDcfEnR = crate::BitReader;
#[doc = "Field `PCLK_DCF_EN` writer - pclk_dcf clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkDcfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_SPI0CODEC_EN` reader - pclk_spi0codec clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi0codecEnR = crate::BitReader;
#[doc = "Field `PCLK_SPI0CODEC_EN` writer - pclk_spi0codec clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi0codecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_SPI1_EN` reader - pclk_spi1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi1EnR = crate::BitReader;
#[doc = "Field `PCLK_SPI1_EN` writer - pclk_spi1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_SPI2_EN` reader - pclk_spi2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi2EnR = crate::BitReader;
#[doc = "Field `PCLK_SPI2_EN` writer - pclk_spi2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_SPI4_EN` reader - pclk_spi4 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi4EnR = crate::BitReader;
#[doc = "Field `PCLK_SPI4_EN` writer - pclk_spi4 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSpi4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_intmem clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_intmem_en(&self) -> AclkIntmemEnR {
        AclkIntmemEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_tzma clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_tzma_en(&self) -> AclkTzmaEnR {
        AclkTzmaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_intmem0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_intmem0_en(&self) -> ClkIntmem0EnR {
        ClkIntmem0EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_intmem1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_intmem1_en(&self) -> ClkIntmem1EnR {
        ClkIntmem1EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_intmem2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_intmem2_en(&self) -> ClkIntmem2EnR {
        ClkIntmem2EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_intmem3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_intmem3_en(&self) -> ClkIntmem3EnR {
        ClkIntmem3EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_intmem4 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_intmem4_en(&self) -> ClkIntmem4EnR {
        ClkIntmem4EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_intmem5 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_intmem5_en(&self) -> ClkIntmem5EnR {
        ClkIntmem5EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_dcf clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_dcf_en(&self) -> AclkDcfEnR {
        AclkDcfEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pclk_dcf clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_dcf_en(&self) -> PclkDcfEnR {
        PclkDcfEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pclk_spi0codec clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_spi0codec_en(&self) -> PclkSpi0codecEnR {
        PclkSpi0codecEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pclk_spi1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_spi1_en(&self) -> PclkSpi1EnR {
        PclkSpi1EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pclk_spi2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_spi2_en(&self) -> PclkSpi2EnR {
        PclkSpi2EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pclk_spi4 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_spi4_en(&self) -> PclkSpi4EnR {
        PclkSpi4EnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_intmem clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_intmem_en(&mut self) -> AclkIntmemEnW<CruClkgateCon23Spec> {
        AclkIntmemEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_tzma clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_tzma_en(&mut self) -> AclkTzmaEnW<CruClkgateCon23Spec> {
        AclkTzmaEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_intmem0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_intmem0_en(&mut self) -> ClkIntmem0EnW<CruClkgateCon23Spec> {
        ClkIntmem0EnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_intmem1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_intmem1_en(&mut self) -> ClkIntmem1EnW<CruClkgateCon23Spec> {
        ClkIntmem1EnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_intmem2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_intmem2_en(&mut self) -> ClkIntmem2EnW<CruClkgateCon23Spec> {
        ClkIntmem2EnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_intmem3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_intmem3_en(&mut self) -> ClkIntmem3EnW<CruClkgateCon23Spec> {
        ClkIntmem3EnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_intmem4 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_intmem4_en(&mut self) -> ClkIntmem4EnW<CruClkgateCon23Spec> {
        ClkIntmem4EnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_intmem5 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_intmem5_en(&mut self) -> ClkIntmem5EnW<CruClkgateCon23Spec> {
        ClkIntmem5EnW::new(self, 7)
    }
    #[doc = "Bit 8 - aclk_dcf clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_dcf_en(&mut self) -> AclkDcfEnW<CruClkgateCon23Spec> {
        AclkDcfEnW::new(self, 8)
    }
    #[doc = "Bit 9 - pclk_dcf clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dcf_en(&mut self) -> PclkDcfEnW<CruClkgateCon23Spec> {
        PclkDcfEnW::new(self, 9)
    }
    #[doc = "Bit 10 - pclk_spi0codec clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi0codec_en(&mut self) -> PclkSpi0codecEnW<CruClkgateCon23Spec> {
        PclkSpi0codecEnW::new(self, 10)
    }
    #[doc = "Bit 11 - pclk_spi1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi1_en(&mut self) -> PclkSpi1EnW<CruClkgateCon23Spec> {
        PclkSpi1EnW::new(self, 11)
    }
    #[doc = "Bit 12 - pclk_spi2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi2_en(&mut self) -> PclkSpi2EnW<CruClkgateCon23Spec> {
        PclkSpi2EnW::new(self, 12)
    }
    #[doc = "Bit 13 - pclk_spi4 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi4_en(&mut self) -> PclkSpi4EnW<CruClkgateCon23Spec> {
        PclkSpi4EnW::new(self, 13)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon23Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon23Spec;
impl crate::RegisterSpec for CruClkgateCon23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con23::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon23Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con23::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON23 to value 0"]
impl crate::Resettable for CruClkgateCon23Spec {
    const RESET_VALUE: u32 = 0;
}

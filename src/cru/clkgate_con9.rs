#[doc = "Register `CLKGATE_CON9` reader"]
pub type R = crate::R<ClkgateCon9Spec>;
#[doc = "Register `CLKGATE_CON9` writer"]
pub type W = crate::W<ClkgateCon9Spec>;
#[doc = "Field `CLK_UART0_SRC_EN` reader - clk_uart0_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart0SrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART0_SRC_EN` writer - clk_uart0_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart0SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UART0_FRAC_SRC_EN` reader - clk_uart0_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart0FracSrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART0_FRAC_SRC_EN` writer - clk_uart0_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart0FracSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UART1_SRC_EN` reader - clk_uart1_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart1SrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART1_SRC_EN` writer - clk_uart1_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart1SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UART1_FRAC_SRC_EN` reader - clk_uart1_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart1FracSrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART1_FRAC_SRC_EN` writer - clk_uart1_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart1FracSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UART2_SRC_EN` reader - clk_uart2_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart2SrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART2_SRC_EN` writer - clk_uart2_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart2SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UART2_FRAC_SRC_EN` reader - clk_uart2_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart2FracSrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART2_FRAC_SRC_EN` writer - clk_uart2_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart2FracSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UART3_SRC_EN` reader - clk_uart3_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart3SrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART3_SRC_EN` writer - clk_uart3_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart3SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UART3_FRAC_SRC_EN` reader - clk_uart3_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart3FracSrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART3_FRAC_SRC_EN` writer - clk_uart3_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUart3FracSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TSADC_SRC_EN` reader - clk_tsadc_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTsadcSrcEnR = crate::BitReader;
#[doc = "Field `CLK_TSADC_SRC_EN` writer - clk_tsadc_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTsadcSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SARADC_SRC_EN` reader - clk_saradc_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSaradcSrcEnR = crate::BitReader;
#[doc = "Field `CLK_SARADC_SRC_EN` writer - clk_saradc_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSaradcSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SPI0_SRC_EN` reader - clk_spi0_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi0SrcEnR = crate::BitReader;
#[doc = "Field `CLK_SPI0_SRC_EN` writer - clk_spi0_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi0SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SPI1_SRC_EN` reader - clk_spi1_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi1SrcEnR = crate::BitReader;
#[doc = "Field `CLK_SPI1_SRC_EN` writer - clk_spi1_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi1SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SPI2_SRC_EN` reader - clk_spi2_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi2SrcEnR = crate::BitReader;
#[doc = "Field `CLK_SPI2_SRC_EN` writer - clk_spi2_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi2SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SPI4_SRC_EN` reader - clk_spi4_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi4SrcEnR = crate::BitReader;
#[doc = "Field `CLK_SPI4_SRC_EN` writer - clk_spi4_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi4SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - clk_uart0_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart0_src_en(&self) -> ClkUart0SrcEnR {
        ClkUart0SrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_uart0_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart0_frac_src_en(&self) -> ClkUart0FracSrcEnR {
        ClkUart0FracSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_uart1_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart1_src_en(&self) -> ClkUart1SrcEnR {
        ClkUart1SrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_uart1_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart1_frac_src_en(&self) -> ClkUart1FracSrcEnR {
        ClkUart1FracSrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_uart2_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart2_src_en(&self) -> ClkUart2SrcEnR {
        ClkUart2SrcEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_uart2_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart2_frac_src_en(&self) -> ClkUart2FracSrcEnR {
        ClkUart2FracSrcEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_uart3_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart3_src_en(&self) -> ClkUart3SrcEnR {
        ClkUart3SrcEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_uart3_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart3_frac_src_en(&self) -> ClkUart3FracSrcEnR {
        ClkUart3FracSrcEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - clk_tsadc_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_tsadc_src_en(&self) -> ClkTsadcSrcEnR {
        ClkTsadcSrcEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - clk_saradc_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_saradc_src_en(&self) -> ClkSaradcSrcEnR {
        ClkSaradcSrcEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - clk_spi0_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_spi0_src_en(&self) -> ClkSpi0SrcEnR {
        ClkSpi0SrcEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - clk_spi1_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_spi1_src_en(&self) -> ClkSpi1SrcEnR {
        ClkSpi1SrcEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - clk_spi2_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_spi2_src_en(&self) -> ClkSpi2SrcEnR {
        ClkSpi2SrcEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - clk_spi4_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_spi4_src_en(&self) -> ClkSpi4SrcEnR {
        ClkSpi4SrcEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_uart0_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart0_src_en(&mut self) -> ClkUart0SrcEnW<ClkgateCon9Spec> {
        ClkUart0SrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_uart0_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart0_frac_src_en(&mut self) -> ClkUart0FracSrcEnW<ClkgateCon9Spec> {
        ClkUart0FracSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_uart1_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart1_src_en(&mut self) -> ClkUart1SrcEnW<ClkgateCon9Spec> {
        ClkUart1SrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_uart1_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart1_frac_src_en(&mut self) -> ClkUart1FracSrcEnW<ClkgateCon9Spec> {
        ClkUart1FracSrcEnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_uart2_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart2_src_en(&mut self) -> ClkUart2SrcEnW<ClkgateCon9Spec> {
        ClkUart2SrcEnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_uart2_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart2_frac_src_en(&mut self) -> ClkUart2FracSrcEnW<ClkgateCon9Spec> {
        ClkUart2FracSrcEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_uart3_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart3_src_en(&mut self) -> ClkUart3SrcEnW<ClkgateCon9Spec> {
        ClkUart3SrcEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_uart3_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart3_frac_src_en(&mut self) -> ClkUart3FracSrcEnW<ClkgateCon9Spec> {
        ClkUart3FracSrcEnW::new(self, 7)
    }
    #[doc = "Bit 10 - clk_tsadc_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_tsadc_src_en(&mut self) -> ClkTsadcSrcEnW<ClkgateCon9Spec> {
        ClkTsadcSrcEnW::new(self, 10)
    }
    #[doc = "Bit 11 - clk_saradc_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_saradc_src_en(&mut self) -> ClkSaradcSrcEnW<ClkgateCon9Spec> {
        ClkSaradcSrcEnW::new(self, 11)
    }
    #[doc = "Bit 12 - clk_spi0_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi0_src_en(&mut self) -> ClkSpi0SrcEnW<ClkgateCon9Spec> {
        ClkSpi0SrcEnW::new(self, 12)
    }
    #[doc = "Bit 13 - clk_spi1_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi1_src_en(&mut self) -> ClkSpi1SrcEnW<ClkgateCon9Spec> {
        ClkSpi1SrcEnW::new(self, 13)
    }
    #[doc = "Bit 14 - clk_spi2_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi2_src_en(&mut self) -> ClkSpi2SrcEnW<ClkgateCon9Spec> {
        ClkSpi2SrcEnW::new(self, 14)
    }
    #[doc = "Bit 15 - clk_spi4_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi4_src_en(&mut self) -> ClkSpi4SrcEnW<ClkgateCon9Spec> {
        ClkSpi4SrcEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon9Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon9Spec;
impl crate::RegisterSpec for ClkgateCon9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con9::R`](R) reader structure"]
impl crate::Readable for ClkgateCon9Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con9::W`](W) writer structure"]
impl crate::Writable for ClkgateCon9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON9 to value 0"]
impl crate::Resettable for ClkgateCon9Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SOFTRST_CON13` reader"]
pub type R = crate::R<SoftrstCon13Spec>;
#[doc = "Register `SOFTRST_CON13` writer"]
pub type W = crate::W<SoftrstCon13Spec>;
#[doc = "Field `PRESETN_UART0_REQ` reader - presetn_uart0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUart0ReqR = crate::BitReader;
#[doc = "Field `PRESETN_UART0_REQ` writer - presetn_uart0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUart0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UART1_REQ` reader - presetn_uart1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUart1ReqR = crate::BitReader;
#[doc = "Field `PRESETN_UART1_REQ` writer - presetn_uart1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUart1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UART2_REQ` reader - presetn_uart2 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUart2ReqR = crate::BitReader;
#[doc = "Field `PRESETN_UART2_REQ` writer - presetn_uart2 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUart2ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UART3_REQ` reader - presetn_uart3 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUart3ReqR = crate::BitReader;
#[doc = "Field `PRESETN_UART3_REQ` writer - presetn_uart3 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUart3ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SARADC_REQ` reader - presetn_saradc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSaradcReqR = crate::BitReader;
#[doc = "Field `PRESETN_SARADC_REQ` writer - presetn_saradc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSaradcReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_TSADC_REQ` reader - presetn_tsadc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnTsadcReqR = crate::BitReader;
#[doc = "Field `PRESETN_TSADC_REQ` writer - presetn_tsadc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnTsadcReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SPI0_REQ` reader - presetn_spi0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi0ReqR = crate::BitReader;
#[doc = "Field `PRESETN_SPI0_REQ` writer - presetn_spi0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SPI1_REQ` reader - presetn_spi1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi1ReqR = crate::BitReader;
#[doc = "Field `PRESETN_SPI1_REQ` writer - presetn_spi1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SPI2_REQ` reader - presetn_spi2 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi2ReqR = crate::BitReader;
#[doc = "Field `PRESETN_SPI2_REQ` writer - presetn_spi2 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi2ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SPI4_REQ` reader - presetn_spi4 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi4ReqR = crate::BitReader;
#[doc = "Field `PRESETN_SPI4_REQ` writer - presetn_spi4 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi4ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SPI5_REQ` reader - presetn_spi5 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi5ReqR = crate::BitReader;
#[doc = "Field `PRESETN_SPI5_REQ` writer - presetn_spi5 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi5ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_SPI0_REQ` reader - resetn_spi0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi0ReqR = crate::BitReader;
#[doc = "Field `RESETN_SPI0_REQ` writer - resetn_spi0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_SPI1_REQ` reader - resetn_spi1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi1ReqR = crate::BitReader;
#[doc = "Field `RESETN_SPI1_REQ` writer - resetn_spi1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_SPI2_REQ` reader - resetn_spi2 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi2ReqR = crate::BitReader;
#[doc = "Field `RESETN_SPI2_REQ` writer - resetn_spi2 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi2ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_SPI4_REQ` reader - resetn_spi4 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi4ReqR = crate::BitReader;
#[doc = "Field `RESETN_SPI4_REQ` writer - resetn_spi4 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi4ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_SPI5_REQ` reader - resetn_spi5 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi5ReqR = crate::BitReader;
#[doc = "Field `RESETN_SPI5_REQ` writer - resetn_spi5 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi5ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - presetn_uart0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uart0_req(&self) -> PresetnUart0ReqR {
        PresetnUart0ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - presetn_uart1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uart1_req(&self) -> PresetnUart1ReqR {
        PresetnUart1ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - presetn_uart2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uart2_req(&self) -> PresetnUart2ReqR {
        PresetnUart2ReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - presetn_uart3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uart3_req(&self) -> PresetnUart3ReqR {
        PresetnUart3ReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - presetn_saradc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_saradc_req(&self) -> PresetnSaradcReqR {
        PresetnSaradcReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - presetn_tsadc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_tsadc_req(&self) -> PresetnTsadcReqR {
        PresetnTsadcReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - presetn_spi0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_spi0_req(&self) -> PresetnSpi0ReqR {
        PresetnSpi0ReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - presetn_spi1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_spi1_req(&self) -> PresetnSpi1ReqR {
        PresetnSpi1ReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - presetn_spi2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_spi2_req(&self) -> PresetnSpi2ReqR {
        PresetnSpi2ReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - presetn_spi4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_spi4_req(&self) -> PresetnSpi4ReqR {
        PresetnSpi4ReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - presetn_spi5 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_spi5_req(&self) -> PresetnSpi5ReqR {
        PresetnSpi5ReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - resetn_spi0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_spi0_req(&self) -> ResetnSpi0ReqR {
        ResetnSpi0ReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - resetn_spi1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_spi1_req(&self) -> ResetnSpi1ReqR {
        ResetnSpi1ReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - resetn_spi2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_spi2_req(&self) -> ResetnSpi2ReqR {
        ResetnSpi2ReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - resetn_spi4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_spi4_req(&self) -> ResetnSpi4ReqR {
        ResetnSpi4ReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - resetn_spi5 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_spi5_req(&self) -> ResetnSpi5ReqR {
        ResetnSpi5ReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - presetn_uart0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uart0_req(&mut self) -> PresetnUart0ReqW<SoftrstCon13Spec> {
        PresetnUart0ReqW::new(self, 0)
    }
    #[doc = "Bit 1 - presetn_uart1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uart1_req(&mut self) -> PresetnUart1ReqW<SoftrstCon13Spec> {
        PresetnUart1ReqW::new(self, 1)
    }
    #[doc = "Bit 2 - presetn_uart2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uart2_req(&mut self) -> PresetnUart2ReqW<SoftrstCon13Spec> {
        PresetnUart2ReqW::new(self, 2)
    }
    #[doc = "Bit 3 - presetn_uart3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uart3_req(&mut self) -> PresetnUart3ReqW<SoftrstCon13Spec> {
        PresetnUart3ReqW::new(self, 3)
    }
    #[doc = "Bit 4 - presetn_saradc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_saradc_req(&mut self) -> PresetnSaradcReqW<SoftrstCon13Spec> {
        PresetnSaradcReqW::new(self, 4)
    }
    #[doc = "Bit 5 - presetn_tsadc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_tsadc_req(&mut self) -> PresetnTsadcReqW<SoftrstCon13Spec> {
        PresetnTsadcReqW::new(self, 5)
    }
    #[doc = "Bit 6 - presetn_spi0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_spi0_req(&mut self) -> PresetnSpi0ReqW<SoftrstCon13Spec> {
        PresetnSpi0ReqW::new(self, 6)
    }
    #[doc = "Bit 7 - presetn_spi1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_spi1_req(&mut self) -> PresetnSpi1ReqW<SoftrstCon13Spec> {
        PresetnSpi1ReqW::new(self, 7)
    }
    #[doc = "Bit 8 - presetn_spi2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_spi2_req(&mut self) -> PresetnSpi2ReqW<SoftrstCon13Spec> {
        PresetnSpi2ReqW::new(self, 8)
    }
    #[doc = "Bit 9 - presetn_spi4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_spi4_req(&mut self) -> PresetnSpi4ReqW<SoftrstCon13Spec> {
        PresetnSpi4ReqW::new(self, 9)
    }
    #[doc = "Bit 10 - presetn_spi5 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_spi5_req(&mut self) -> PresetnSpi5ReqW<SoftrstCon13Spec> {
        PresetnSpi5ReqW::new(self, 10)
    }
    #[doc = "Bit 11 - resetn_spi0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_spi0_req(&mut self) -> ResetnSpi0ReqW<SoftrstCon13Spec> {
        ResetnSpi0ReqW::new(self, 11)
    }
    #[doc = "Bit 12 - resetn_spi1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_spi1_req(&mut self) -> ResetnSpi1ReqW<SoftrstCon13Spec> {
        ResetnSpi1ReqW::new(self, 12)
    }
    #[doc = "Bit 13 - resetn_spi2 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_spi2_req(&mut self) -> ResetnSpi2ReqW<SoftrstCon13Spec> {
        ResetnSpi2ReqW::new(self, 13)
    }
    #[doc = "Bit 14 - resetn_spi4 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_spi4_req(&mut self) -> ResetnSpi4ReqW<SoftrstCon13Spec> {
        ResetnSpi4ReqW::new(self, 14)
    }
    #[doc = "Bit 15 - resetn_spi5 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_spi5_req(&mut self) -> ResetnSpi5ReqW<SoftrstCon13Spec> {
        ResetnSpi5ReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon13Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon13Spec;
impl crate::RegisterSpec for SoftrstCon13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con13::R`](R) reader structure"]
impl crate::Readable for SoftrstCon13Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con13::W`](W) writer structure"]
impl crate::Writable for SoftrstCon13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON13 to value 0"]
impl crate::Resettable for SoftrstCon13Spec {
    const RESET_VALUE: u32 = 0;
}

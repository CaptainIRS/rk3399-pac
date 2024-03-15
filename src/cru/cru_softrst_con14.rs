#[doc = "Register `CRU_SOFTRST_CON14` reader"]
pub type R = crate::R<CruSoftrstCon14Spec>;
#[doc = "Register `CRU_SOFTRST_CON14` writer"]
pub type W = crate::W<CruSoftrstCon14Spec>;
#[doc = "Field `RESETN_I2S0_REQ` reader - resetn_i2s0 request bit When HIGH, reset relative logic"]
pub type ResetnI2s0ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2S0_REQ` writer - resetn_i2s0 request bit When HIGH, reset relative logic"]
pub type ResetnI2s0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2S1_REQ` reader - resetn_i2s1 request bit When HIGH, reset relative logic"]
pub type ResetnI2s1ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2S1_REQ` writer - resetn_i2s1 request bit When HIGH, reset relative logic"]
pub type ResetnI2s1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2S2_REQ` reader - resetn_i2s2 request bit When HIGH, reset relative logic"]
pub type ResetnI2s2ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2S2_REQ` writer - resetn_i2s2 request bit When HIGH, reset relative logic"]
pub type ResetnI2s2ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_SPDIF_8CH_REQ` reader - resetn_spdif_8ch request bit When HIGH, reset relative logic"]
pub type ResetnSpdif8chReqR = crate::BitReader;
#[doc = "Field `RESETN_SPDIF_8CH_REQ` writer - resetn_spdif_8ch request bit When HIGH, reset relative logic"]
pub type ResetnSpdif8chReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UART0_REQ` reader - resetn_uart0 request bit When HIGH, reset relative logic"]
pub type ResetnUart0ReqR = crate::BitReader;
#[doc = "Field `RESETN_UART0_REQ` writer - resetn_uart0 request bit When HIGH, reset relative logic"]
pub type ResetnUart0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UART1_REQ` reader - resetn_uart1 request bit When HIGH, reset relative logic"]
pub type ResetnUart1ReqR = crate::BitReader;
#[doc = "Field `RESETN_UART1_REQ` writer - resetn_uart1 request bit When HIGH, reset relative logic"]
pub type ResetnUart1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UART2_REQ` reader - resetn_uart2 request bit When HIGH, reset relative logic"]
pub type ResetnUart2ReqR = crate::BitReader;
#[doc = "Field `RESETN_UART2_REQ` writer - resetn_uart2 request bit When HIGH, reset relative logic"]
pub type ResetnUart2ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UART3_REQ` reader - resetn_uart3 request bit When HIGH, reset relative logic"]
pub type ResetnUart3ReqR = crate::BitReader;
#[doc = "Field `RESETN_UART3_REQ` writer - resetn_uart3 request bit When HIGH, reset relative logic"]
pub type ResetnUart3ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_TSADC_REQ` reader - resetn_tsadc request bit When HIGH, reset relative logic"]
pub type ResetnTsadcReqR = crate::BitReader;
#[doc = "Field `RESETN_TSADC_REQ` writer - resetn_tsadc request bit When HIGH, reset relative logic"]
pub type ResetnTsadcReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C1_REQ` reader - resetn_i2c1 request bit When HIGH, reset relative logic"]
pub type ResetnI2c1ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2C1_REQ` writer - resetn_i2c1 request bit When HIGH, reset relative logic"]
pub type ResetnI2c1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C5_REQ` reader - resetn_i2c5 request bit When HIGH, reset relative logic"]
pub type ResetnI2c5ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2C5_REQ` writer - resetn_i2c5 request bit When HIGH, reset relative logic"]
pub type ResetnI2c5ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C2_REQ` reader - resetn_i2c2 request bit When HIGH, reset relative logic"]
pub type ResetnI2c2ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2C2_REQ` writer - resetn_i2c2 request bit When HIGH, reset relative logic"]
pub type ResetnI2c2ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C6_REQ` reader - resetn_i2c6 request bit When HIGH, reset relative logic"]
pub type ResetnI2c6ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2C6_REQ` writer - resetn_i2c6 request bit When HIGH, reset relative logic"]
pub type ResetnI2c6ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C3_REQ` reader - resetn_i2c3 request bit When HIGH, reset relative logic"]
pub type ResetnI2c3ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2C3_REQ` writer - resetn_i2c3 request bit When HIGH, reset relative logic"]
pub type ResetnI2c3ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_I2C7_REQ` reader - resetn_i2c7 request bit When HIGH, reset relative logic"]
pub type ResetnI2c7ReqR = crate::BitReader;
#[doc = "Field `RESETN_I2C7_REQ` writer - resetn_i2c7 request bit When HIGH, reset relative logic"]
pub type ResetnI2c7ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_SDIOAUDIO_NOC_REQ` reader - hresetn_sdioaudio_noc request bit When HIGH, reset relative logic"]
pub type HresetnSdioaudioNocReqR = crate::BitReader;
#[doc = "Field `HRESETN_SDIOAUDIO_NOC_REQ` writer - hresetn_sdioaudio_noc request bit When HIGH, reset relative logic"]
pub type HresetnSdioaudioNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - resetn_i2s0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2s0_req(&self) -> ResetnI2s0ReqR {
        ResetnI2s0ReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - resetn_i2s1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2s1_req(&self) -> ResetnI2s1ReqR {
        ResetnI2s1ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - resetn_i2s2 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2s2_req(&self) -> ResetnI2s2ReqR {
        ResetnI2s2ReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - resetn_spdif_8ch request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_spdif_8ch_req(&self) -> ResetnSpdif8chReqR {
        ResetnSpdif8chReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - resetn_uart0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uart0_req(&self) -> ResetnUart0ReqR {
        ResetnUart0ReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - resetn_uart1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uart1_req(&self) -> ResetnUart1ReqR {
        ResetnUart1ReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - resetn_uart2 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uart2_req(&self) -> ResetnUart2ReqR {
        ResetnUart2ReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - resetn_uart3 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uart3_req(&self) -> ResetnUart3ReqR {
        ResetnUart3ReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - resetn_tsadc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_tsadc_req(&self) -> ResetnTsadcReqR {
        ResetnTsadcReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - resetn_i2c1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2c1_req(&self) -> ResetnI2c1ReqR {
        ResetnI2c1ReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - resetn_i2c5 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2c5_req(&self) -> ResetnI2c5ReqR {
        ResetnI2c5ReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - resetn_i2c2 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2c2_req(&self) -> ResetnI2c2ReqR {
        ResetnI2c2ReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - resetn_i2c6 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2c6_req(&self) -> ResetnI2c6ReqR {
        ResetnI2c6ReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - resetn_i2c3 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2c3_req(&self) -> ResetnI2c3ReqR {
        ResetnI2c3ReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - resetn_i2c7 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_i2c7_req(&self) -> ResetnI2c7ReqR {
        ResetnI2c7ReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - hresetn_sdioaudio_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_sdioaudio_noc_req(&self) -> HresetnSdioaudioNocReqR {
        HresetnSdioaudioNocReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - resetn_i2s0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2s0_req(&mut self) -> ResetnI2s0ReqW<CruSoftrstCon14Spec> {
        ResetnI2s0ReqW::new(self, 0)
    }
    #[doc = "Bit 1 - resetn_i2s1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2s1_req(&mut self) -> ResetnI2s1ReqW<CruSoftrstCon14Spec> {
        ResetnI2s1ReqW::new(self, 1)
    }
    #[doc = "Bit 2 - resetn_i2s2 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2s2_req(&mut self) -> ResetnI2s2ReqW<CruSoftrstCon14Spec> {
        ResetnI2s2ReqW::new(self, 2)
    }
    #[doc = "Bit 3 - resetn_spdif_8ch request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_spdif_8ch_req(&mut self) -> ResetnSpdif8chReqW<CruSoftrstCon14Spec> {
        ResetnSpdif8chReqW::new(self, 3)
    }
    #[doc = "Bit 4 - resetn_uart0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uart0_req(&mut self) -> ResetnUart0ReqW<CruSoftrstCon14Spec> {
        ResetnUart0ReqW::new(self, 4)
    }
    #[doc = "Bit 5 - resetn_uart1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uart1_req(&mut self) -> ResetnUart1ReqW<CruSoftrstCon14Spec> {
        ResetnUart1ReqW::new(self, 5)
    }
    #[doc = "Bit 6 - resetn_uart2 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uart2_req(&mut self) -> ResetnUart2ReqW<CruSoftrstCon14Spec> {
        ResetnUart2ReqW::new(self, 6)
    }
    #[doc = "Bit 7 - resetn_uart3 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uart3_req(&mut self) -> ResetnUart3ReqW<CruSoftrstCon14Spec> {
        ResetnUart3ReqW::new(self, 7)
    }
    #[doc = "Bit 8 - resetn_tsadc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_tsadc_req(&mut self) -> ResetnTsadcReqW<CruSoftrstCon14Spec> {
        ResetnTsadcReqW::new(self, 8)
    }
    #[doc = "Bit 9 - resetn_i2c1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c1_req(&mut self) -> ResetnI2c1ReqW<CruSoftrstCon14Spec> {
        ResetnI2c1ReqW::new(self, 9)
    }
    #[doc = "Bit 10 - resetn_i2c5 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c5_req(&mut self) -> ResetnI2c5ReqW<CruSoftrstCon14Spec> {
        ResetnI2c5ReqW::new(self, 10)
    }
    #[doc = "Bit 11 - resetn_i2c2 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c2_req(&mut self) -> ResetnI2c2ReqW<CruSoftrstCon14Spec> {
        ResetnI2c2ReqW::new(self, 11)
    }
    #[doc = "Bit 12 - resetn_i2c6 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c6_req(&mut self) -> ResetnI2c6ReqW<CruSoftrstCon14Spec> {
        ResetnI2c6ReqW::new(self, 12)
    }
    #[doc = "Bit 13 - resetn_i2c3 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c3_req(&mut self) -> ResetnI2c3ReqW<CruSoftrstCon14Spec> {
        ResetnI2c3ReqW::new(self, 13)
    }
    #[doc = "Bit 14 - resetn_i2c7 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_i2c7_req(&mut self) -> ResetnI2c7ReqW<CruSoftrstCon14Spec> {
        ResetnI2c7ReqW::new(self, 14)
    }
    #[doc = "Bit 15 - hresetn_sdioaudio_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_sdioaudio_noc_req(&mut self) -> HresetnSdioaudioNocReqW<CruSoftrstCon14Spec> {
        HresetnSdioaudioNocReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon14Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon14Spec;
impl crate::RegisterSpec for CruSoftrstCon14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con14::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon14Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con14::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON14 to value 0"]
impl crate::Resettable for CruSoftrstCon14Spec {
    const RESET_VALUE: u32 = 0;
}

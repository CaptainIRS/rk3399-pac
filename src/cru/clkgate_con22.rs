#[doc = "Register `CLKGATE_CON22` reader"]
pub type R = crate::R<ClkgateCon22Spec>;
#[doc = "Register `CLKGATE_CON22` writer"]
pub type W = crate::W<ClkgateCon22Spec>;
#[doc = "Field `PCLK_UART0_EN` reader - pclk_uart0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkUart0EnR = crate::BitReader;
#[doc = "Field `PCLK_UART0_EN` writer - pclk_uart0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkUart0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_UART1_EN` reader - pclk_uart1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkUart1EnR = crate::BitReader;
#[doc = "Field `PCLK_UART1_EN` writer - pclk_uart1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkUart1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_UART2_EN` reader - pclk_uart2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkUart2EnR = crate::BitReader;
#[doc = "Field `PCLK_UART2_EN` writer - pclk_uart2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkUart2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_UART3_EN` reader - pclk_uart3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkUart3EnR = crate::BitReader;
#[doc = "Field `PCLK_UART3_EN` writer - pclk_uart3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkUart3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_RKI2C7_EN` reader - pclk_rki2c7 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c7EnR = crate::BitReader;
#[doc = "Field `PCLK_RKI2C7_EN` writer - pclk_rki2c7 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_RKI2C1CAM_EN` reader - pclk_rki2c1cam clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c1camEnR = crate::BitReader;
#[doc = "Field `PCLK_RKI2C1CAM_EN` writer - pclk_rki2c1cam clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c1camEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_RKI2C5PAD_EN` reader - pclk_rki2c5pad clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c5padEnR = crate::BitReader;
#[doc = "Field `PCLK_RKI2C5PAD_EN` writer - pclk_rki2c5pad clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c5padEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_RKI2C6_EN` reader - pclk_rki2c6 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c6EnR = crate::BitReader;
#[doc = "Field `PCLK_RKI2C6_EN` writer - pclk_rki2c6 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_RKI2C2_EN` reader - pclk_rki2c2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c2EnR = crate::BitReader;
#[doc = "Field `PCLK_RKI2C2_EN` writer - pclk_rki2c2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_RKI2C3_EN` reader - pclk_rki2c3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c3EnR = crate::BitReader;
#[doc = "Field `PCLK_RKI2C3_EN` writer - pclk_rki2c3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkRki2c3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_MAILBOX0_EN` reader - pclk_mailbox0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkMailbox0EnR = crate::BitReader;
#[doc = "Field `PCLK_MAILBOX0_EN` writer - pclk_mailbox0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkMailbox0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_SARADC_EN` reader - pclk_saradc clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSaradcEnR = crate::BitReader;
#[doc = "Field `PCLK_SARADC_EN` writer - pclk_saradc clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkSaradcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_TSADC_EN` reader - pclk_tsadc clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkTsadcEnR = crate::BitReader;
#[doc = "Field `PCLK_TSADC_EN` writer - pclk_tsadc clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkTsadcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_EFUSE1024NS_EN` reader - pclk_efuse1024ns clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkEfuse1024nsEnR = crate::BitReader;
#[doc = "Field `PCLK_EFUSE1024NS_EN` writer - pclk_efuse1024ns clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkEfuse1024nsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_EFUSE1024S_EN` reader - pclk_efuse1024s clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkEfuse1024sEnR = crate::BitReader;
#[doc = "Field `PCLK_EFUSE1024S_EN` writer - pclk_efuse1024s clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkEfuse1024sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pclk_uart0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_uart0_en(&self) -> PclkUart0EnR {
        PclkUart0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pclk_uart1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_uart1_en(&self) -> PclkUart1EnR {
        PclkUart1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pclk_uart2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_uart2_en(&self) -> PclkUart2EnR {
        PclkUart2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pclk_uart3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_uart3_en(&self) -> PclkUart3EnR {
        PclkUart3EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - pclk_rki2c7 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_rki2c7_en(&self) -> PclkRki2c7EnR {
        PclkRki2c7EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pclk_rki2c1cam clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_rki2c1cam_en(&self) -> PclkRki2c1camEnR {
        PclkRki2c1camEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pclk_rki2c5pad clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_rki2c5pad_en(&self) -> PclkRki2c5padEnR {
        PclkRki2c5padEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pclk_rki2c6 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_rki2c6_en(&self) -> PclkRki2c6EnR {
        PclkRki2c6EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pclk_rki2c2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_rki2c2_en(&self) -> PclkRki2c2EnR {
        PclkRki2c2EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pclk_rki2c3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_rki2c3_en(&self) -> PclkRki2c3EnR {
        PclkRki2c3EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pclk_mailbox0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_mailbox0_en(&self) -> PclkMailbox0EnR {
        PclkMailbox0EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pclk_saradc clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_saradc_en(&self) -> PclkSaradcEnR {
        PclkSaradcEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pclk_tsadc clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_tsadc_en(&self) -> PclkTsadcEnR {
        PclkTsadcEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pclk_efuse1024ns clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_efuse1024ns_en(&self) -> PclkEfuse1024nsEnR {
        PclkEfuse1024nsEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pclk_efuse1024s clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_efuse1024s_en(&self) -> PclkEfuse1024sEnR {
        PclkEfuse1024sEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - pclk_uart0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart0_en(&mut self) -> PclkUart0EnW<ClkgateCon22Spec> {
        PclkUart0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - pclk_uart1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart1_en(&mut self) -> PclkUart1EnW<ClkgateCon22Spec> {
        PclkUart1EnW::new(self, 1)
    }
    #[doc = "Bit 2 - pclk_uart2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart2_en(&mut self) -> PclkUart2EnW<ClkgateCon22Spec> {
        PclkUart2EnW::new(self, 2)
    }
    #[doc = "Bit 3 - pclk_uart3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart3_en(&mut self) -> PclkUart3EnW<ClkgateCon22Spec> {
        PclkUart3EnW::new(self, 3)
    }
    #[doc = "Bit 5 - pclk_rki2c7 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rki2c7_en(&mut self) -> PclkRki2c7EnW<ClkgateCon22Spec> {
        PclkRki2c7EnW::new(self, 5)
    }
    #[doc = "Bit 6 - pclk_rki2c1cam clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rki2c1cam_en(&mut self) -> PclkRki2c1camEnW<ClkgateCon22Spec> {
        PclkRki2c1camEnW::new(self, 6)
    }
    #[doc = "Bit 7 - pclk_rki2c5pad clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rki2c5pad_en(&mut self) -> PclkRki2c5padEnW<ClkgateCon22Spec> {
        PclkRki2c5padEnW::new(self, 7)
    }
    #[doc = "Bit 8 - pclk_rki2c6 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rki2c6_en(&mut self) -> PclkRki2c6EnW<ClkgateCon22Spec> {
        PclkRki2c6EnW::new(self, 8)
    }
    #[doc = "Bit 9 - pclk_rki2c2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rki2c2_en(&mut self) -> PclkRki2c2EnW<ClkgateCon22Spec> {
        PclkRki2c2EnW::new(self, 9)
    }
    #[doc = "Bit 10 - pclk_rki2c3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rki2c3_en(&mut self) -> PclkRki2c3EnW<ClkgateCon22Spec> {
        PclkRki2c3EnW::new(self, 10)
    }
    #[doc = "Bit 11 - pclk_mailbox0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_mailbox0_en(&mut self) -> PclkMailbox0EnW<ClkgateCon22Spec> {
        PclkMailbox0EnW::new(self, 11)
    }
    #[doc = "Bit 12 - pclk_saradc clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_saradc_en(&mut self) -> PclkSaradcEnW<ClkgateCon22Spec> {
        PclkSaradcEnW::new(self, 12)
    }
    #[doc = "Bit 13 - pclk_tsadc clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_tsadc_en(&mut self) -> PclkTsadcEnW<ClkgateCon22Spec> {
        PclkTsadcEnW::new(self, 13)
    }
    #[doc = "Bit 14 - pclk_efuse1024ns clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_efuse1024ns_en(&mut self) -> PclkEfuse1024nsEnW<ClkgateCon22Spec> {
        PclkEfuse1024nsEnW::new(self, 14)
    }
    #[doc = "Bit 15 - pclk_efuse1024s clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_efuse1024s_en(&mut self) -> PclkEfuse1024sEnW<ClkgateCon22Spec> {
        PclkEfuse1024sEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon22Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon22Spec;
impl crate::RegisterSpec for ClkgateCon22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con22::R`](R) reader structure"]
impl crate::Readable for ClkgateCon22Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con22::W`](W) writer structure"]
impl crate::Writable for ClkgateCon22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON22 to value 0"]
impl crate::Resettable for ClkgateCon22Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `I2S_INTCR` reader"]
pub type R = crate::R<I2sIntcrSpec>;
#[doc = "Register `I2S_INTCR` writer"]
pub type W = crate::W<I2sIntcrSpec>;
#[doc = "TX empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txeie {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Txeie> for bool {
    #[inline(always)]
    fn from(variant: Txeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - TX empty interrupt enable"]
pub type TxeieR = crate::BitReader<Txeie>;
impl TxeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txeie {
        match self.bits {
            false => Txeie::B0,
            true => Txeie::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txeie::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txeie::B1
    }
}
#[doc = "Field `TXEIE` writer - TX empty interrupt enable"]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG, Txeie>;
impl<'a, REG> TxeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txeie::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txeie::B1)
    }
}
#[doc = "TX underrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txuie {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Txuie> for bool {
    #[inline(always)]
    fn from(variant: Txuie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUIE` reader - TX underrun interrupt enable"]
pub type TxuieR = crate::BitReader<Txuie>;
impl TxuieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txuie {
        match self.bits {
            false => Txuie::B0,
            true => Txuie::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txuie::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txuie::B1
    }
}
#[doc = "Field `TXUIE` writer - TX underrun interrupt enable"]
pub type TxuieW<'a, REG> = crate::BitWriter<'a, REG, Txuie>;
impl<'a, REG> TxuieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txuie::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txuie::B1)
    }
}
#[doc = "Field `TXUIC` writer - TX underrun interrupt clear Write 1 to clear TX underrun interrupt."]
pub type TxuicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFT` reader - Transmit FIFO Threshold When the number of transmit FIFO (TXFIFO0 if TCSR=00; TXFIFO1 if TCSR=01, TXFIFO2 if TCSR=10, TXFIFO3 if TCSR=11) entries is less than or equal to this threshold, the transmit FIFO empty interrupt is triggered."]
pub type TftR = crate::FieldReader;
#[doc = "Field `TFT` writer - Transmit FIFO Threshold When the number of transmit FIFO (TXFIFO0 if TCSR=00; TXFIFO1 if TCSR=01, TXFIFO2 if TCSR=10, TXFIFO3 if TCSR=11) entries is less than or equal to this threshold, the transmit FIFO empty interrupt is triggered."]
pub type TftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "RX full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfie {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Rxfie> for bool {
    #[inline(always)]
    fn from(variant: Rxfie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIE` reader - RX full interrupt enable"]
pub type RxfieR = crate::BitReader<Rxfie>;
impl RxfieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfie {
        match self.bits {
            false => Rxfie::B0,
            true => Rxfie::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxfie::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxfie::B1
    }
}
#[doc = "Field `RXFIE` writer - RX full interrupt enable"]
pub type RxfieW<'a, REG> = crate::BitWriter<'a, REG, Rxfie>;
impl<'a, REG> RxfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfie::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfie::B1)
    }
}
#[doc = "RX overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxoie {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Rxoie> for bool {
    #[inline(always)]
    fn from(variant: Rxoie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOIE` reader - RX overrun interrupt enable"]
pub type RxoieR = crate::BitReader<Rxoie>;
impl RxoieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxoie {
        match self.bits {
            false => Rxoie::B0,
            true => Rxoie::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxoie::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxoie::B1
    }
}
#[doc = "Field `RXOIE` writer - RX overrun interrupt enable"]
pub type RxoieW<'a, REG> = crate::BitWriter<'a, REG, Rxoie>;
impl<'a, REG> RxoieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxoie::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxoie::B1)
    }
}
#[doc = "Field `RXOIC` writer - RX overrun interrupt clear Write 1 to clear RX overrun interrupt."]
pub type RxoicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFT` reader - Receive FIFO Threshold When the number of receive FIFO entries (RXFIFO0 if RCSR=00; RXFIFO1 if RCSR=01, RXFIFO2 if RCSR=10, RXFIFO3 if RCSR=11) is more than or equal to this threshold plus 1, the receive FIFO full interrupt is triggered."]
pub type RftR = crate::FieldReader;
#[doc = "Field `RFT` writer - Receive FIFO Threshold When the number of receive FIFO entries (RXFIFO0 if RCSR=00; RXFIFO1 if RCSR=01, RXFIFO2 if RCSR=10, RXFIFO3 if RCSR=11) is more than or equal to this threshold plus 1, the receive FIFO full interrupt is triggered."]
pub type RftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - TX empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX underrun interrupt enable"]
    #[inline(always)]
    pub fn txuie(&self) -> TxuieR {
        TxuieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:8 - Transmit FIFO Threshold When the number of transmit FIFO (TXFIFO0 if TCSR=00; TXFIFO1 if TCSR=01, TXFIFO2 if TCSR=10, TXFIFO3 if TCSR=11) entries is less than or equal to this threshold, the transmit FIFO empty interrupt is triggered."]
    #[inline(always)]
    pub fn tft(&self) -> TftR {
        TftR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - RX full interrupt enable"]
    #[inline(always)]
    pub fn rxfie(&self) -> RxfieR {
        RxfieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX overrun interrupt enable"]
    #[inline(always)]
    pub fn rxoie(&self) -> RxoieR {
        RxoieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:24 - Receive FIFO Threshold When the number of receive FIFO entries (RXFIFO0 if RCSR=00; RXFIFO1 if RCSR=01, RXFIFO2 if RCSR=10, RXFIFO3 if RCSR=11) is more than or equal to this threshold plus 1, the receive FIFO full interrupt is triggered."]
    #[inline(always)]
    pub fn rft(&self) -> RftR {
        RftR::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TxeieW<I2sIntcrSpec> {
        TxeieW::new(self, 0)
    }
    #[doc = "Bit 1 - TX underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txuie(&mut self) -> TxuieW<I2sIntcrSpec> {
        TxuieW::new(self, 1)
    }
    #[doc = "Bit 2 - TX underrun interrupt clear Write 1 to clear TX underrun interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txuic(&mut self) -> TxuicW<I2sIntcrSpec> {
        TxuicW::new(self, 2)
    }
    #[doc = "Bits 4:8 - Transmit FIFO Threshold When the number of transmit FIFO (TXFIFO0 if TCSR=00; TXFIFO1 if TCSR=01, TXFIFO2 if TCSR=10, TXFIFO3 if TCSR=11) entries is less than or equal to this threshold, the transmit FIFO empty interrupt is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn tft(&mut self) -> TftW<I2sIntcrSpec> {
        TftW::new(self, 4)
    }
    #[doc = "Bit 16 - RX full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfie(&mut self) -> RxfieW<I2sIntcrSpec> {
        RxfieW::new(self, 16)
    }
    #[doc = "Bit 17 - RX overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RxoieW<I2sIntcrSpec> {
        RxoieW::new(self, 17)
    }
    #[doc = "Bit 18 - RX overrun interrupt clear Write 1 to clear RX overrun interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxoic(&mut self) -> RxoicW<I2sIntcrSpec> {
        RxoicW::new(self, 18)
    }
    #[doc = "Bits 20:24 - Receive FIFO Threshold When the number of receive FIFO entries (RXFIFO0 if RCSR=00; RXFIFO1 if RCSR=01, RXFIFO2 if RCSR=10, RXFIFO3 if RCSR=11) is more than or equal to this threshold plus 1, the receive FIFO full interrupt is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn rft(&mut self) -> RftW<I2sIntcrSpec> {
        RftW::new(self, 20)
    }
}
#[doc = "interrupt control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_intcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_intcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sIntcrSpec;
impl crate::RegisterSpec for I2sIntcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_intcr::R`](R) reader structure"]
impl crate::Readable for I2sIntcrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_intcr::W`](W) writer structure"]
impl crate::Writable for I2sIntcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2S_INTCR to value 0"]
impl crate::Resettable for I2sIntcrSpec {
    const RESET_VALUE: u32 = 0;
}

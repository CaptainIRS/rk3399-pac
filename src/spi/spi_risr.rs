#[doc = "Register `SPI_RISR` reader"]
pub type R = crate::R<SpiRisrSpec>;
#[doc = "Transmit FIFO Empty Raw Interrupt Status\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tferis {
    #[doc = "0: spi_txe_intr interrupt is not active prior to masking"]
    B0 = 0,
    #[doc = "1: spi_txe_intr interrupt is active prior to masking"]
    B1 = 1,
}
impl From<Tferis> for bool {
    #[inline(always)]
    fn from(variant: Tferis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFERIS` reader - Transmit FIFO Empty Raw Interrupt Status"]
pub type TferisR = crate::BitReader<Tferis>;
impl TferisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tferis {
        match self.bits {
            false => Tferis::B0,
            true => Tferis::B1,
        }
    }
    #[doc = "spi_txe_intr interrupt is not active prior to masking"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tferis::B0
    }
    #[doc = "spi_txe_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tferis::B1
    }
}
#[doc = "Transmit FIFO Overflow Raw Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tforis {
    #[doc = "0: spi_txo_intr interrupt is not active prior to masking"]
    B0 = 0,
    #[doc = "1: spi_txo_intr interrupt is active prior to masking"]
    B1 = 1,
}
impl From<Tforis> for bool {
    #[inline(always)]
    fn from(variant: Tforis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFORIS` reader - Transmit FIFO Overflow Raw Interrupt Status"]
pub type TforisR = crate::BitReader<Tforis>;
impl TforisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tforis {
        match self.bits {
            false => Tforis::B0,
            true => Tforis::B1,
        }
    }
    #[doc = "spi_txo_intr interrupt is not active prior to masking"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tforis::B0
    }
    #[doc = "spi_txo_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tforis::B1
    }
}
#[doc = "Receive FIFO Underflow Raw Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfuris {
    #[doc = "0: spi_rxu_intr interrupt is not active prior to masking"]
    B0 = 0,
    #[doc = "1: spi_rxu_intr interrupt is active prior to masking"]
    B1 = 1,
}
impl From<Rfuris> for bool {
    #[inline(always)]
    fn from(variant: Rfuris) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFURIS` reader - Receive FIFO Underflow Raw Interrupt Status"]
pub type RfurisR = crate::BitReader<Rfuris>;
impl RfurisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfuris {
        match self.bits {
            false => Rfuris::B0,
            true => Rfuris::B1,
        }
    }
    #[doc = "spi_rxu_intr interrupt is not active prior to masking"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rfuris::B0
    }
    #[doc = "spi_rxu_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rfuris::B1
    }
}
#[doc = "Field `RFORIS` reader - Receive FIFO Overflow Raw Interrupt Status\n\n1'b0 = spi_rxo_intr interrupt is not active prior to masking\n\n1'b1 = spi_rxo_intr interrupt is active prior to masking"]
pub type RforisR = crate::BitReader;
#[doc = "Receive FIFO Full Raw Interrupt Status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffris {
    #[doc = "0: spi_rxf_intr interrupt is not active prior to masking"]
    B0 = 0,
    #[doc = "1: spi_rxf_intr interrupt is full prior to masking"]
    B1 = 1,
}
impl From<Rffris> for bool {
    #[inline(always)]
    fn from(variant: Rffris) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFRIS` reader - Receive FIFO Full Raw Interrupt Status"]
pub type RffrisR = crate::BitReader<Rffris>;
impl RffrisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rffris {
        match self.bits {
            false => Rffris::B0,
            true => Rffris::B1,
        }
    }
    #[doc = "spi_rxf_intr interrupt is not active prior to masking"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rffris::B0
    }
    #[doc = "spi_rxf_intr interrupt is full prior to masking"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rffris::B1
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn tferis(&self) -> TferisR {
        TferisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Raw Interrupt Status"]
    #[inline(always)]
    pub fn tforis(&self) -> TforisR {
        TforisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Raw Interrupt Status"]
    #[inline(always)]
    pub fn rfuris(&self) -> RfurisR {
        RfurisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Raw Interrupt Status\n\n1'b0 = spi_rxo_intr interrupt is not active prior to masking\n\n1'b1 = spi_rxo_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn rforis(&self) -> RforisR {
        RforisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn rffris(&self) -> RffrisR {
        RffrisR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_risr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiRisrSpec;
impl crate::RegisterSpec for SpiRisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_risr::R`](R) reader structure"]
impl crate::Readable for SpiRisrSpec {}
#[doc = "`reset()` method sets SPI_RISR to value 0x01"]
impl crate::Resettable for SpiRisrSpec {
    const RESET_VALUE: u32 = 0x01;
}

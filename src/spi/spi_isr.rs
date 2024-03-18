#[doc = "Register `SPI_ISR` reader"]
pub type R = crate::R<SpiIsrSpec>;
#[doc = "Transmit FIFO Empty Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfeis {
    #[doc = "0: spi_txe_intr interrupt is active after masking"]
    B0 = 0,
    #[doc = "1: spi_txe_intr interrupt is active after masking"]
    B1 = 1,
}
impl From<Tfeis> for bool {
    #[inline(always)]
    fn from(variant: Tfeis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFEIS` reader - Transmit FIFO Empty Interrupt Status"]
pub type TfeisR = crate::BitReader<Tfeis>;
impl TfeisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfeis {
        match self.bits {
            false => Tfeis::B0,
            true => Tfeis::B1,
        }
    }
    #[doc = "spi_txe_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tfeis::B0
    }
    #[doc = "spi_txe_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tfeis::B1
    }
}
#[doc = "Transmit FIFO Overflow Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfois {
    #[doc = "0: spi_txo_intr interrupt is active after masking"]
    B0 = 0,
    #[doc = "1: spi_txo_intr interrupt is active after masking"]
    B1 = 1,
}
impl From<Tfois> for bool {
    #[inline(always)]
    fn from(variant: Tfois) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFOIS` reader - Transmit FIFO Overflow Interrupt Status"]
pub type TfoisR = crate::BitReader<Tfois>;
impl TfoisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfois {
        match self.bits {
            false => Tfois::B0,
            true => Tfois::B1,
        }
    }
    #[doc = "spi_txo_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tfois::B0
    }
    #[doc = "spi_txo_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tfois::B1
    }
}
#[doc = "Receive FIFO Underflow Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfuis {
    #[doc = "0: spi_rxu_intr interrupt is active after masking"]
    B0 = 0,
    #[doc = "1: spi_rxu_intr interrupt is active after masking"]
    B1 = 1,
}
impl From<Rfuis> for bool {
    #[inline(always)]
    fn from(variant: Rfuis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFUIS` reader - Receive FIFO Underflow Interrupt Status"]
pub type RfuisR = crate::BitReader<Rfuis>;
impl RfuisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfuis {
        match self.bits {
            false => Rfuis::B0,
            true => Rfuis::B1,
        }
    }
    #[doc = "spi_rxu_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rfuis::B0
    }
    #[doc = "spi_rxu_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rfuis::B1
    }
}
#[doc = "Receive FIFO Overflow Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfois {
    #[doc = "0: spi_rxo_intr interrupt is active after masking"]
    B0 = 0,
    #[doc = "1: spi_rxo_intr interrupt is active after masking"]
    B1 = 1,
}
impl From<Rfois> for bool {
    #[inline(always)]
    fn from(variant: Rfois) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOIS` reader - Receive FIFO Overflow Interrupt Status"]
pub type RfoisR = crate::BitReader<Rfois>;
impl RfoisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfois {
        match self.bits {
            false => Rfois::B0,
            true => Rfois::B1,
        }
    }
    #[doc = "spi_rxo_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rfois::B0
    }
    #[doc = "spi_rxo_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rfois::B1
    }
}
#[doc = "Receive FIFO Full Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rffis {
    #[doc = "0: spi_rxf_intr interrupt is full after masking"]
    B0 = 0,
    #[doc = "1: spi_rxf_intr interrupt is full after masking"]
    B1 = 1,
}
impl From<Rffis> for bool {
    #[inline(always)]
    fn from(variant: Rffis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFIS` reader - Receive FIFO Full Interrupt Status"]
pub type RffisR = crate::BitReader<Rffis>;
impl RffisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rffis {
        match self.bits {
            false => Rffis::B0,
            true => Rffis::B1,
        }
    }
    #[doc = "spi_rxf_intr interrupt is full after masking"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rffis::B0
    }
    #[doc = "spi_rxf_intr interrupt is full after masking"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rffis::B1
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Status"]
    #[inline(always)]
    pub fn tfeis(&self) -> TfeisR {
        TfeisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Status"]
    #[inline(always)]
    pub fn tfois(&self) -> TfoisR {
        TfoisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Status"]
    #[inline(always)]
    pub fn rfuis(&self) -> RfuisR {
        RfuisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Status"]
    #[inline(always)]
    pub fn rfois(&self) -> RfoisR {
        RfoisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn rffis(&self) -> RffisR {
        RffisR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiIsrSpec;
impl crate::RegisterSpec for SpiIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_isr::R`](R) reader structure"]
impl crate::Readable for SpiIsrSpec {}
#[doc = "`reset()` method sets SPI_ISR to value 0"]
impl crate::Resettable for SpiIsrSpec {
    const RESET_VALUE: u32 = 0;
}

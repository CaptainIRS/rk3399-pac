#[doc = "Register `SPI_SR` reader"]
pub type R = crate::R<SpiSrSpec>;
#[doc = "SPI Busy Flag When set, indicates that a serial transfer is in progress; when cleared indicates that the SPI is idle or disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsf {
    #[doc = "0: SPI is actively transferring data"]
    B0 = 0,
    #[doc = "1: SPI is actively transferring data"]
    B1 = 1,
}
impl From<Bsf> for bool {
    #[inline(always)]
    fn from(variant: Bsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSF` reader - SPI Busy Flag When set, indicates that a serial transfer is in progress; when cleared indicates that the SPI is idle or disabled."]
pub type BsfR = crate::BitReader<Bsf>;
impl BsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsf {
        match self.bits {
            false => Bsf::B0,
            true => Bsf::B1,
        }
    }
    #[doc = "SPI is actively transferring data"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bsf::B0
    }
    #[doc = "SPI is actively transferring data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bsf::B1
    }
}
#[doc = "Transmit FIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tff {
    #[doc = "0: Transmit FIFO is full"]
    B0 = 0,
    #[doc = "1: Transmit FIFO is full"]
    B1 = 1,
}
impl From<Tff> for bool {
    #[inline(always)]
    fn from(variant: Tff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFF` reader - Transmit FIFO Full"]
pub type TffR = crate::BitReader<Tff>;
impl TffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tff {
        match self.bits {
            false => Tff::B0,
            true => Tff::B1,
        }
    }
    #[doc = "Transmit FIFO is full"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tff::B0
    }
    #[doc = "Transmit FIFO is full"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tff::B1
    }
}
#[doc = "Transmit FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfe {
    #[doc = "0: Transmit FIFO is empty"]
    B0 = 0,
    #[doc = "1: Transmit FIFO is empty"]
    B1 = 1,
}
impl From<Tfe> for bool {
    #[inline(always)]
    fn from(variant: Tfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Empty"]
pub type TfeR = crate::BitReader<Tfe>;
impl TfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfe {
        match self.bits {
            false => Tfe::B0,
            true => Tfe::B1,
        }
    }
    #[doc = "Transmit FIFO is empty"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tfe::B0
    }
    #[doc = "Transmit FIFO is empty"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tfe::B1
    }
}
#[doc = "Receive FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfe {
    #[doc = "0: Receive FIFO is empty"]
    B0 = 0,
    #[doc = "1: Receive FIFO is empty"]
    B1 = 1,
}
impl From<Rfe> for bool {
    #[inline(always)]
    fn from(variant: Rfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - Receive FIFO Empty"]
pub type RfeR = crate::BitReader<Rfe>;
impl RfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfe {
        match self.bits {
            false => Rfe::B0,
            true => Rfe::B1,
        }
    }
    #[doc = "Receive FIFO is empty"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rfe::B0
    }
    #[doc = "Receive FIFO is empty"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rfe::B1
    }
}
#[doc = "Receive FIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rff {
    #[doc = "0: Receive FIFO is full"]
    B0 = 0,
    #[doc = "1: Receive FIFO is full"]
    B1 = 1,
}
impl From<Rff> for bool {
    #[inline(always)]
    fn from(variant: Rff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO Full"]
pub type RffR = crate::BitReader<Rff>;
impl RffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rff {
        match self.bits {
            false => Rff::B0,
            true => Rff::B1,
        }
    }
    #[doc = "Receive FIFO is full"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rff::B0
    }
    #[doc = "Receive FIFO is full"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rff::B1
    }
}
impl R {
    #[doc = "Bit 0 - SPI Busy Flag When set, indicates that a serial transfer is in progress; when cleared indicates that the SPI is idle or disabled."]
    #[inline(always)]
    pub fn bsf(&self) -> BsfR {
        BsfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Full"]
    #[inline(always)]
    pub fn tff(&self) -> TffR {
        TffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Empty"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "SPI Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSrSpec;
impl crate::RegisterSpec for SpiSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_sr::R`](R) reader structure"]
impl crate::Readable for SpiSrSpec {}
#[doc = "`reset()` method sets SPI_SR to value 0x0c"]
impl crate::Resettable for SpiSrSpec {
    const RESET_VALUE: u32 = 0x0c;
}

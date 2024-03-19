#[doc = "Register `SPI_DMACR` reader"]
pub type R = crate::R<SpiDmacrSpec>;
#[doc = "Register `SPI_DMACR` writer"]
pub type W = crate::W<SpiDmacrSpec>;
#[doc = "Receive DMA Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rde {
    #[doc = "0: Receive DMA disabled"]
    B0 = 0,
    #[doc = "1: Receive DMA enabled"]
    B1 = 1,
}
impl From<Rde> for bool {
    #[inline(always)]
    fn from(variant: Rde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDE` reader - Receive DMA Enable"]
pub type RdeR = crate::BitReader<Rde>;
impl RdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rde {
        match self.bits {
            false => Rde::B0,
            true => Rde::B1,
        }
    }
    #[doc = "Receive DMA disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rde::B0
    }
    #[doc = "Receive DMA enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rde::B1
    }
}
#[doc = "Field `RDE` writer - Receive DMA Enable"]
pub type RdeW<'a, REG> = crate::BitWriter<'a, REG, Rde>;
impl<'a, REG> RdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive DMA disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rde::B0)
    }
    #[doc = "Receive DMA enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rde::B1)
    }
}
#[doc = "Transmit DMA Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tde {
    #[doc = "0: Transmit DMA disabled"]
    B0 = 0,
    #[doc = "1: Transmit DMA enabled"]
    B1 = 1,
}
impl From<Tde> for bool {
    #[inline(always)]
    fn from(variant: Tde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDE` reader - Transmit DMA Enable"]
pub type TdeR = crate::BitReader<Tde>;
impl TdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tde {
        match self.bits {
            false => Tde::B0,
            true => Tde::B1,
        }
    }
    #[doc = "Transmit DMA disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tde::B0
    }
    #[doc = "Transmit DMA enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tde::B1
    }
}
#[doc = "Field `TDE` writer - Transmit DMA Enable"]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG, Tde>;
impl<'a, REG> TdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit DMA disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::B0)
    }
    #[doc = "Transmit DMA enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tde::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive DMA Enable"]
    #[inline(always)]
    pub fn rde(&self) -> RdeR {
        RdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rde(&mut self) -> RdeW<SpiDmacrSpec> {
        RdeW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TdeW<SpiDmacrSpec> {
        TdeW::new(self, 1)
    }
}
#[doc = "DMA Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiDmacrSpec;
impl crate::RegisterSpec for SpiDmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_dmacr::R`](R) reader structure"]
impl crate::Readable for SpiDmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_dmacr::W`](W) writer structure"]
impl crate::Writable for SpiDmacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_DMACR to value 0"]
impl crate::Resettable for SpiDmacrSpec {
    const RESET_VALUE: u32 = 0;
}

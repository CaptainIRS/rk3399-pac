#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DmacrSpec>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DmacrSpec>;
#[doc = "Field `TDL` reader - Transmit Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe transmit logic. It is equal to the watermark level; that is, the\n\ndma_tx_req signal is generated when the number of valid data\n\nentries in the TXFIFO(TXFIFO0 if TCSR=00;TXFIFO1 if\n\nTCSR=01,TXFIFO2 if TCSR=10,TXFIFO3 if TCSR=11)is equal to\n\nor below this field value."]
pub type TdlR = crate::FieldReader;
#[doc = "Field `TDL` writer - Transmit Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe transmit logic. It is equal to the watermark level; that is, the\n\ndma_tx_req signal is generated when the number of valid data\n\nentries in the TXFIFO(TXFIFO0 if TCSR=00;TXFIFO1 if\n\nTCSR=01,TXFIFO2 if TCSR=10,TXFIFO3 if TCSR=11)is equal to\n\nor below this field value."]
pub type TdlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
#[doc = "Field `RDL` reader - Receive Data Level\n\nThis bit field controls the level at which a DMA request is made\n\nby the receive logic. The watermark level = DMARDL+1; that is,\n\ndma_rx_req is generated when the number of valid data entries\n\nin the receive FIFO (RXFIFO0 if RCSR=00;RXFIFO1 if\n\nRCSR=01,RXFIFO2 if RCSR=10,RXFIFO3 if RCSR=11)is equal to\n\nor above this field value + 1."]
pub type RdlR = crate::FieldReader;
#[doc = "Field `RDL` writer - Receive Data Level\n\nThis bit field controls the level at which a DMA request is made\n\nby the receive logic. The watermark level = DMARDL+1; that is,\n\ndma_rx_req is generated when the number of valid data entries\n\nin the receive FIFO (RXFIFO0 if RCSR=00;RXFIFO1 if\n\nRCSR=01,RXFIFO2 if RCSR=10,RXFIFO3 if RCSR=11)is equal to\n\nor above this field value + 1."]
pub type RdlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
impl R {
    #[doc = "Bits 0:4 - Transmit Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe transmit logic. It is equal to the watermark level; that is, the\n\ndma_tx_req signal is generated when the number of valid data\n\nentries in the TXFIFO(TXFIFO0 if TCSR=00;TXFIFO1 if\n\nTCSR=01,TXFIFO2 if TCSR=10,TXFIFO3 if TCSR=11)is equal to\n\nor below this field value."]
    #[inline(always)]
    pub fn tdl(&self) -> TdlR {
        TdlR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Receive Data Level\n\nThis bit field controls the level at which a DMA request is made\n\nby the receive logic. The watermark level = DMARDL+1; that is,\n\ndma_rx_req is generated when the number of valid data entries\n\nin the receive FIFO (RXFIFO0 if RCSR=00;RXFIFO1 if\n\nRCSR=01,RXFIFO2 if RCSR=10,RXFIFO3 if RCSR=11)is equal to\n\nor above this field value + 1."]
    #[inline(always)]
    pub fn rdl(&self) -> RdlR {
        RdlR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Receive DMA Enable"]
    #[inline(always)]
    pub fn rde(&self) -> RdeR {
        RdeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe transmit logic. It is equal to the watermark level; that is, the\n\ndma_tx_req signal is generated when the number of valid data\n\nentries in the TXFIFO(TXFIFO0 if TCSR=00;TXFIFO1 if\n\nTCSR=01,TXFIFO2 if TCSR=10,TXFIFO3 if TCSR=11)is equal to\n\nor below this field value."]
    #[inline(always)]
    #[must_use]
    pub fn tdl(&mut self) -> TdlW<DmacrSpec> {
        TdlW::new(self, 0)
    }
    #[doc = "Bit 8 - Transmit DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TdeW<DmacrSpec> {
        TdeW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Receive Data Level\n\nThis bit field controls the level at which a DMA request is made\n\nby the receive logic. The watermark level = DMARDL+1; that is,\n\ndma_rx_req is generated when the number of valid data entries\n\nin the receive FIFO (RXFIFO0 if RCSR=00;RXFIFO1 if\n\nRCSR=01,RXFIFO2 if RCSR=10,RXFIFO3 if RCSR=11)is equal to\n\nor above this field value + 1."]
    #[inline(always)]
    #[must_use]
    pub fn rdl(&mut self) -> RdlW<DmacrSpec> {
        RdlW::new(self, 16)
    }
    #[doc = "Bit 24 - Receive DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rde(&mut self) -> RdeW<DmacrSpec> {
        RdeW::new(self, 24)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrSpec;
impl crate::RegisterSpec for DmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DmacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACR to value 0x001f_0000"]
impl crate::Resettable for DmacrSpec {
    const RESET_VALUE: u32 = 0x001f_0000;
}

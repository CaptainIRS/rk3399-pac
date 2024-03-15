#[doc = "Register `SPDIF_DMACR` reader"]
pub type R = crate::R<SpdifDmacrSpec>;
#[doc = "Register `SPDIF_DMACR` writer"]
pub type W = crate::W<SpdifDmacrSpec>;
#[doc = "Field `TDL` reader - Transmit Data Level This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the Sample Date Buffer is equal to or below this field value"]
pub type TdlR = crate::FieldReader;
#[doc = "Field `TDL` writer - Transmit Data Level This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the Sample Date Buffer is equal to or below this field value"]
pub type TdlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Transmit DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tde {
    #[doc = "0: Transmit DMA enabled"]
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
    #[doc = "Transmit DMA enabled"]
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
    #[doc = "Transmit DMA enabled"]
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
    #[doc = "Bits 0:4 - Transmit Data Level This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the Sample Date Buffer is equal to or below this field value"]
    #[inline(always)]
    pub fn tdl(&self) -> TdlR {
        TdlR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit Data Level This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the Sample Date Buffer is equal to or below this field value"]
    #[inline(always)]
    #[must_use]
    pub fn tdl(&mut self) -> TdlW<SpdifDmacrSpec> {
        TdlW::new(self, 0)
    }
    #[doc = "Bit 5 - Transmit DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TdeW<SpdifDmacrSpec> {
        TdeW::new(self, 5)
    }
}
#[doc = "DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifDmacrSpec;
impl crate::RegisterSpec for SpdifDmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_dmacr::R`](R) reader structure"]
impl crate::Readable for SpdifDmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdif_dmacr::W`](W) writer structure"]
impl crate::Writable for SpdifDmacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIF_DMACR to value 0"]
impl crate::Resettable for SpdifDmacrSpec {
    const RESET_VALUE: u32 = 0;
}

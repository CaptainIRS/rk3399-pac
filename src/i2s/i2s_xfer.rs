#[doc = "Register `I2S_XFER` reader"]
pub type R = crate::R<I2sXferSpec>;
#[doc = "Register `I2S_XFER` writer"]
pub type W = crate::W<I2sXferSpec>;
#[doc = "TX Transfer start bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txs {
    #[doc = "0: stop TX transfer."]
    B0 = 0,
    #[doc = "1: start TX transfer"]
    B1 = 1,
}
impl From<Txs> for bool {
    #[inline(always)]
    fn from(variant: Txs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXS` reader - TX Transfer start bit"]
pub type TxsR = crate::BitReader<Txs>;
impl TxsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txs {
        match self.bits {
            false => Txs::B0,
            true => Txs::B1,
        }
    }
    #[doc = "stop TX transfer."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txs::B0
    }
    #[doc = "start TX transfer"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txs::B1
    }
}
#[doc = "Field `TXS` writer - TX Transfer start bit"]
pub type TxsW<'a, REG> = crate::BitWriter<'a, REG, Txs>;
impl<'a, REG> TxsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stop TX transfer."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txs::B0)
    }
    #[doc = "start TX transfer"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txs::B1)
    }
}
#[doc = "RX Transfer start bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxs {
    #[doc = "0: stop RX transfer."]
    B0 = 0,
    #[doc = "1: start RX transfer"]
    B1 = 1,
}
impl From<Rxs> for bool {
    #[inline(always)]
    fn from(variant: Rxs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXS` reader - RX Transfer start bit"]
pub type RxsR = crate::BitReader<Rxs>;
impl RxsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxs {
        match self.bits {
            false => Rxs::B0,
            true => Rxs::B1,
        }
    }
    #[doc = "stop RX transfer."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxs::B0
    }
    #[doc = "start RX transfer"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxs::B1
    }
}
#[doc = "Field `RXS` writer - RX Transfer start bit"]
pub type RxsW<'a, REG> = crate::BitWriter<'a, REG, Rxs>;
impl<'a, REG> RxsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stop RX transfer."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxs::B0)
    }
    #[doc = "start RX transfer"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxs::B1)
    }
}
impl R {
    #[doc = "Bit 0 - TX Transfer start bit"]
    #[inline(always)]
    pub fn txs(&self) -> TxsR {
        TxsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX Transfer start bit"]
    #[inline(always)]
    pub fn rxs(&self) -> RxsR {
        RxsR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Transfer start bit"]
    #[inline(always)]
    #[must_use]
    pub fn txs(&mut self) -> TxsW<I2sXferSpec> {
        TxsW::new(self, 0)
    }
    #[doc = "Bit 1 - RX Transfer start bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RxsW<I2sXferSpec> {
        RxsW::new(self, 1)
    }
}
#[doc = "Transfer Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_xfer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_xfer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sXferSpec;
impl crate::RegisterSpec for I2sXferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_xfer::R`](R) reader structure"]
impl crate::Readable for I2sXferSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_xfer::W`](W) writer structure"]
impl crate::Writable for I2sXferSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2S_XFER to value 0"]
impl crate::Resettable for I2sXferSpec {
    const RESET_VALUE: u32 = 0;
}

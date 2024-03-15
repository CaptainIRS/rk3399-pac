#[doc = "Register `TX_INSTUFFING` reader"]
pub type R = crate::R<TxInstuffingSpec>;
#[doc = "Register `TX_INSTUFFING` writer"]
pub type W = crate::W<TxInstuffingSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GydataStuffing {
    #[doc = "0: When the dataen signal is low, the value in the gydata\\[15:0\\]
output is given by the values in TX_GYDTA0 and TX_GYDATA1 registers."]
    B0 = 0,
    #[doc = "1: When the dataen signal is low, the value in the gydata\\[15:0\\]
output is given by the values in TX_GYDTA0 and TX_GYDATA1 registers."]
    B1 = 1,
}
impl From<GydataStuffing> for bool {
    #[inline(always)]
    fn from(variant: GydataStuffing) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GYDATA_STUFFING` reader - "]
pub type GydataStuffingR = crate::BitReader<GydataStuffing>;
impl GydataStuffingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GydataStuffing {
        match self.bits {
            false => GydataStuffing::B0,
            true => GydataStuffing::B1,
        }
    }
    #[doc = "When the dataen signal is low, the value in the gydata\\[15:0\\]
output is given by the values in TX_GYDTA0 and TX_GYDATA1 registers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GydataStuffing::B0
    }
    #[doc = "When the dataen signal is low, the value in the gydata\\[15:0\\]
output is given by the values in TX_GYDTA0 and TX_GYDATA1 registers."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GydataStuffing::B1
    }
}
#[doc = "Field `GYDATA_STUFFING` writer - "]
pub type GydataStuffingW<'a, REG> = crate::BitWriter<'a, REG, GydataStuffing>;
impl<'a, REG> GydataStuffingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When the dataen signal is low, the value in the gydata\\[15:0\\]
output is given by the values in TX_GYDTA0 and TX_GYDATA1 registers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GydataStuffing::B0)
    }
    #[doc = "When the dataen signal is low, the value in the gydata\\[15:0\\]
output is given by the values in TX_GYDTA0 and TX_GYDATA1 registers."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GydataStuffing::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RcrdataStuffing {
    #[doc = "0: When the dataen signal is low, the value in the rcrdata\\[15:0\\]
output is given by the values in TX_RCRDTA0 and TX_RCRDATA1 registers."]
    B0 = 0,
    #[doc = "1: When the dataen signal is low, the value in the rcrdata\\[15:0\\]
output is given by the values in TX_RCRDTA0 and TX_RCRDATA1 registers."]
    B1 = 1,
}
impl From<RcrdataStuffing> for bool {
    #[inline(always)]
    fn from(variant: RcrdataStuffing) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCRDATA_STUFFING` reader - "]
pub type RcrdataStuffingR = crate::BitReader<RcrdataStuffing>;
impl RcrdataStuffingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RcrdataStuffing {
        match self.bits {
            false => RcrdataStuffing::B0,
            true => RcrdataStuffing::B1,
        }
    }
    #[doc = "When the dataen signal is low, the value in the rcrdata\\[15:0\\]
output is given by the values in TX_RCRDTA0 and TX_RCRDATA1 registers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RcrdataStuffing::B0
    }
    #[doc = "When the dataen signal is low, the value in the rcrdata\\[15:0\\]
output is given by the values in TX_RCRDTA0 and TX_RCRDATA1 registers."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RcrdataStuffing::B1
    }
}
#[doc = "Field `RCRDATA_STUFFING` writer - "]
pub type RcrdataStuffingW<'a, REG> = crate::BitWriter<'a, REG, RcrdataStuffing>;
impl<'a, REG> RcrdataStuffingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When the dataen signal is low, the value in the rcrdata\\[15:0\\]
output is given by the values in TX_RCRDTA0 and TX_RCRDATA1 registers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RcrdataStuffing::B0)
    }
    #[doc = "When the dataen signal is low, the value in the rcrdata\\[15:0\\]
output is given by the values in TX_RCRDTA0 and TX_RCRDATA1 registers."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RcrdataStuffing::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcbdataStuffing {
    #[doc = "0: When the dataen signal is low, the value in the bcbdata\\[15:0\\]
output is given by the values in the TX_BCBDTA0 and TX_BCBDATA1 registers."]
    B0 = 0,
    #[doc = "1: When the dataen signal is low, the value in the bcbdata\\[15:0\\]
output is given by the values in the TX_BCBDTA0 and TX_BCBDATA1 registers."]
    B1 = 1,
}
impl From<BcbdataStuffing> for bool {
    #[inline(always)]
    fn from(variant: BcbdataStuffing) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCBDATA_STUFFING` reader - "]
pub type BcbdataStuffingR = crate::BitReader<BcbdataStuffing>;
impl BcbdataStuffingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BcbdataStuffing {
        match self.bits {
            false => BcbdataStuffing::B0,
            true => BcbdataStuffing::B1,
        }
    }
    #[doc = "When the dataen signal is low, the value in the bcbdata\\[15:0\\]
output is given by the values in the TX_BCBDTA0 and TX_BCBDATA1 registers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BcbdataStuffing::B0
    }
    #[doc = "When the dataen signal is low, the value in the bcbdata\\[15:0\\]
output is given by the values in the TX_BCBDTA0 and TX_BCBDATA1 registers."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BcbdataStuffing::B1
    }
}
#[doc = "Field `BCBDATA_STUFFING` writer - "]
pub type BcbdataStuffingW<'a, REG> = crate::BitWriter<'a, REG, BcbdataStuffing>;
impl<'a, REG> BcbdataStuffingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When the dataen signal is low, the value in the bcbdata\\[15:0\\]
output is given by the values in the TX_BCBDTA0 and TX_BCBDATA1 registers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BcbdataStuffing::B0)
    }
    #[doc = "When the dataen signal is low, the value in the bcbdata\\[15:0\\]
output is given by the values in the TX_BCBDTA0 and TX_BCBDATA1 registers."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BcbdataStuffing::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gydata_stuffing(&self) -> GydataStuffingR {
        GydataStuffingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rcrdata_stuffing(&self) -> RcrdataStuffingR {
        RcrdataStuffingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bcbdata_stuffing(&self) -> BcbdataStuffingR {
        BcbdataStuffingR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gydata_stuffing(&mut self) -> GydataStuffingW<TxInstuffingSpec> {
        GydataStuffingW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rcrdata_stuffing(&mut self) -> RcrdataStuffingW<TxInstuffingSpec> {
        RcrdataStuffingW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn bcbdata_stuffing(&mut self) -> BcbdataStuffingW<TxInstuffingSpec> {
        BcbdataStuffingW::new(self, 2)
    }
}
#[doc = "0b: When the dataen signal is low, the value in the gydata\\[15:0\\]
output is the one sampled from the corresponding input data. 1b: When the dataen signal is low, the value in the gydata\\[15:0\\]
output is given by the values in TX_GYDTA0 and TX_GYDATA1 registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_instuffing::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_instuffing::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxInstuffingSpec;
impl crate::RegisterSpec for TxInstuffingSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_instuffing::R`](R) reader structure"]
impl crate::Readable for TxInstuffingSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_instuffing::W`](W) writer structure"]
impl crate::Writable for TxInstuffingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TX_INSTUFFING to value 0"]
impl crate::Resettable for TxInstuffingSpec {
    const RESET_VALUE: u8 = 0;
}

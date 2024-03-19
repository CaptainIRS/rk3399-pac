#[doc = "Register `TX_COMMON2` reader"]
pub type R = crate::R<TxCommon2Spec>;
#[doc = "Register `TX_COMMON2` writer"]
pub type W = crate::W<TxCommon2Spec>;
#[doc = "TX data Patten\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxDataPatten {
    #[doc = "0: all zero"]
    B000 = 0,
    #[doc = "1: all one"]
    B001 = 1,
    #[doc = "2: D10.2"]
    B010 = 2,
    #[doc = "3: 1100"]
    B011 = 3,
    #[doc = "4: K28.5"]
    B100 = 4,
    #[doc = "5: K28.7"]
    B101 = 5,
    #[doc = "6: 1111100000"]
    B110 = 6,
    #[doc = "7: 11111111110000000000"]
    B111 = 7,
}
impl From<TxDataPatten> for u8 {
    #[inline(always)]
    fn from(variant: TxDataPatten) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxDataPatten {
    type Ux = u8;
}
#[doc = "Field `TX_DATA_PATTEN` reader - TX data Patten"]
pub type TxDataPattenR = crate::FieldReader<TxDataPatten>;
impl TxDataPattenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxDataPatten {
        match self.bits {
            0 => TxDataPatten::B000,
            1 => TxDataPatten::B001,
            2 => TxDataPatten::B010,
            3 => TxDataPatten::B011,
            4 => TxDataPatten::B100,
            5 => TxDataPatten::B101,
            6 => TxDataPatten::B110,
            7 => TxDataPatten::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "all zero"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == TxDataPatten::B000
    }
    #[doc = "all one"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == TxDataPatten::B001
    }
    #[doc = "D10.2"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == TxDataPatten::B010
    }
    #[doc = "1100"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == TxDataPatten::B011
    }
    #[doc = "K28.5"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == TxDataPatten::B100
    }
    #[doc = "K28.7"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == TxDataPatten::B101
    }
    #[doc = "1111100000"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == TxDataPatten::B110
    }
    #[doc = "11111111110000000000"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == TxDataPatten::B111
    }
}
#[doc = "Field `TX_DATA_PATTEN` writer - TX data Patten"]
pub type TxDataPattenW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TxDataPatten>;
impl<'a, REG> TxDataPattenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "all zero"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(TxDataPatten::B000)
    }
    #[doc = "all one"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(TxDataPatten::B001)
    }
    #[doc = "D10.2"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(TxDataPatten::B010)
    }
    #[doc = "1100"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(TxDataPatten::B011)
    }
    #[doc = "K28.5"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(TxDataPatten::B100)
    }
    #[doc = "K28.7"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(TxDataPatten::B101)
    }
    #[doc = "1111100000"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(TxDataPatten::B110)
    }
    #[doc = "11111111110000000000"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(TxDataPatten::B111)
    }
}
#[doc = "TX output pattern enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxOutPatternEn {
    #[doc = "0: normal TX"]
    B0 = 0,
    #[doc = "1: dedicate pattern"]
    B1 = 1,
}
impl From<TxOutPatternEn> for bool {
    #[inline(always)]
    fn from(variant: TxOutPatternEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_OUT_PATTERN_EN` reader - TX output pattern enable"]
pub type TxOutPatternEnR = crate::BitReader<TxOutPatternEn>;
impl TxOutPatternEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxOutPatternEn {
        match self.bits {
            false => TxOutPatternEn::B0,
            true => TxOutPatternEn::B1,
        }
    }
    #[doc = "normal TX"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TxOutPatternEn::B0
    }
    #[doc = "dedicate pattern"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TxOutPatternEn::B1
    }
}
#[doc = "Field `TX_OUT_PATTERN_EN` writer - TX output pattern enable"]
pub type TxOutPatternEnW<'a, REG> = crate::BitWriter<'a, REG, TxOutPatternEn>;
impl<'a, REG> TxOutPatternEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal TX"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutPatternEn::B0)
    }
    #[doc = "dedicate pattern"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutPatternEn::B1)
    }
}
#[doc = "TX ch0 output p-n inverse control:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxOutputPnInverseCh0 {
    #[doc = "0: not inverse"]
    B0 = 0,
    #[doc = "1: output p and n inverse"]
    B1 = 1,
}
impl From<TxOutputPnInverseCh0> for bool {
    #[inline(always)]
    fn from(variant: TxOutputPnInverseCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_OUTPUT_PN_INVERSE_CH0` reader - TX ch0 output p-n inverse control:"]
pub type TxOutputPnInverseCh0R = crate::BitReader<TxOutputPnInverseCh0>;
impl TxOutputPnInverseCh0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxOutputPnInverseCh0 {
        match self.bits {
            false => TxOutputPnInverseCh0::B0,
            true => TxOutputPnInverseCh0::B1,
        }
    }
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TxOutputPnInverseCh0::B0
    }
    #[doc = "output p and n inverse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TxOutputPnInverseCh0::B1
    }
}
#[doc = "Field `TX_OUTPUT_PN_INVERSE_CH0` writer - TX ch0 output p-n inverse control:"]
pub type TxOutputPnInverseCh0W<'a, REG> = crate::BitWriter1C<'a, REG, TxOutputPnInverseCh0>;
impl<'a, REG> TxOutputPnInverseCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutputPnInverseCh0::B0)
    }
    #[doc = "output p and n inverse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutputPnInverseCh0::B1)
    }
}
#[doc = "TX ch1 output p-n inverse control:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxOutputPnInverseCh1 {
    #[doc = "0: not inverse"]
    B0 = 0,
    #[doc = "1: output p and n inverse"]
    B1 = 1,
}
impl From<TxOutputPnInverseCh1> for bool {
    #[inline(always)]
    fn from(variant: TxOutputPnInverseCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_OUTPUT_PN_INVERSE_CH1` reader - TX ch1 output p-n inverse control:"]
pub type TxOutputPnInverseCh1R = crate::BitReader<TxOutputPnInverseCh1>;
impl TxOutputPnInverseCh1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxOutputPnInverseCh1 {
        match self.bits {
            false => TxOutputPnInverseCh1::B0,
            true => TxOutputPnInverseCh1::B1,
        }
    }
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TxOutputPnInverseCh1::B0
    }
    #[doc = "output p and n inverse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TxOutputPnInverseCh1::B1
    }
}
#[doc = "Field `TX_OUTPUT_PN_INVERSE_CH1` writer - TX ch1 output p-n inverse control:"]
pub type TxOutputPnInverseCh1W<'a, REG> = crate::BitWriter1C<'a, REG, TxOutputPnInverseCh1>;
impl<'a, REG> TxOutputPnInverseCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutputPnInverseCh1::B0)
    }
    #[doc = "output p and n inverse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutputPnInverseCh1::B1)
    }
}
#[doc = "TX ch2 output p-n inverse control:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxOutputPnInverseCh2 {
    #[doc = "0: not inverse"]
    B0 = 0,
    #[doc = "1: output p and n inverse"]
    B1 = 1,
}
impl From<TxOutputPnInverseCh2> for bool {
    #[inline(always)]
    fn from(variant: TxOutputPnInverseCh2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_OUTPUT_PN_INVERSE_CH2` reader - TX ch2 output p-n inverse control:"]
pub type TxOutputPnInverseCh2R = crate::BitReader<TxOutputPnInverseCh2>;
impl TxOutputPnInverseCh2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxOutputPnInverseCh2 {
        match self.bits {
            false => TxOutputPnInverseCh2::B0,
            true => TxOutputPnInverseCh2::B1,
        }
    }
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TxOutputPnInverseCh2::B0
    }
    #[doc = "output p and n inverse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TxOutputPnInverseCh2::B1
    }
}
#[doc = "Field `TX_OUTPUT_PN_INVERSE_CH2` writer - TX ch2 output p-n inverse control:"]
pub type TxOutputPnInverseCh2W<'a, REG> = crate::BitWriter1C<'a, REG, TxOutputPnInverseCh2>;
impl<'a, REG> TxOutputPnInverseCh2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutputPnInverseCh2::B0)
    }
    #[doc = "output p and n inverse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutputPnInverseCh2::B1)
    }
}
#[doc = "TX ch3 output p-n inverse control:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxOutputPnInverseCh3 {
    #[doc = "0: not inverse"]
    B0 = 0,
    #[doc = "1: output p and n inverse"]
    B1 = 1,
}
impl From<TxOutputPnInverseCh3> for bool {
    #[inline(always)]
    fn from(variant: TxOutputPnInverseCh3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_OUTPUT_PN_INVERSE_CH3` reader - TX ch3 output p-n inverse control:"]
pub type TxOutputPnInverseCh3R = crate::BitReader<TxOutputPnInverseCh3>;
impl TxOutputPnInverseCh3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxOutputPnInverseCh3 {
        match self.bits {
            false => TxOutputPnInverseCh3::B0,
            true => TxOutputPnInverseCh3::B1,
        }
    }
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TxOutputPnInverseCh3::B0
    }
    #[doc = "output p and n inverse"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TxOutputPnInverseCh3::B1
    }
}
#[doc = "Field `TX_OUTPUT_PN_INVERSE_CH3` writer - TX ch3 output p-n inverse control:"]
pub type TxOutputPnInverseCh3W<'a, REG> = crate::BitWriter1C<'a, REG, TxOutputPnInverseCh3>;
impl<'a, REG> TxOutputPnInverseCh3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not inverse"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutputPnInverseCh3::B0)
    }
    #[doc = "output p and n inverse"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TxOutputPnInverseCh3::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - TX data Patten"]
    #[inline(always)]
    pub fn tx_data_patten(&self) -> TxDataPattenR {
        TxDataPattenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - TX output pattern enable"]
    #[inline(always)]
    pub fn tx_out_pattern_en(&self) -> TxOutPatternEnR {
        TxOutPatternEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX ch0 output p-n inverse control:"]
    #[inline(always)]
    pub fn tx_output_pn_inverse_ch0(&self) -> TxOutputPnInverseCh0R {
        TxOutputPnInverseCh0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX ch1 output p-n inverse control:"]
    #[inline(always)]
    pub fn tx_output_pn_inverse_ch1(&self) -> TxOutputPnInverseCh1R {
        TxOutputPnInverseCh1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX ch2 output p-n inverse control:"]
    #[inline(always)]
    pub fn tx_output_pn_inverse_ch2(&self) -> TxOutputPnInverseCh2R {
        TxOutputPnInverseCh2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX ch3 output p-n inverse control:"]
    #[inline(always)]
    pub fn tx_output_pn_inverse_ch3(&self) -> TxOutputPnInverseCh3R {
        TxOutputPnInverseCh3R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TX data Patten"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_patten(&mut self) -> TxDataPattenW<TxCommon2Spec> {
        TxDataPattenW::new(self, 0)
    }
    #[doc = "Bit 3 - TX output pattern enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_out_pattern_en(&mut self) -> TxOutPatternEnW<TxCommon2Spec> {
        TxOutPatternEnW::new(self, 3)
    }
    #[doc = "Bit 4 - TX ch0 output p-n inverse control:"]
    #[inline(always)]
    #[must_use]
    pub fn tx_output_pn_inverse_ch0(&mut self) -> TxOutputPnInverseCh0W<TxCommon2Spec> {
        TxOutputPnInverseCh0W::new(self, 4)
    }
    #[doc = "Bit 5 - TX ch1 output p-n inverse control:"]
    #[inline(always)]
    #[must_use]
    pub fn tx_output_pn_inverse_ch1(&mut self) -> TxOutputPnInverseCh1W<TxCommon2Spec> {
        TxOutputPnInverseCh1W::new(self, 5)
    }
    #[doc = "Bit 6 - TX ch2 output p-n inverse control:"]
    #[inline(always)]
    #[must_use]
    pub fn tx_output_pn_inverse_ch2(&mut self) -> TxOutputPnInverseCh2W<TxCommon2Spec> {
        TxOutputPnInverseCh2W::new(self, 6)
    }
    #[doc = "Bit 7 - TX ch3 output p-n inverse control:"]
    #[inline(always)]
    #[must_use]
    pub fn tx_output_pn_inverse_ch3(&mut self) -> TxOutputPnInverseCh3W<TxCommon2Spec> {
        TxOutputPnInverseCh3W::new(self, 7)
    }
}
#[doc = "Tx terminal resistor control2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_common2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_common2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCommon2Spec;
impl crate::RegisterSpec for TxCommon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_common2::R`](R) reader structure"]
impl crate::Readable for TxCommon2Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_common2::W`](W) writer structure"]
impl crate::Writable for TxCommon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf0;
}
#[doc = "`reset()` method sets TX_COMMON2 to value 0x50"]
impl crate::Resettable for TxCommon2Spec {
    const RESET_VALUE: u32 = 0x50;
}

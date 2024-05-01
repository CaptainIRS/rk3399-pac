#[doc = "Register `TDES_CTRL` reader"]
pub type R = crate::R<TdesCtrlSpec>;
#[doc = "Register `TDES_CTRL` writer"]
pub type W = crate::W<TdesCtrlSpec>;
#[doc = "Specifies the Encryption/ Decryption mode selection signal\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdesEnc {
    #[doc = "0: Encryption"]
    B0 = 0,
    #[doc = "1: Decryption"]
    B1 = 1,
}
impl From<TdesEnc> for bool {
    #[inline(always)]
    fn from(variant: TdesEnc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDES_ENC` reader - Specifies the Encryption/ Decryption mode selection signal"]
pub type TdesEncR = crate::BitReader<TdesEnc>;
impl TdesEncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdesEnc {
        match self.bits {
            false => TdesEnc::B0,
            true => TdesEnc::B1,
        }
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TdesEnc::B0
    }
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TdesEnc::B1
    }
}
#[doc = "Field `TDES_ENC` writer - Specifies the Encryption/ Decryption mode selection signal"]
pub type TdesEncW<'a, REG> = crate::BitWriter<'a, REG, TdesEnc>;
impl<'a, REG> TdesEncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TdesEnc::B0)
    }
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TdesEnc::B1)
    }
}
#[doc = "Specify TDES Fifo Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdesFifomode {
    #[doc = "0: Slave mode"]
    B0 = 0,
    #[doc = "1: Fifo mode"]
    B1 = 1,
}
impl From<TdesFifomode> for bool {
    #[inline(always)]
    fn from(variant: TdesFifomode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDES_FIFOMODE` reader - Specify TDES Fifo Mode"]
pub type TdesFifomodeR = crate::BitReader<TdesFifomode>;
impl TdesFifomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdesFifomode {
        match self.bits {
            false => TdesFifomode::B0,
            true => TdesFifomode::B1,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TdesFifomode::B0
    }
    #[doc = "Fifo mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TdesFifomode::B1
    }
}
#[doc = "Field `TDES_FIFOMODE` writer - Specify TDES Fifo Mode"]
pub type TdesFifomodeW<'a, REG> = crate::BitWriter<'a, REG, TdesFifomode>;
impl<'a, REG> TdesFifomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TdesFifomode::B0)
    }
    #[doc = "Fifo mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TdesFifomode::B1)
    }
}
#[doc = "Specify DES or TDES cipher\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdesSelect {
    #[doc = "0: DES"]
    B0 = 0,
    #[doc = "1: TDES"]
    B1 = 1,
}
impl From<TdesSelect> for bool {
    #[inline(always)]
    fn from(variant: TdesSelect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDES_SELECT` reader - Specify DES or TDES cipher"]
pub type TdesSelectR = crate::BitReader<TdesSelect>;
impl TdesSelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdesSelect {
        match self.bits {
            false => TdesSelect::B0,
            true => TdesSelect::B1,
        }
    }
    #[doc = "DES"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TdesSelect::B0
    }
    #[doc = "TDES"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TdesSelect::B1
    }
}
#[doc = "Field `TDES_SELECT` writer - Specify DES or TDES cipher"]
pub type TdesSelectW<'a, REG> = crate::BitWriter<'a, REG, TdesSelect>;
impl<'a, REG> TdesSelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DES"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TdesSelect::B0)
    }
    #[doc = "TDES"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TdesSelect::B1)
    }
}
#[doc = "Specifies the TDES key mode selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdesEee {
    #[doc = "0: EDE"]
    B0 = 0,
    #[doc = "1: EEE"]
    B1 = 1,
}
impl From<TdesEee> for bool {
    #[inline(always)]
    fn from(variant: TdesEee) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDES_EEE` reader - Specifies the TDES key mode selection"]
pub type TdesEeeR = crate::BitReader<TdesEee>;
impl TdesEeeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdesEee {
        match self.bits {
            false => TdesEee::B0,
            true => TdesEee::B1,
        }
    }
    #[doc = "EDE"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TdesEee::B0
    }
    #[doc = "EEE"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TdesEee::B1
    }
}
#[doc = "Field `TDES_EEE` writer - Specifies the TDES key mode selection"]
pub type TdesEeeW<'a, REG> = crate::BitWriter<'a, REG, TdesEee>;
impl<'a, REG> TdesEeeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EDE"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TdesEee::B0)
    }
    #[doc = "EEE"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TdesEee::B1)
    }
}
#[doc = "Specifies TDES chain mode selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdesChainmode {
    #[doc = "0: ECB mode"]
    B0 = 0,
    #[doc = "1: CBC mode"]
    B1 = 1,
}
impl From<TdesChainmode> for bool {
    #[inline(always)]
    fn from(variant: TdesChainmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDES_CHAINMODE` reader - Specifies TDES chain mode selection"]
pub type TdesChainmodeR = crate::BitReader<TdesChainmode>;
impl TdesChainmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdesChainmode {
        match self.bits {
            false => TdesChainmode::B0,
            true => TdesChainmode::B1,
        }
    }
    #[doc = "ECB mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TdesChainmode::B0
    }
    #[doc = "CBC mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TdesChainmode::B1
    }
}
#[doc = "Field `TDES_CHAINMODE` writer - Specifies TDES chain mode selection"]
pub type TdesChainmodeW<'a, REG> = crate::BitWriter<'a, REG, TdesChainmode>;
impl<'a, REG> TdesChainmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECB mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TdesChainmode::B0)
    }
    #[doc = "CBC mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TdesChainmode::B1)
    }
}
#[doc = "Field `TDES_BYTESWAP_DI` reader - 0 = Disables Input data byte swap\n\n1 = Enables Input data byte swap"]
pub type TdesByteswapDiR = crate::BitReader;
#[doc = "Field `TDES_BYTESWAP_DI` writer - 0 = Disables Input data byte swap\n\n1 = Enables Input data byte swap"]
pub type TdesByteswapDiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDES_BYTESWAP_DO` reader - 0 = Disables Output data byte swap\n\n1 = Enables Output data byte swap"]
pub type TdesByteswapDoR = crate::BitReader;
#[doc = "Field `TDES_BYTESWAP_DO` writer - 0 = Disables Output data byte swap\n\n1 = Enables Output data byte swap"]
pub type TdesByteswapDoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDES_BYTESWAP_IV` reader - 0 = Disables Initial value byte swap\n\n1 = Enables Initial value byte swap"]
pub type TdesByteswapIvR = crate::BitReader;
#[doc = "Field `TDES_BYTESWAP_IV` writer - 0 = Disables Initial value byte swap\n\n1 = Enables Initial value byte swap"]
pub type TdesByteswapIvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDES_BYTESWAP_KEY` reader - 0 = Disables Key byte swap\n\n1 = Enables Key byte swap"]
pub type TdesByteswapKeyR = crate::BitReader;
#[doc = "Field `TDES_BYTESWAP_KEY` writer - 0 = Disables Key byte swap\n\n1 = Enables Key byte swap"]
pub type TdesByteswapKeyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies the Encryption/ Decryption mode selection signal"]
    #[inline(always)]
    pub fn tdes_enc(&self) -> TdesEncR {
        TdesEncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specify TDES Fifo Mode"]
    #[inline(always)]
    pub fn tdes_fifomode(&self) -> TdesFifomodeR {
        TdesFifomodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Specify DES or TDES cipher"]
    #[inline(always)]
    pub fn tdes_select(&self) -> TdesSelectR {
        TdesSelectR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Specifies the TDES key mode selection"]
    #[inline(always)]
    pub fn tdes_eee(&self) -> TdesEeeR {
        TdesEeeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Specifies TDES chain mode selection"]
    #[inline(always)]
    pub fn tdes_chainmode(&self) -> TdesChainmodeR {
        TdesChainmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0 = Disables Input data byte swap\n\n1 = Enables Input data byte swap"]
    #[inline(always)]
    pub fn tdes_byteswap_di(&self) -> TdesByteswapDiR {
        TdesByteswapDiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0 = Disables Output data byte swap\n\n1 = Enables Output data byte swap"]
    #[inline(always)]
    pub fn tdes_byteswap_do(&self) -> TdesByteswapDoR {
        TdesByteswapDoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0 = Disables Initial value byte swap\n\n1 = Enables Initial value byte swap"]
    #[inline(always)]
    pub fn tdes_byteswap_iv(&self) -> TdesByteswapIvR {
        TdesByteswapIvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 = Disables Key byte swap\n\n1 = Enables Key byte swap"]
    #[inline(always)]
    pub fn tdes_byteswap_key(&self) -> TdesByteswapKeyR {
        TdesByteswapKeyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the Encryption/ Decryption mode selection signal"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_enc(&mut self) -> TdesEncW<TdesCtrlSpec> {
        TdesEncW::new(self, 0)
    }
    #[doc = "Bit 1 - Specify TDES Fifo Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_fifomode(&mut self) -> TdesFifomodeW<TdesCtrlSpec> {
        TdesFifomodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Specify DES or TDES cipher"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_select(&mut self) -> TdesSelectW<TdesCtrlSpec> {
        TdesSelectW::new(self, 2)
    }
    #[doc = "Bit 3 - Specifies the TDES key mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_eee(&mut self) -> TdesEeeW<TdesCtrlSpec> {
        TdesEeeW::new(self, 3)
    }
    #[doc = "Bit 4 - Specifies TDES chain mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_chainmode(&mut self) -> TdesChainmodeW<TdesCtrlSpec> {
        TdesChainmodeW::new(self, 4)
    }
    #[doc = "Bit 5 - 0 = Disables Input data byte swap\n\n1 = Enables Input data byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_byteswap_di(&mut self) -> TdesByteswapDiW<TdesCtrlSpec> {
        TdesByteswapDiW::new(self, 5)
    }
    #[doc = "Bit 6 - 0 = Disables Output data byte swap\n\n1 = Enables Output data byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_byteswap_do(&mut self) -> TdesByteswapDoW<TdesCtrlSpec> {
        TdesByteswapDoW::new(self, 6)
    }
    #[doc = "Bit 7 - 0 = Disables Initial value byte swap\n\n1 = Enables Initial value byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_byteswap_iv(&mut self) -> TdesByteswapIvW<TdesCtrlSpec> {
        TdesByteswapIvW::new(self, 7)
    }
    #[doc = "Bit 8 - 0 = Disables Key byte swap\n\n1 = Enables Key byte swap"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_byteswap_key(&mut self) -> TdesByteswapKeyW<TdesCtrlSpec> {
        TdesByteswapKeyW::new(self, 8)
    }
}
#[doc = "TDES Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesCtrlSpec;
impl crate::RegisterSpec for TdesCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_ctrl::R`](R) reader structure"]
impl crate::Readable for TdesCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tdes_ctrl::W`](W) writer structure"]
impl crate::Writable for TdesCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDES_CTRL to value 0"]
impl crate::Resettable for TdesCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

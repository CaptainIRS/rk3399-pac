#[doc = "Register `TSADC_INT_EN` reader"]
pub type R = crate::R<TsadcIntEnSpec>;
#[doc = "Register `TSADC_INT_EN` writer"]
pub type W = crate::W<TsadcIntEnSpec>;
#[doc = "high temperature interrupt enable for src0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HtIntenSrc0 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<HtIntenSrc0> for bool {
    #[inline(always)]
    fn from(variant: HtIntenSrc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HT_INTEN_SRC0` reader - high temperature interrupt enable for src0"]
pub type HtIntenSrc0R = crate::BitReader<HtIntenSrc0>;
impl HtIntenSrc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HtIntenSrc0 {
        match self.bits {
            false => HtIntenSrc0::B0,
            true => HtIntenSrc0::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HtIntenSrc0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HtIntenSrc0::B1
    }
}
#[doc = "Field `HT_INTEN_SRC0` writer - high temperature interrupt enable for src0"]
pub type HtIntenSrc0W<'a, REG> = crate::BitWriter<'a, REG, HtIntenSrc0>;
impl<'a, REG> HtIntenSrc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HtIntenSrc0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HtIntenSrc0::B1)
    }
}
#[doc = "high temperature interrupt enable for src1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HtIntenSrc1 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<HtIntenSrc1> for bool {
    #[inline(always)]
    fn from(variant: HtIntenSrc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HT_INTEN_SRC1` reader - high temperature interrupt enable for src1"]
pub type HtIntenSrc1R = crate::BitReader<HtIntenSrc1>;
impl HtIntenSrc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HtIntenSrc1 {
        match self.bits {
            false => HtIntenSrc1::B0,
            true => HtIntenSrc1::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HtIntenSrc1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HtIntenSrc1::B1
    }
}
#[doc = "Field `HT_INTEN_SRC1` writer - high temperature interrupt enable for src1"]
pub type HtIntenSrc1W<'a, REG> = crate::BitWriter<'a, REG, HtIntenSrc1>;
impl<'a, REG> HtIntenSrc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HtIntenSrc1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HtIntenSrc1::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tshut2gpioEnSrc0 {
    #[doc = "0: TSHUT output to gpio disabled. TSHUT output will always keep low ."]
    B0 = 0,
    #[doc = "1: TSHUT output works."]
    B1 = 1,
}
impl From<Tshut2gpioEnSrc0> for bool {
    #[inline(always)]
    fn from(variant: Tshut2gpioEnSrc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSHUT_2GPIO_EN_SRC0` reader - "]
pub type Tshut2gpioEnSrc0R = crate::BitReader<Tshut2gpioEnSrc0>;
impl Tshut2gpioEnSrc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tshut2gpioEnSrc0 {
        match self.bits {
            false => Tshut2gpioEnSrc0::B0,
            true => Tshut2gpioEnSrc0::B1,
        }
    }
    #[doc = "TSHUT output to gpio disabled. TSHUT output will always keep low ."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tshut2gpioEnSrc0::B0
    }
    #[doc = "TSHUT output works."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tshut2gpioEnSrc0::B1
    }
}
#[doc = "Field `TSHUT_2GPIO_EN_SRC0` writer - "]
pub type Tshut2gpioEnSrc0W<'a, REG> = crate::BitWriter<'a, REG, Tshut2gpioEnSrc0>;
impl<'a, REG> Tshut2gpioEnSrc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSHUT output to gpio disabled. TSHUT output will always keep low ."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tshut2gpioEnSrc0::B0)
    }
    #[doc = "TSHUT output works."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tshut2gpioEnSrc0::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tshut2gpioEnSrc1 {
    #[doc = "0: TSHUT output to gpio disabled. TSHUT output will always keep low ."]
    B0 = 0,
    #[doc = "1: TSHUT output works."]
    B1 = 1,
}
impl From<Tshut2gpioEnSrc1> for bool {
    #[inline(always)]
    fn from(variant: Tshut2gpioEnSrc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSHUT_2GPIO_EN_SRC1` reader - "]
pub type Tshut2gpioEnSrc1R = crate::BitReader<Tshut2gpioEnSrc1>;
impl Tshut2gpioEnSrc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tshut2gpioEnSrc1 {
        match self.bits {
            false => Tshut2gpioEnSrc1::B0,
            true => Tshut2gpioEnSrc1::B1,
        }
    }
    #[doc = "TSHUT output to gpio disabled. TSHUT output will always keep low ."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tshut2gpioEnSrc1::B0
    }
    #[doc = "TSHUT output works."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tshut2gpioEnSrc1::B1
    }
}
#[doc = "Field `TSHUT_2GPIO_EN_SRC1` writer - "]
pub type Tshut2gpioEnSrc1W<'a, REG> = crate::BitWriter<'a, REG, Tshut2gpioEnSrc1>;
impl<'a, REG> Tshut2gpioEnSrc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSHUT output to gpio disabled. TSHUT output will always keep low ."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tshut2gpioEnSrc1::B0)
    }
    #[doc = "TSHUT output works."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tshut2gpioEnSrc1::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tshut2cruEnSrc0 {
    #[doc = "0: TSHUT output to cru disabled. TSHUT output will always keep low ."]
    B0 = 0,
    #[doc = "1: TSHUT output works."]
    B1 = 1,
}
impl From<Tshut2cruEnSrc0> for bool {
    #[inline(always)]
    fn from(variant: Tshut2cruEnSrc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSHUT_2CRU_EN_SRC0` reader - "]
pub type Tshut2cruEnSrc0R = crate::BitReader<Tshut2cruEnSrc0>;
impl Tshut2cruEnSrc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tshut2cruEnSrc0 {
        match self.bits {
            false => Tshut2cruEnSrc0::B0,
            true => Tshut2cruEnSrc0::B1,
        }
    }
    #[doc = "TSHUT output to cru disabled. TSHUT output will always keep low ."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tshut2cruEnSrc0::B0
    }
    #[doc = "TSHUT output works."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tshut2cruEnSrc0::B1
    }
}
#[doc = "Field `TSHUT_2CRU_EN_SRC0` writer - "]
pub type Tshut2cruEnSrc0W<'a, REG> = crate::BitWriter<'a, REG, Tshut2cruEnSrc0>;
impl<'a, REG> Tshut2cruEnSrc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSHUT output to cru disabled. TSHUT output will always keep low ."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tshut2cruEnSrc0::B0)
    }
    #[doc = "TSHUT output works."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tshut2cruEnSrc0::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tshut2cruEnSrc1 {
    #[doc = "0: TSHUT output to cru disabled. TSHUT output will always keep low ."]
    B0 = 0,
    #[doc = "1: TSHUT output works."]
    B1 = 1,
}
impl From<Tshut2cruEnSrc1> for bool {
    #[inline(always)]
    fn from(variant: Tshut2cruEnSrc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSHUT_2CRU_EN_SRC1` reader - "]
pub type Tshut2cruEnSrc1R = crate::BitReader<Tshut2cruEnSrc1>;
impl Tshut2cruEnSrc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tshut2cruEnSrc1 {
        match self.bits {
            false => Tshut2cruEnSrc1::B0,
            true => Tshut2cruEnSrc1::B1,
        }
    }
    #[doc = "TSHUT output to cru disabled. TSHUT output will always keep low ."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tshut2cruEnSrc1::B0
    }
    #[doc = "TSHUT output works."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tshut2cruEnSrc1::B1
    }
}
#[doc = "Field `TSHUT_2CRU_EN_SRC1` writer - "]
pub type Tshut2cruEnSrc1W<'a, REG> = crate::BitWriter<'a, REG, Tshut2cruEnSrc1>;
impl<'a, REG> Tshut2cruEnSrc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TSHUT output to cru disabled. TSHUT output will always keep low ."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tshut2cruEnSrc1::B0)
    }
    #[doc = "TSHUT output works."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tshut2cruEnSrc1::B1)
    }
}
#[doc = "low temperature interrupt enable for src0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LtIntenSrc0 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<LtIntenSrc0> for bool {
    #[inline(always)]
    fn from(variant: LtIntenSrc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LT_INTEN_SRC0` reader - low temperature interrupt enable for src0"]
pub type LtIntenSrc0R = crate::BitReader<LtIntenSrc0>;
impl LtIntenSrc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LtIntenSrc0 {
        match self.bits {
            false => LtIntenSrc0::B0,
            true => LtIntenSrc0::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LtIntenSrc0::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LtIntenSrc0::B1
    }
}
#[doc = "Field `LT_INTEN_SRC0` writer - low temperature interrupt enable for src0"]
pub type LtIntenSrc0W<'a, REG> = crate::BitWriter<'a, REG, LtIntenSrc0>;
impl<'a, REG> LtIntenSrc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LtIntenSrc0::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LtIntenSrc0::B1)
    }
}
#[doc = "low temperature interrupt enable for src1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LtIntenSrc1 {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<LtIntenSrc1> for bool {
    #[inline(always)]
    fn from(variant: LtIntenSrc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LT_INTEN_SRC1` reader - low temperature interrupt enable for src1"]
pub type LtIntenSrc1R = crate::BitReader<LtIntenSrc1>;
impl LtIntenSrc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LtIntenSrc1 {
        match self.bits {
            false => LtIntenSrc1::B0,
            true => LtIntenSrc1::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LtIntenSrc1::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LtIntenSrc1::B1
    }
}
#[doc = "Field `LT_INTEN_SRC1` writer - low temperature interrupt enable for src1"]
pub type LtIntenSrc1W<'a, REG> = crate::BitWriter<'a, REG, LtIntenSrc1>;
impl<'a, REG> LtIntenSrc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LtIntenSrc1::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LtIntenSrc1::B1)
    }
}
#[doc = "eoc_Interrupt enable.\n\neoc_interrupt enable in user defined mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EocIntEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<EocIntEn> for bool {
    #[inline(always)]
    fn from(variant: EocIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC_INT_EN` reader - eoc_Interrupt enable.\n\neoc_interrupt enable in user defined mode"]
pub type EocIntEnR = crate::BitReader<EocIntEn>;
impl EocIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EocIntEn {
        match self.bits {
            false => EocIntEn::B0,
            true => EocIntEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == EocIntEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == EocIntEn::B1
    }
}
#[doc = "Field `EOC_INT_EN` writer - eoc_Interrupt enable.\n\neoc_interrupt enable in user defined mode"]
pub type EocIntEnW<'a, REG> = crate::BitWriter<'a, REG, EocIntEn>;
impl<'a, REG> EocIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(EocIntEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(EocIntEn::B1)
    }
}
impl R {
    #[doc = "Bit 0 - high temperature interrupt enable for src0"]
    #[inline(always)]
    pub fn ht_inten_src0(&self) -> HtIntenSrc0R {
        HtIntenSrc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - high temperature interrupt enable for src1"]
    #[inline(always)]
    pub fn ht_inten_src1(&self) -> HtIntenSrc1R {
        HtIntenSrc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tshut_2gpio_en_src0(&self) -> Tshut2gpioEnSrc0R {
        Tshut2gpioEnSrc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tshut_2gpio_en_src1(&self) -> Tshut2gpioEnSrc1R {
        Tshut2gpioEnSrc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tshut_2cru_en_src0(&self) -> Tshut2cruEnSrc0R {
        Tshut2cruEnSrc0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tshut_2cru_en_src1(&self) -> Tshut2cruEnSrc1R {
        Tshut2cruEnSrc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - low temperature interrupt enable for src0"]
    #[inline(always)]
    pub fn lt_inten_src0(&self) -> LtIntenSrc0R {
        LtIntenSrc0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - low temperature interrupt enable for src1"]
    #[inline(always)]
    pub fn lt_inten_src1(&self) -> LtIntenSrc1R {
        LtIntenSrc1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - eoc_Interrupt enable.\n\neoc_interrupt enable in user defined mode"]
    #[inline(always)]
    pub fn eoc_int_en(&self) -> EocIntEnR {
        EocIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - high temperature interrupt enable for src0"]
    #[inline(always)]
    #[must_use]
    pub fn ht_inten_src0(&mut self) -> HtIntenSrc0W<TsadcIntEnSpec> {
        HtIntenSrc0W::new(self, 0)
    }
    #[doc = "Bit 1 - high temperature interrupt enable for src1"]
    #[inline(always)]
    #[must_use]
    pub fn ht_inten_src1(&mut self) -> HtIntenSrc1W<TsadcIntEnSpec> {
        HtIntenSrc1W::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tshut_2gpio_en_src0(&mut self) -> Tshut2gpioEnSrc0W<TsadcIntEnSpec> {
        Tshut2gpioEnSrc0W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tshut_2gpio_en_src1(&mut self) -> Tshut2gpioEnSrc1W<TsadcIntEnSpec> {
        Tshut2gpioEnSrc1W::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tshut_2cru_en_src0(&mut self) -> Tshut2cruEnSrc0W<TsadcIntEnSpec> {
        Tshut2cruEnSrc0W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tshut_2cru_en_src1(&mut self) -> Tshut2cruEnSrc1W<TsadcIntEnSpec> {
        Tshut2cruEnSrc1W::new(self, 9)
    }
    #[doc = "Bit 12 - low temperature interrupt enable for src0"]
    #[inline(always)]
    #[must_use]
    pub fn lt_inten_src0(&mut self) -> LtIntenSrc0W<TsadcIntEnSpec> {
        LtIntenSrc0W::new(self, 12)
    }
    #[doc = "Bit 13 - low temperature interrupt enable for src1"]
    #[inline(always)]
    #[must_use]
    pub fn lt_inten_src1(&mut self) -> LtIntenSrc1W<TsadcIntEnSpec> {
        LtIntenSrc1W::new(self, 13)
    }
    #[doc = "Bit 16 - eoc_Interrupt enable.\n\neoc_interrupt enable in user defined mode"]
    #[inline(always)]
    #[must_use]
    pub fn eoc_int_en(&mut self) -> EocIntEnW<TsadcIntEnSpec> {
        EocIntEnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsadcIntEnSpec;
impl crate::RegisterSpec for TsadcIntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsadc_int_en::R`](R) reader structure"]
impl crate::Readable for TsadcIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`tsadc_int_en::W`](W) writer structure"]
impl crate::Writable for TsadcIntEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSADC_INT_EN to value 0"]
impl crate::Resettable for TsadcIntEnSpec {
    const RESET_VALUE: u32 = 0;
}

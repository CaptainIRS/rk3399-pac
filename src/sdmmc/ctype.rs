#[doc = "Register `CTYPE` reader"]
pub type R = crate::R<CtypeSpec>;
#[doc = "Register `CTYPE` writer"]
pub type W = crate::W<CtypeSpec>;
#[doc = "Indicates if card is 1-bit or 4-bit:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardWidth {
    #[doc = "0: 1-bit mode"]
    B0 = 0,
    #[doc = "1: 4-bit mode"]
    B1 = 1,
}
impl From<CardWidth> for bool {
    #[inline(always)]
    fn from(variant: CardWidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_WIDTH` reader - Indicates if card is 1-bit or 4-bit:"]
pub type CardWidthR = crate::BitReader<CardWidth>;
impl CardWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardWidth {
        match self.bits {
            false => CardWidth::B0,
            true => CardWidth::B1,
        }
    }
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CardWidth::B0
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CardWidth::B1
    }
}
#[doc = "Field `CARD_WIDTH` writer - Indicates if card is 1-bit or 4-bit:"]
pub type CardWidthW<'a, REG> = crate::BitWriter<'a, REG, CardWidth>;
impl<'a, REG> CardWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CardWidth::B0)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CardWidth::B1)
    }
}
#[doc = "Indicates if card is 8-bit:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardWidth8 {
    #[doc = "0: non 8-bit mode"]
    B0 = 0,
    #[doc = "1: 8-bit mode"]
    B1 = 1,
}
impl From<CardWidth8> for bool {
    #[inline(always)]
    fn from(variant: CardWidth8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_WIDTH_8` reader - Indicates if card is 8-bit:"]
pub type CardWidth8R = crate::BitReader<CardWidth8>;
impl CardWidth8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardWidth8 {
        match self.bits {
            false => CardWidth8::B0,
            true => CardWidth8::B1,
        }
    }
    #[doc = "non 8-bit mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CardWidth8::B0
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CardWidth8::B1
    }
}
#[doc = "Field `CARD_WIDTH_8` writer - Indicates if card is 8-bit:"]
pub type CardWidth8W<'a, REG> = crate::BitWriter<'a, REG, CardWidth8>;
impl<'a, REG> CardWidth8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "non 8-bit mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CardWidth8::B0)
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CardWidth8::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if card is 1-bit or 4-bit:"]
    #[inline(always)]
    pub fn card_width(&self) -> CardWidthR {
        CardWidthR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates if card is 8-bit:"]
    #[inline(always)]
    pub fn card_width_8(&self) -> CardWidth8R {
        CardWidth8R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if card is 1-bit or 4-bit:"]
    #[inline(always)]
    #[must_use]
    pub fn card_width(&mut self) -> CardWidthW<CtypeSpec> {
        CardWidthW::new(self, 0)
    }
    #[doc = "Bit 16 - Indicates if card is 8-bit:"]
    #[inline(always)]
    #[must_use]
    pub fn card_width_8(&mut self) -> CardWidth8W<CtypeSpec> {
        CardWidth8W::new(self, 16)
    }
}
#[doc = "Card-type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtypeSpec;
impl crate::RegisterSpec for CtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctype::R`](R) reader structure"]
impl crate::Readable for CtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`ctype::W`](W) writer structure"]
impl crate::Writable for CtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTYPE to value 0"]
impl crate::Resettable for CtypeSpec {
    const RESET_VALUE: u32 = 0;
}

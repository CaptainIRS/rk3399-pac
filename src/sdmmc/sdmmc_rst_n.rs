#[doc = "Register `SDMMC_RST_n` reader"]
pub type R = crate::R<SdmmcRstNSpec>;
#[doc = "Register `SDMMC_RST_n` writer"]
pub type W = crate::W<SdmmcRstNSpec>;
#[doc = "Hardware reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardReset {
    #[doc = "0: reset These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. CARD_RESET\\[0\\]
should be set to 1'b1 to reset card."]
    B0 = 0,
    #[doc = "1: reset These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. CARD_RESET\\[0\\]
should be set to 1'b1 to reset card."]
    B1 = 1,
}
impl From<CardReset> for bool {
    #[inline(always)]
    fn from(variant: CardReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_RESET` reader - Hardware reset."]
pub type CardResetR = crate::BitReader<CardReset>;
impl CardResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardReset {
        match self.bits {
            false => CardReset::B0,
            true => CardReset::B1,
        }
    }
    #[doc = "reset These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. CARD_RESET\\[0\\]
should be set to 1'b1 to reset card."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CardReset::B0
    }
    #[doc = "reset These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. CARD_RESET\\[0\\]
should be set to 1'b1 to reset card."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CardReset::B1
    }
}
#[doc = "Field `CARD_RESET` writer - Hardware reset."]
pub type CardResetW<'a, REG> = crate::BitWriter<'a, REG, CardReset>;
impl<'a, REG> CardResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reset These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. CARD_RESET\\[0\\]
should be set to 1'b1 to reset card."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CardReset::B0)
    }
    #[doc = "reset These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. CARD_RESET\\[0\\]
should be set to 1'b1 to reset card."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CardReset::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Hardware reset."]
    #[inline(always)]
    pub fn card_reset(&self) -> CardResetR {
        CardResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware reset."]
    #[inline(always)]
    #[must_use]
    pub fn card_reset(&mut self) -> CardResetW<SdmmcRstNSpec> {
        CardResetW::new(self, 0)
    }
}
#[doc = "Hardware reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_rst_n::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_rst_n::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcRstNSpec;
impl crate::RegisterSpec for SdmmcRstNSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_rst_n::R`](R) reader structure"]
impl crate::Readable for SdmmcRstNSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_rst_n::W`](W) writer structure"]
impl crate::Writable for SdmmcRstNSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_RST_n to value 0x01"]
impl crate::Resettable for SdmmcRstNSpec {
    const RESET_VALUE: u32 = 0x01;
}

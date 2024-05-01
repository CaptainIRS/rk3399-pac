#[doc = "Register `BLANKING_VALUE` reader"]
pub type R = crate::R<BlankingValueSpec>;
#[doc = "Register `BLANKING_VALUE` writer"]
pub type W = crate::W<BlankingValueSpec>;
#[doc = "Field `BLANKING_VALUE` reader - vop output data value when blanking,24bits"]
pub type BlankingValueR = crate::FieldReader<u32>;
#[doc = "Field `BLANKING_VALUE` writer - vop output data value when blanking,24bits"]
pub type BlankingValueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlankingValueConfigEn {
    #[doc = "0: disable blanking value when vop blank"]
    B0 = 0,
    #[doc = "1: enable blanking value when vop blank"]
    B1 = 1,
}
impl From<BlankingValueConfigEn> for bool {
    #[inline(always)]
    fn from(variant: BlankingValueConfigEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLANKING_VALUE_CONFIG_EN` reader - "]
pub type BlankingValueConfigEnR = crate::BitReader<BlankingValueConfigEn>;
impl BlankingValueConfigEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlankingValueConfigEn {
        match self.bits {
            false => BlankingValueConfigEn::B0,
            true => BlankingValueConfigEn::B1,
        }
    }
    #[doc = "disable blanking value when vop blank"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BlankingValueConfigEn::B0
    }
    #[doc = "enable blanking value when vop blank"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BlankingValueConfigEn::B1
    }
}
#[doc = "Field `BLANKING_VALUE_CONFIG_EN` writer - "]
pub type BlankingValueConfigEnW<'a, REG> = crate::BitWriter<'a, REG, BlankingValueConfigEn>;
impl<'a, REG> BlankingValueConfigEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable blanking value when vop blank"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BlankingValueConfigEn::B0)
    }
    #[doc = "enable blanking value when vop blank"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BlankingValueConfigEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:23 - vop output data value when blanking,24bits"]
    #[inline(always)]
    pub fn blanking_value(&self) -> BlankingValueR {
        BlankingValueR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn blanking_value_config_en(&self) -> BlankingValueConfigEnR {
        BlankingValueConfigEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - vop output data value when blanking,24bits"]
    #[inline(always)]
    #[must_use]
    pub fn blanking_value(&mut self) -> BlankingValueW<BlankingValueSpec> {
        BlankingValueW::new(self, 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn blanking_value_config_en(&mut self) -> BlankingValueConfigEnW<BlankingValueSpec> {
        BlankingValueConfigEnW::new(self, 24)
    }
}
#[doc = "Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blanking_value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blanking_value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlankingValueSpec;
impl crate::RegisterSpec for BlankingValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blanking_value::R`](R) reader structure"]
impl crate::Readable for BlankingValueSpec {}
#[doc = "`write(|w| ..)` method takes [`blanking_value::W`](W) writer structure"]
impl crate::Writable for BlankingValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLANKING_VALUE to value 0"]
impl crate::Resettable for BlankingValueSpec {
    const RESET_VALUE: u32 = 0;
}

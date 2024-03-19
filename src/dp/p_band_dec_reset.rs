#[doc = "Register `P_BAND_DEC_RESET` reader"]
pub type R = crate::R<PBandDecResetSpec>;
#[doc = "Register `P_BAND_DEC_RESET` writer"]
pub type W = crate::W<PBandDecResetSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBandDecReset {
    #[doc = "1: reset band decoder"]
    B1 = 1,
    #[doc = "0: band decoder works"]
    B0 = 0,
}
impl From<RBandDecReset> for bool {
    #[inline(always)]
    fn from(variant: RBandDecReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_BAND_DEC_RESET` reader - "]
pub type RBandDecResetR = crate::BitReader<RBandDecReset>;
impl RBandDecResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBandDecReset {
        match self.bits {
            true => RBandDecReset::B1,
            false => RBandDecReset::B0,
        }
    }
    #[doc = "reset band decoder"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RBandDecReset::B1
    }
    #[doc = "band decoder works"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RBandDecReset::B0
    }
}
#[doc = "Field `R_BAND_DEC_RESET` writer - "]
pub type RBandDecResetW<'a, REG> = crate::BitWriter1C<'a, REG, RBandDecReset>;
impl<'a, REG> RBandDecResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reset band decoder"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RBandDecReset::B1)
    }
    #[doc = "band decoder works"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RBandDecReset::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_band_dec_reset(&self) -> RBandDecResetR {
        RBandDecResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn r_band_dec_reset(&mut self) -> RBandDecResetW<PBandDecResetSpec> {
        RBandDecResetW::new(self, 0)
    }
}
#[doc = "reset band decoder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p_band_dec_reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p_band_dec_reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PBandDecResetSpec;
impl crate::RegisterSpec for PBandDecResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p_band_dec_reset::R`](R) reader structure"]
impl crate::Readable for PBandDecResetSpec {}
#[doc = "`write(|w| ..)` method takes [`p_band_dec_reset::W`](W) writer structure"]
impl crate::Writable for PBandDecResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets P_BAND_DEC_RESET to value 0"]
impl crate::Resettable for PBandDecResetSpec {
    const RESET_VALUE: u32 = 0;
}

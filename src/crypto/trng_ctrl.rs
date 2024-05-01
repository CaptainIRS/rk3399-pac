#[doc = "Register `TRNG_CTRL` reader"]
pub type R = crate::R<TrngCtrlSpec>;
#[doc = "Register `TRNG_CTRL` writer"]
pub type W = crate::W<TrngCtrlSpec>;
#[doc = "Field `PERIOD` reader - sample period TRNG use clock_crypto to sample ring osc output,\n\nthis parameter is specify how many cycles to generate 1 bit\n\nrandom data."]
pub type PeriodR = crate::FieldReader<u16>;
#[doc = "Field `PERIOD` writer - sample period TRNG use clock_crypto to sample ring osc output,\n\nthis parameter is specify how many cycles to generate 1 bit\n\nrandom data."]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "osc_ring enable\n\nIt controls the running of osc_ring. And it is independent of clock\n\nand flush signal. This mean that it can run even when clock is\n\ngating or flush is asserted as long as osc_enable is asserted.\n\nBefore it is used to get TRNG result , please run osc_ring first to\n\nget enough entropy.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OscEnable {
    #[doc = "1: Enable ;"]
    B1 = 1,
    #[doc = "0: Disable ;"]
    B0 = 0,
}
impl From<OscEnable> for bool {
    #[inline(always)]
    fn from(variant: OscEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSC_ENABLE` reader - osc_ring enable\n\nIt controls the running of osc_ring. And it is independent of clock\n\nand flush signal. This mean that it can run even when clock is\n\ngating or flush is asserted as long as osc_enable is asserted.\n\nBefore it is used to get TRNG result , please run osc_ring first to\n\nget enough entropy."]
pub type OscEnableR = crate::BitReader<OscEnable>;
impl OscEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OscEnable {
        match self.bits {
            true => OscEnable::B1,
            false => OscEnable::B0,
        }
    }
    #[doc = "Enable ;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == OscEnable::B1
    }
    #[doc = "Disable ;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == OscEnable::B0
    }
}
#[doc = "Field `OSC_ENABLE` writer - osc_ring enable\n\nIt controls the running of osc_ring. And it is independent of clock\n\nand flush signal. This mean that it can run even when clock is\n\ngating or flush is asserted as long as osc_enable is asserted.\n\nBefore it is used to get TRNG result , please run osc_ring first to\n\nget enough entropy."]
pub type OscEnableW<'a, REG> = crate::BitWriter<'a, REG, OscEnable>;
impl<'a, REG> OscEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable ;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(OscEnable::B1)
    }
    #[doc = "Disable ;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(OscEnable::B0)
    }
}
impl R {
    #[doc = "Bits 0:15 - sample period TRNG use clock_crypto to sample ring osc output,\n\nthis parameter is specify how many cycles to generate 1 bit\n\nrandom data."]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - osc_ring enable\n\nIt controls the running of osc_ring. And it is independent of clock\n\nand flush signal. This mean that it can run even when clock is\n\ngating or flush is asserted as long as osc_enable is asserted.\n\nBefore it is used to get TRNG result , please run osc_ring first to\n\nget enough entropy."]
    #[inline(always)]
    pub fn osc_enable(&self) -> OscEnableR {
        OscEnableR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - sample period TRNG use clock_crypto to sample ring osc output,\n\nthis parameter is specify how many cycles to generate 1 bit\n\nrandom data."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PeriodW<TrngCtrlSpec> {
        PeriodW::new(self, 0)
    }
    #[doc = "Bit 16 - osc_ring enable\n\nIt controls the running of osc_ring. And it is independent of clock\n\nand flush signal. This mean that it can run even when clock is\n\ngating or flush is asserted as long as osc_enable is asserted.\n\nBefore it is used to get TRNG result , please run osc_ring first to\n\nget enough entropy."]
    #[inline(always)]
    #[must_use]
    pub fn osc_enable(&mut self) -> OscEnableW<TrngCtrlSpec> {
        OscEnableW::new(self, 16)
    }
}
#[doc = "TRNG Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trng_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trng_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngCtrlSpec;
impl crate::RegisterSpec for TrngCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_ctrl::R`](R) reader structure"]
impl crate::Readable for TrngCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`trng_ctrl::W`](W) writer structure"]
impl crate::Writable for TrngCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRNG_CTRL to value 0"]
impl crate::Resettable for TrngCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

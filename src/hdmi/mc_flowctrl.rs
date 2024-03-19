#[doc = "Register `MC_FLOWCTRL` reader"]
pub type R = crate::R<McFlowctrlSpec>;
#[doc = "Register `MC_FLOWCTRL` writer"]
pub type W = crate::W<McFlowctrlSpec>;
#[doc = "Video path Feed Through enable bit:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedThroughOff {
    #[doc = "1: Color Space Converter is in the video data path."]
    B1 = 1,
    #[doc = "0: Color Space Converter is bypassed (not in the video data path)."]
    B0 = 0,
}
impl From<FeedThroughOff> for bool {
    #[inline(always)]
    fn from(variant: FeedThroughOff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEED_THROUGH_OFF` reader - Video path Feed Through enable bit:"]
pub type FeedThroughOffR = crate::BitReader<FeedThroughOff>;
impl FeedThroughOffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FeedThroughOff {
        match self.bits {
            true => FeedThroughOff::B1,
            false => FeedThroughOff::B0,
        }
    }
    #[doc = "Color Space Converter is in the video data path."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FeedThroughOff::B1
    }
    #[doc = "Color Space Converter is bypassed (not in the video data path)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FeedThroughOff::B0
    }
}
#[doc = "Field `FEED_THROUGH_OFF` writer - Video path Feed Through enable bit:"]
pub type FeedThroughOffW<'a, REG> = crate::BitWriter<'a, REG, FeedThroughOff>;
impl<'a, REG> FeedThroughOffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Color Space Converter is in the video data path."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FeedThroughOff::B1)
    }
    #[doc = "Color Space Converter is bypassed (not in the video data path)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FeedThroughOff::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Video path Feed Through enable bit:"]
    #[inline(always)]
    pub fn feed_through_off(&self) -> FeedThroughOffR {
        FeedThroughOffR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Video path Feed Through enable bit:"]
    #[inline(always)]
    #[must_use]
    pub fn feed_through_off(&mut self) -> FeedThroughOffW<McFlowctrlSpec> {
        FeedThroughOffW::new(self, 0)
    }
}
#[doc = "Main Controller Feed Through Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_flowctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_flowctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McFlowctrlSpec;
impl crate::RegisterSpec for McFlowctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_flowctrl::R`](R) reader structure"]
impl crate::Readable for McFlowctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mc_flowctrl::W`](W) writer structure"]
impl crate::Writable for McFlowctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MC_FLOWCTRL to value 0"]
impl crate::Resettable for McFlowctrlSpec {
    const RESET_VALUE: u8 = 0;
}

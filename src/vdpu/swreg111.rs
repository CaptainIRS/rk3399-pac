#[doc = "Register `SWREG111` reader"]
pub type R = crate::R<Swreg111Spec>;
#[doc = "Register `SWREG111` writer"]
pub type W = crate::W<Swreg111Spec>;
#[doc = "Field `H264_MAX_REFNUM` reader - short and long term reference frames's maximum number\n\nthis value is for decoded picture buffer"]
pub type H264MaxRefnumR = crate::FieldReader;
#[doc = "Field `H264_MAX_REFNUM` writer - short and long term reference frames's maximum number\n\nthis value is for decoded picture buffer"]
pub type H264MaxRefnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "wp(weighted prediction) specification for B slice\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum H264WpBsliceSel {
    #[doc = "0: default wp be used"]
    D0 = 0,
    #[doc = "1: explicit wp be used"]
    D1 = 1,
    #[doc = "2: implicit wp be used"]
    D2 = 2,
}
impl From<H264WpBsliceSel> for u8 {
    #[inline(always)]
    fn from(variant: H264WpBsliceSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for H264WpBsliceSel {
    type Ux = u8;
}
#[doc = "Field `H264_WP_BSLICE_SEL` reader - wp(weighted prediction) specification for B slice"]
pub type H264WpBsliceSelR = crate::FieldReader<H264WpBsliceSel>;
impl H264WpBsliceSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<H264WpBsliceSel> {
        match self.bits {
            0 => Some(H264WpBsliceSel::D0),
            1 => Some(H264WpBsliceSel::D1),
            2 => Some(H264WpBsliceSel::D2),
            _ => None,
        }
    }
    #[doc = "default wp be used"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == H264WpBsliceSel::D0
    }
    #[doc = "explicit wp be used"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == H264WpBsliceSel::D1
    }
    #[doc = "implicit wp be used"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == H264WpBsliceSel::D2
    }
}
#[doc = "Field `H264_WP_BSLICE_SEL` writer - wp(weighted prediction) specification for B slice"]
pub type H264WpBsliceSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, H264WpBsliceSel>;
impl<'a, REG> H264WpBsliceSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "default wp be used"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(H264WpBsliceSel::D0)
    }
    #[doc = "explicit wp be used"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(H264WpBsliceSel::D1)
    }
    #[doc = "implicit wp be used"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(H264WpBsliceSel::D2)
    }
}
impl R {
    #[doc = "Bits 0:4 - short and long term reference frames's maximum number\n\nthis value is for decoded picture buffer"]
    #[inline(always)]
    pub fn h264_max_refnum(&self) -> H264MaxRefnumR {
        H264MaxRefnumR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - wp(weighted prediction) specification for B slice"]
    #[inline(always)]
    pub fn h264_wp_bslice_sel(&self) -> H264WpBsliceSelR {
        H264WpBsliceSelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - short and long term reference frames's maximum number\n\nthis value is for decoded picture buffer"]
    #[inline(always)]
    #[must_use]
    pub fn h264_max_refnum(&mut self) -> H264MaxRefnumW<Swreg111Spec> {
        H264MaxRefnumW::new(self, 0)
    }
    #[doc = "Bits 16:17 - wp(weighted prediction) specification for B slice"]
    #[inline(always)]
    #[must_use]
    pub fn h264_wp_bslice_sel(&mut self) -> H264WpBsliceSelW<Swreg111Spec> {
        H264WpBsliceSelW::new(self, 16)
    }
}
#[doc = "h264 ctrl related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg111::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg111::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg111Spec;
impl crate::RegisterSpec for Swreg111Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg111::R`](R) reader structure"]
impl crate::Readable for Swreg111Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg111::W`](W) writer structure"]
impl crate::Writable for Swreg111Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG111 to value 0"]
impl crate::Resettable for Swreg111Spec {
    const RESET_VALUE: u32 = 0;
}

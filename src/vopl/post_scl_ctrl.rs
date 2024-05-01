#[doc = "Register `POST_SCL_CTRL` reader"]
pub type R = crate::R<PostSclCtrlSpec>;
#[doc = "Register `POST_SCL_CTRL` writer"]
pub type W = crate::W<PostSclCtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PostHorSdEn {
    #[doc = "0: post hor scl down disable"]
    B0 = 0,
    #[doc = "1: post hor scl down enable"]
    B1 = 1,
}
impl From<PostHorSdEn> for bool {
    #[inline(always)]
    fn from(variant: PostHorSdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POST_HOR_SD_EN` reader - "]
pub type PostHorSdEnR = crate::BitReader<PostHorSdEn>;
impl PostHorSdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PostHorSdEn {
        match self.bits {
            false => PostHorSdEn::B0,
            true => PostHorSdEn::B1,
        }
    }
    #[doc = "post hor scl down disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PostHorSdEn::B0
    }
    #[doc = "post hor scl down enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PostHorSdEn::B1
    }
}
#[doc = "Field `POST_HOR_SD_EN` writer - "]
pub type PostHorSdEnW<'a, REG> = crate::BitWriter<'a, REG, PostHorSdEn>;
impl<'a, REG> PostHorSdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "post hor scl down disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PostHorSdEn::B0)
    }
    #[doc = "post hor scl down enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PostHorSdEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PostVerSdEn {
    #[doc = "0: post ver scl down disable"]
    B0 = 0,
    #[doc = "1: post ver scl down enable"]
    B1 = 1,
}
impl From<PostVerSdEn> for bool {
    #[inline(always)]
    fn from(variant: PostVerSdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POST_VER_SD_EN` reader - "]
pub type PostVerSdEnR = crate::BitReader<PostVerSdEn>;
impl PostVerSdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PostVerSdEn {
        match self.bits {
            false => PostVerSdEn::B0,
            true => PostVerSdEn::B1,
        }
    }
    #[doc = "post ver scl down disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PostVerSdEn::B0
    }
    #[doc = "post ver scl down enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PostVerSdEn::B1
    }
}
#[doc = "Field `POST_VER_SD_EN` writer - "]
pub type PostVerSdEnW<'a, REG> = crate::BitWriter<'a, REG, PostVerSdEn>;
impl<'a, REG> PostVerSdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "post ver scl down disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PostVerSdEn::B0)
    }
    #[doc = "post ver scl down enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PostVerSdEn::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn post_hor_sd_en(&self) -> PostHorSdEnR {
        PostHorSdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn post_ver_sd_en(&self) -> PostVerSdEnR {
        PostVerSdEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn post_hor_sd_en(&mut self) -> PostHorSdEnW<PostSclCtrlSpec> {
        PostHorSdEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn post_ver_sd_en(&mut self) -> PostVerSdEnW<PostSclCtrlSpec> {
        PostVerSdEnW::new(self, 1)
    }
}
#[doc = "Post scaling start point offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`post_scl_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`post_scl_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PostSclCtrlSpec;
impl crate::RegisterSpec for PostSclCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`post_scl_ctrl::R`](R) reader structure"]
impl crate::Readable for PostSclCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`post_scl_ctrl::W`](W) writer structure"]
impl crate::Writable for PostSclCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POST_SCL_CTRL to value 0"]
impl crate::Resettable for PostSclCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

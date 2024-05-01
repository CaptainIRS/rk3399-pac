#[doc = "Register `SWREG0_ID` reader"]
pub type R = crate::R<Swreg0IdSpec>;
#[doc = "Register `SWREG0_ID` writer"]
pub type W = crate::W<Swreg0IdSpec>;
#[doc = "Field `MINOR_VER` reader - minor version\n\nminor version"]
pub type MinorVerR = crate::FieldReader;
#[doc = "level\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    #[doc = "0: FHD"]
    B0 = 0,
    #[doc = "1: UHD"]
    B1 = 1,
}
impl From<Level> for bool {
    #[inline(always)]
    fn from(variant: Level) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEVEL` reader - level"]
pub type LevelR = crate::BitReader<Level>;
impl LevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Level {
        match self.bits {
            false => Level::B0,
            true => Level::B1,
        }
    }
    #[doc = "FHD"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Level::B0
    }
    #[doc = "UHD"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Level::B1
    }
}
#[doc = "Field `DEC_SUPPORT` reader - dec support bits\n\nbit0: HEVC support or not, when it is 1'b1, support\n\nbit1: H264 support or not\n\nbit2: VP9 support or not"]
pub type DecSupportR = crate::FieldReader;
#[doc = "hevc profile\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    #[doc = "0: Main"]
    B0 = 0,
    #[doc = "1: Main10"]
    B1 = 1,
}
impl From<Profile> for bool {
    #[inline(always)]
    fn from(variant: Profile) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROFILE` reader - hevc profile"]
pub type ProfileR = crate::BitReader<Profile>;
impl ProfileR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Profile {
        match self.bits {
            false => Profile::B0,
            true => Profile::B1,
        }
    }
    #[doc = "Main"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Profile::B0
    }
    #[doc = "Main10"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Profile::B1
    }
}
#[doc = "Field `PROFILE` writer - hevc profile"]
pub type ProfileW<'a, REG> = crate::BitWriter<'a, REG, Profile>;
impl<'a, REG> ProfileW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Main"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Profile::B0)
    }
    #[doc = "Main10"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Profile::B1)
    }
}
#[doc = "codec flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodecFlag {
    #[doc = "0: only dec"]
    B0 = 0,
    #[doc = "1: dec + enc"]
    B1 = 1,
}
impl From<CodecFlag> for bool {
    #[inline(always)]
    fn from(variant: CodecFlag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CODEC_FLAG` reader - codec flag"]
pub type CodecFlagR = crate::BitReader<CodecFlag>;
impl CodecFlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CodecFlag {
        match self.bits {
            false => CodecFlag::B0,
            true => CodecFlag::B1,
        }
    }
    #[doc = "only dec"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CodecFlag::B0
    }
    #[doc = "dec + enc"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CodecFlag::B1
    }
}
#[doc = "Field `PROD_NUM` reader - product number\n\nThe ascii code of 'hv', which is 0x6876"]
pub type ProdNumR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - minor version\n\nminor version"]
    #[inline(always)]
    pub fn minor_ver(&self) -> MinorVerR {
        MinorVerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - level"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - dec support bits\n\nbit0: HEVC support or not, when it is 1'b1, support\n\nbit1: H264 support or not\n\nbit2: VP9 support or not"]
    #[inline(always)]
    pub fn dec_support(&self) -> DecSupportR {
        DecSupportR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - hevc profile"]
    #[inline(always)]
    pub fn profile(&self) -> ProfileR {
        ProfileR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - codec flag"]
    #[inline(always)]
    pub fn codec_flag(&self) -> CodecFlagR {
        CodecFlagR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - product number\n\nThe ascii code of 'hv', which is 0x6876"]
    #[inline(always)]
    pub fn prod_num(&self) -> ProdNumR {
        ProdNumR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 12 - hevc profile"]
    #[inline(always)]
    #[must_use]
    pub fn profile(&mut self) -> ProfileW<Swreg0IdSpec> {
        ProfileW::new(self, 12)
    }
}
#[doc = "ID register (read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg0_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg0_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg0IdSpec;
impl crate::RegisterSpec for Swreg0IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg0_id::R`](R) reader structure"]
impl crate::Readable for Swreg0IdSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg0_id::W`](W) writer structure"]
impl crate::Writable for Swreg0IdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG0_ID to value 0x6876_1f00"]
impl crate::Resettable for Swreg0IdSpec {
    const RESET_VALUE: u32 = 0x6876_1f00;
}

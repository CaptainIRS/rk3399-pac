#[doc = "Register `SWREG_74` reader"]
pub type R = crate::R<Swreg74Spec>;
#[doc = "Register `SWREG_74` writer"]
pub type W = crate::W<Swreg74Spec>;
#[doc = "Field `NAL_MODE` reader - the output of NAL size to base control\n\nthe output of NAL size to base control"]
pub type NalModeR = crate::BitReader;
#[doc = "Field `NAL_MODE` writer - the output of NAL size to base control\n\nthe output of NAL size to base control"]
pub type NalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "the input image rotation\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ImgInRot {
    #[doc = "0: no rotation"]
    D0 = 0,
    #[doc = "1: rotate right 90 degress"]
    D1 = 1,
    #[doc = "2: rotate left 90 degress"]
    D2 = 2,
}
impl From<ImgInRot> for u8 {
    #[inline(always)]
    fn from(variant: ImgInRot) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ImgInRot {
    type Ux = u8;
}
#[doc = "Field `IMG_IN_ROT` reader - the input image rotation"]
pub type ImgInRotR = crate::FieldReader<ImgInRot>;
impl ImgInRotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ImgInRot> {
        match self.bits {
            0 => Some(ImgInRot::D0),
            1 => Some(ImgInRot::D1),
            2 => Some(ImgInRot::D2),
            _ => None,
        }
    }
    #[doc = "no rotation"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == ImgInRot::D0
    }
    #[doc = "rotate right 90 degress"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == ImgInRot::D1
    }
    #[doc = "rotate left 90 degress"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == ImgInRot::D2
    }
}
#[doc = "Field `IMG_IN_ROT` writer - the input image rotation"]
pub type ImgInRotW<'a, REG> = crate::FieldWriter<'a, REG, 2, ImgInRot>;
impl<'a, REG> ImgInRotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no rotation"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(ImgInRot::D0)
    }
    #[doc = "rotate right 90 degress"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(ImgInRot::D1)
    }
    #[doc = "rotate left 90 degress"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(ImgInRot::D2)
    }
}
#[doc = "Field `IMG_FMT_IN` reader - input image format.\n\nYUV420P\n\nYUV420SP\n\nYUV422\n\nUYVY422\n\nRGB565\n\nRGB444\n\nRGB888\n\nRGB101010"]
pub type ImgFmtInR = crate::FieldReader;
#[doc = "Field `IMG_FMT_IN` writer - input image format.\n\nYUV420P\n\nYUV420SP\n\nYUV422\n\nUYVY422\n\nRGB565\n\nRGB444\n\nRGB888\n\nRGB101010"]
pub type ImgFmtInW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENCODERED_SLICES` reader - the number of encoder slices which used in h.264\n\nthe number of encoder slices which used in h.264"]
pub type EncoderedSlicesR = crate::FieldReader;
#[doc = "Field `ENCODERED_SLICES` writer - the number of encoder slices which used in h.264\n\nthe number of encoder slices which used in h.264"]
pub type EncoderedSlicesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAD_THSLD` reader - MAD threshold\n\nvalue = (MAD threshold)/256"]
pub type MadThsldR = crate::FieldReader;
#[doc = "Field `MAD_THSLD` writer - MAD threshold\n\nvalue = (MAD threshold)/256"]
pub type MadThsldW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - the output of NAL size to base control\n\nthe output of NAL size to base control"]
    #[inline(always)]
    pub fn nal_mode(&self) -> NalModeR {
        NalModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - the input image rotation"]
    #[inline(always)]
    pub fn img_in_rot(&self) -> ImgInRotR {
        ImgInRotR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - input image format.\n\nYUV420P\n\nYUV420SP\n\nYUV422\n\nUYVY422\n\nRGB565\n\nRGB444\n\nRGB888\n\nRGB101010"]
    #[inline(always)]
    pub fn img_fmt_in(&self) -> ImgFmtInR {
        ImgFmtInR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - the number of encoder slices which used in h.264\n\nthe number of encoder slices which used in h.264"]
    #[inline(always)]
    pub fn encodered_slices(&self) -> EncoderedSlicesR {
        EncoderedSlicesR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - MAD threshold\n\nvalue = (MAD threshold)/256"]
    #[inline(always)]
    pub fn mad_thsld(&self) -> MadThsldR {
        MadThsldR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - the output of NAL size to base control\n\nthe output of NAL size to base control"]
    #[inline(always)]
    #[must_use]
    pub fn nal_mode(&mut self) -> NalModeW<Swreg74Spec> {
        NalModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - the input image rotation"]
    #[inline(always)]
    #[must_use]
    pub fn img_in_rot(&mut self) -> ImgInRotW<Swreg74Spec> {
        ImgInRotW::new(self, 2)
    }
    #[doc = "Bits 4:7 - input image format.\n\nYUV420P\n\nYUV420SP\n\nYUV422\n\nUYVY422\n\nRGB565\n\nRGB444\n\nRGB888\n\nRGB101010"]
    #[inline(always)]
    #[must_use]
    pub fn img_fmt_in(&mut self) -> ImgFmtInW<Swreg74Spec> {
        ImgFmtInW::new(self, 4)
    }
    #[doc = "Bits 16:23 - the number of encoder slices which used in h.264\n\nthe number of encoder slices which used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn encodered_slices(&mut self) -> EncoderedSlicesW<Swreg74Spec> {
        EncoderedSlicesW::new(self, 16)
    }
    #[doc = "Bits 24:29 - MAD threshold\n\nvalue = (MAD threshold)/256"]
    #[inline(always)]
    #[must_use]
    pub fn mad_thsld(&mut self) -> MadThsldW<Swreg74Spec> {
        MadThsldW::new(self, 24)
    }
}
#[doc = "input image format\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_74::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_74::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg74Spec;
impl crate::RegisterSpec for Swreg74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_74::R`](R) reader structure"]
impl crate::Readable for Swreg74Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_74::W`](W) writer structure"]
impl crate::Writable for Swreg74Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_74 to value 0"]
impl crate::Resettable for Swreg74Spec {
    const RESET_VALUE: u32 = 0;
}

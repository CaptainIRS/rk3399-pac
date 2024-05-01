#[doc = "Register `SWREG_96` reader"]
pub type R = crate::R<Swreg96Spec>;
#[doc = "Register `SWREG_96` writer"]
pub type W = crate::W<Swreg96Spec>;
#[doc = "Field `RGB2YUV_COE3` reader - the 3st conversion coefficien for RGB to YUV\n\nthe 3st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe3R = crate::FieldReader<u16>;
#[doc = "Field `RGB2YUV_COE3` writer - the 3st conversion coefficien for RGB to YUV\n\nthe 3st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RGB2YUV_COE5` reader - the 5st conversion coefficien for RGB to YUV\n\nthe 5st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe5R = crate::FieldReader<u16>;
#[doc = "Field `RGB2YUV_COE5` writer - the 5st conversion coefficien for RGB to YUV\n\nthe 5st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the 3st conversion coefficien for RGB to YUV\n\nthe 3st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    pub fn rgb2yuv_coe3(&self) -> Rgb2yuvCoe3R {
        Rgb2yuvCoe3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the 5st conversion coefficien for RGB to YUV\n\nthe 5st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    pub fn rgb2yuv_coe5(&self) -> Rgb2yuvCoe5R {
        Rgb2yuvCoe5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the 3st conversion coefficien for RGB to YUV\n\nthe 3st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_coe3(&mut self) -> Rgb2yuvCoe3W<Swreg96Spec> {
        Rgb2yuvCoe3W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the 5st conversion coefficien for RGB to YUV\n\nthe 5st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_coe5(&mut self) -> Rgb2yuvCoe5W<Swreg96Spec> {
        Rgb2yuvCoe5W::new(self, 16)
    }
}
#[doc = "RGB to YUV conversion coefficient register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_96::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_96::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg96Spec;
impl crate::RegisterSpec for Swreg96Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_96::R`](R) reader structure"]
impl crate::Readable for Swreg96Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_96::W`](W) writer structure"]
impl crate::Writable for Swreg96Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_96 to value 0"]
impl crate::Resettable for Swreg96Spec {
    const RESET_VALUE: u32 = 0;
}

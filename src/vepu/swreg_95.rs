#[doc = "Register `SWREG_95` reader"]
pub type R = crate::R<Swreg95Spec>;
#[doc = "Register `SWREG_95` writer"]
pub type W = crate::W<Swreg95Spec>;
#[doc = "Field `RGB2YUV_COE1` reader - the 1st conversion coefficien for RGB to YUV\n\nthe 1st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe1R = crate::FieldReader<u16>;
#[doc = "Field `RGB2YUV_COE1` writer - the 1st conversion coefficien for RGB to YUV\n\nthe 1st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RGB2YUV_COE2` reader - the 2st conversion coefficien for RGB to YUV\n\nthe 2st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe2R = crate::FieldReader<u16>;
#[doc = "Field `RGB2YUV_COE2` writer - the 2st conversion coefficien for RGB to YUV\n\nthe 2st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the 1st conversion coefficien for RGB to YUV\n\nthe 1st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    pub fn rgb2yuv_coe1(&self) -> Rgb2yuvCoe1R {
        Rgb2yuvCoe1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the 2st conversion coefficien for RGB to YUV\n\nthe 2st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    pub fn rgb2yuv_coe2(&self) -> Rgb2yuvCoe2R {
        Rgb2yuvCoe2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the 1st conversion coefficien for RGB to YUV\n\nthe 1st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_coe1(&mut self) -> Rgb2yuvCoe1W<Swreg95Spec> {
        Rgb2yuvCoe1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the 2st conversion coefficien for RGB to YUV\n\nthe 2st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_coe2(&mut self) -> Rgb2yuvCoe2W<Swreg95Spec> {
        Rgb2yuvCoe2W::new(self, 16)
    }
}
#[doc = "RGB to YUV conversion coefficient register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_95::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_95::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg95Spec;
impl crate::RegisterSpec for Swreg95Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_95::R`](R) reader structure"]
impl crate::Readable for Swreg95Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_95::W`](W) writer structure"]
impl crate::Writable for Swreg95Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_95 to value 0"]
impl crate::Resettable for Swreg95Spec {
    const RESET_VALUE: u32 = 0;
}

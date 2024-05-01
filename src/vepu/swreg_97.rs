#[doc = "Register `SWREG_97` reader"]
pub type R = crate::R<Swreg97Spec>;
#[doc = "Register `SWREG_97` writer"]
pub type W = crate::W<Swreg97Spec>;
#[doc = "Field `RGB2YUV_COE6` reader - the 6st conversion coefficien for RGB to YUV\n\nthe 6st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe6R = crate::FieldReader<u16>;
#[doc = "Field `RGB2YUV_COE6` writer - the 6st conversion coefficien for RGB to YUV\n\nthe 6st conversion coefficien for RGB to YUV"]
pub type Rgb2yuvCoe6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the 6st conversion coefficien for RGB to YUV\n\nthe 6st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    pub fn rgb2yuv_coe6(&self) -> Rgb2yuvCoe6R {
        Rgb2yuvCoe6R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the 6st conversion coefficien for RGB to YUV\n\nthe 6st conversion coefficien for RGB to YUV"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_coe6(&mut self) -> Rgb2yuvCoe6W<Swreg97Spec> {
        Rgb2yuvCoe6W::new(self, 0)
    }
}
#[doc = "RGB to YUV conversion coefficient register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_97::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_97::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg97Spec;
impl crate::RegisterSpec for Swreg97Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_97::R`](R) reader structure"]
impl crate::Readable for Swreg97Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_97::W`](W) writer structure"]
impl crate::Writable for Swreg97Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_97 to value 0"]
impl crate::Resettable for Swreg97Spec {
    const RESET_VALUE: u32 = 0;
}

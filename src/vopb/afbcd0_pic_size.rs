#[doc = "Register `AFBCD0_PIC_SIZE` reader"]
pub type R = crate::R<Afbcd0PicSizeSpec>;
#[doc = "Register `AFBCD0_PIC_SIZE` writer"]
pub type W = crate::W<Afbcd0PicSizeSpec>;
#[doc = "Field `AFBCD_HREG_PIC_WIDTH` reader - afbcd_hreg_pic_width"]
pub type AfbcdHregPicWidthR = crate::FieldReader<u16>;
#[doc = "Field `AFBCD_HREG_PIC_WIDTH` writer - afbcd_hreg_pic_width"]
pub type AfbcdHregPicWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AFBCD_HREG_PIC_HEIGHT` reader - afbcd_hreg_pic_height"]
pub type AfbcdHregPicHeightR = crate::FieldReader<u16>;
#[doc = "Field `AFBCD_HREG_PIC_HEIGHT` writer - afbcd_hreg_pic_height"]
pub type AfbcdHregPicHeightW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - afbcd_hreg_pic_width"]
    #[inline(always)]
    pub fn afbcd_hreg_pic_width(&self) -> AfbcdHregPicWidthR {
        AfbcdHregPicWidthR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - afbcd_hreg_pic_height"]
    #[inline(always)]
    pub fn afbcd_hreg_pic_height(&self) -> AfbcdHregPicHeightR {
        AfbcdHregPicHeightR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - afbcd_hreg_pic_width"]
    #[inline(always)]
    #[must_use]
    pub fn afbcd_hreg_pic_width(&mut self) -> AfbcdHregPicWidthW<Afbcd0PicSizeSpec> {
        AfbcdHregPicWidthW::new(self, 0)
    }
    #[doc = "Bits 16:31 - afbcd_hreg_pic_height"]
    #[inline(always)]
    #[must_use]
    pub fn afbcd_hreg_pic_height(&mut self) -> AfbcdHregPicHeightW<Afbcd0PicSizeSpec> {
        AfbcdHregPicHeightW::new(self, 16)
    }
}
#[doc = "AFBCD0 pic size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afbcd0_pic_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afbcd0_pic_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afbcd0PicSizeSpec;
impl crate::RegisterSpec for Afbcd0PicSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afbcd0_pic_size::R`](R) reader structure"]
impl crate::Readable for Afbcd0PicSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`afbcd0_pic_size::W`](W) writer structure"]
impl crate::Writable for Afbcd0PicSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFBCD0_PIC_SIZE to value 0"]
impl crate::Resettable for Afbcd0PicSizeSpec {
    const RESET_VALUE: u32 = 0;
}

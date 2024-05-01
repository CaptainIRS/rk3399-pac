#[doc = "Register `DST_IMG_WIDTH_TILE1` reader"]
pub type R = crate::R<DstImgWidthTile1Spec>;
#[doc = "Register `DST_IMG_WIDTH_TILE1` writer"]
pub type W = crate::W<DstImgWidthTile1Spec>;
#[doc = "Field `DST_IMAGE_WIDTH_TILE1` reader - Destination image tile1 width"]
pub type DstImageWidthTile1R = crate::FieldReader<u16>;
#[doc = "Field `DST_IMAGE_WIDTH_TILE1` writer - Destination image tile1 width"]
pub type DstImageWidthTile1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Destination image tile1 width"]
    #[inline(always)]
    pub fn dst_image_width_tile1(&self) -> DstImageWidthTile1R {
        DstImageWidthTile1R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Destination image tile1 width"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_width_tile1(&mut self) -> DstImageWidthTile1W<DstImgWidthTile1Spec> {
        DstImageWidthTile1W::new(self, 0)
    }
}
#[doc = "Destination image tile1 width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_width_tile1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_width_tile1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstImgWidthTile1Spec;
impl crate::RegisterSpec for DstImgWidthTile1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_img_width_tile1::R`](R) reader structure"]
impl crate::Readable for DstImgWidthTile1Spec {}
#[doc = "`write(|w| ..)` method takes [`dst_img_width_tile1::W`](W) writer structure"]
impl crate::Writable for DstImgWidthTile1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_IMG_WIDTH_TILE1 to value 0"]
impl crate::Resettable for DstImgWidthTile1Spec {
    const RESET_VALUE: u32 = 0;
}

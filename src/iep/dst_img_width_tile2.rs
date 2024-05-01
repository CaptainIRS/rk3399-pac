#[doc = "Register `DST_IMG_WIDTH_TILE2` reader"]
pub type R = crate::R<DstImgWidthTile2Spec>;
#[doc = "Register `DST_IMG_WIDTH_TILE2` writer"]
pub type W = crate::W<DstImgWidthTile2Spec>;
#[doc = "Field `DST_IMAGE_WIDTH_TILE2` reader - Destination image tile2 width"]
pub type DstImageWidthTile2R = crate::FieldReader<u16>;
#[doc = "Field `DST_IMAGE_WIDTH_TILE2` writer - Destination image tile2 width"]
pub type DstImageWidthTile2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Destination image tile2 width"]
    #[inline(always)]
    pub fn dst_image_width_tile2(&self) -> DstImageWidthTile2R {
        DstImageWidthTile2R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Destination image tile2 width"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_width_tile2(&mut self) -> DstImageWidthTile2W<DstImgWidthTile2Spec> {
        DstImageWidthTile2W::new(self, 0)
    }
}
#[doc = "Destination image tile2 width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_width_tile2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_width_tile2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstImgWidthTile2Spec;
impl crate::RegisterSpec for DstImgWidthTile2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_img_width_tile2::R`](R) reader structure"]
impl crate::Readable for DstImgWidthTile2Spec {}
#[doc = "`write(|w| ..)` method takes [`dst_img_width_tile2::W`](W) writer structure"]
impl crate::Writable for DstImgWidthTile2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_IMG_WIDTH_TILE2 to value 0"]
impl crate::Resettable for DstImgWidthTile2Spec {
    const RESET_VALUE: u32 = 0;
}

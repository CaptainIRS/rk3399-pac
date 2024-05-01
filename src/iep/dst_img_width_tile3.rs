#[doc = "Register `DST_IMG_WIDTH_TILE3` reader"]
pub type R = crate::R<DstImgWidthTile3Spec>;
#[doc = "Register `DST_IMG_WIDTH_TILE3` writer"]
pub type W = crate::W<DstImgWidthTile3Spec>;
#[doc = "Field `DST_IMAGE_WIDTH_TILE3` reader - Destination image tile3 width"]
pub type DstImageWidthTile3R = crate::FieldReader<u16>;
#[doc = "Field `DST_IMAGE_WIDTH_TILE3` writer - Destination image tile3 width"]
pub type DstImageWidthTile3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Destination image tile3 width"]
    #[inline(always)]
    pub fn dst_image_width_tile3(&self) -> DstImageWidthTile3R {
        DstImageWidthTile3R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Destination image tile3 width"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_width_tile3(&mut self) -> DstImageWidthTile3W<DstImgWidthTile3Spec> {
        DstImageWidthTile3W::new(self, 0)
    }
}
#[doc = "Destination image tile3 width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_width_tile3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_width_tile3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstImgWidthTile3Spec;
impl crate::RegisterSpec for DstImgWidthTile3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_img_width_tile3::R`](R) reader structure"]
impl crate::Readable for DstImgWidthTile3Spec {}
#[doc = "`write(|w| ..)` method takes [`dst_img_width_tile3::W`](W) writer structure"]
impl crate::Writable for DstImgWidthTile3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_IMG_WIDTH_TILE3 to value 0"]
impl crate::Resettable for DstImgWidthTile3Spec {
    const RESET_VALUE: u32 = 0;
}

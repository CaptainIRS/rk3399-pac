#[doc = "Register `DST_IMG_WIDTH_TILE0` reader"]
pub type R = crate::R<DstImgWidthTile0Spec>;
#[doc = "Register `DST_IMG_WIDTH_TILE0` writer"]
pub type W = crate::W<DstImgWidthTile0Spec>;
#[doc = "Field `DST_IMAGE_WIDTH_TILE0` reader - Destination image tile0 width"]
pub type DstImageWidthTile0R = crate::FieldReader<u16>;
#[doc = "Field `DST_IMAGE_WIDTH_TILE0` writer - Destination image tile0 width"]
pub type DstImageWidthTile0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Destination image tile0 width"]
    #[inline(always)]
    pub fn dst_image_width_tile0(&self) -> DstImageWidthTile0R {
        DstImageWidthTile0R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Destination image tile0 width"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_width_tile0(&mut self) -> DstImageWidthTile0W<DstImgWidthTile0Spec> {
        DstImageWidthTile0W::new(self, 0)
    }
}
#[doc = "Destination image tile0 width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_width_tile0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_width_tile0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstImgWidthTile0Spec;
impl crate::RegisterSpec for DstImgWidthTile0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_img_width_tile0::R`](R) reader structure"]
impl crate::Readable for DstImgWidthTile0Spec {}
#[doc = "`write(|w| ..)` method takes [`dst_img_width_tile0::W`](W) writer structure"]
impl crate::Writable for DstImgWidthTile0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_IMG_WIDTH_TILE0 to value 0"]
impl crate::Resettable for DstImgWidthTile0Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DST_IMG_SIZE` reader"]
pub type R = crate::R<DstImgSizeSpec>;
#[doc = "Register `DST_IMG_SIZE` writer"]
pub type W = crate::W<DstImgSizeSpec>;
#[doc = "Field `DST_IMAGE_WIDTH` reader - Destination image width"]
pub type DstImageWidthR = crate::FieldReader<u16>;
#[doc = "Field `DST_IMAGE_WIDTH` writer - Destination image width"]
pub type DstImageWidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DST_IMAGE_HEIGHT` reader - Destination image height"]
pub type DstImageHeightR = crate::FieldReader<u16>;
#[doc = "Field `DST_IMAGE_HEIGHT` writer - Destination image height"]
pub type DstImageHeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Destination image width"]
    #[inline(always)]
    pub fn dst_image_width(&self) -> DstImageWidthR {
        DstImageWidthR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Destination image height"]
    #[inline(always)]
    pub fn dst_image_height(&self) -> DstImageHeightR {
        DstImageHeightR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Destination image width"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_width(&mut self) -> DstImageWidthW<DstImgSizeSpec> {
        DstImageWidthW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Destination image height"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_height(&mut self) -> DstImageHeightW<DstImgSizeSpec> {
        DstImageHeightW::new(self, 16)
    }
}
#[doc = "Destination image width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstImgSizeSpec;
impl crate::RegisterSpec for DstImgSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_img_size::R`](R) reader structure"]
impl crate::Readable for DstImgSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_img_size::W`](W) writer structure"]
impl crate::Writable for DstImgSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_IMG_SIZE to value 0x00ef_013f"]
impl crate::Resettable for DstImgSizeSpec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}

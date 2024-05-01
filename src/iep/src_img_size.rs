#[doc = "Register `SRC_IMG_SIZE` reader"]
pub type R = crate::R<SrcImgSizeSpec>;
#[doc = "Register `SRC_IMG_SIZE` writer"]
pub type W = crate::W<SrcImgSizeSpec>;
#[doc = "Field `SRC_IMAGE_WIDTH` reader - source image width"]
pub type SrcImageWidthR = crate::FieldReader<u16>;
#[doc = "Field `SRC_IMAGE_WIDTH` writer - source image width"]
pub type SrcImageWidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `SRC_IMAGE_HEIGHT` reader - source image height"]
pub type SrcImageHeightR = crate::FieldReader<u16>;
#[doc = "Field `SRC_IMAGE_HEIGHT` writer - source image height"]
pub type SrcImageHeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - source image width"]
    #[inline(always)]
    pub fn src_image_width(&self) -> SrcImageWidthR {
        SrcImageWidthR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - source image height"]
    #[inline(always)]
    pub fn src_image_height(&self) -> SrcImageHeightR {
        SrcImageHeightR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - source image width"]
    #[inline(always)]
    #[must_use]
    pub fn src_image_width(&mut self) -> SrcImageWidthW<SrcImgSizeSpec> {
        SrcImageWidthW::new(self, 0)
    }
    #[doc = "Bits 16:28 - source image height"]
    #[inline(always)]
    #[must_use]
    pub fn src_image_height(&mut self) -> SrcImageHeightW<SrcImgSizeSpec> {
        SrcImageHeightW::new(self, 16)
    }
}
#[doc = "Source image width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_img_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_img_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcImgSizeSpec;
impl crate::RegisterSpec for SrcImgSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_img_size::R`](R) reader structure"]
impl crate::Readable for SrcImgSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`src_img_size::W`](W) writer structure"]
impl crate::Writable for SrcImgSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_IMG_SIZE to value 0x00ef_013f"]
impl crate::Resettable for SrcImgSizeSpec {
    const RESET_VALUE: u32 = 0x00ef_013f;
}

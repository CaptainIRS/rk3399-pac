#[doc = "Register `RAW_SRC_IMG_SIZE` reader"]
pub type R = crate::R<RawSrcImgSizeSpec>;
#[doc = "Field `SRC_IMAGE_WIDTH` reader - source image width"]
pub type SrcImageWidthR = crate::FieldReader<u16>;
#[doc = "Field `SRC_IMAGE_HEIGHT` reader - source image height"]
pub type SrcImageHeightR = crate::FieldReader<u16>;
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
#[doc = "Source image width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_src_img_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawSrcImgSizeSpec;
impl crate::RegisterSpec for RawSrcImgSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_src_img_size::R`](R) reader structure"]
impl crate::Readable for RawSrcImgSizeSpec {}
#[doc = "`reset()` method sets RAW_SRC_IMG_SIZE to value 0x00f0_0140"]
impl crate::Resettable for RawSrcImgSizeSpec {
    const RESET_VALUE: u32 = 0x00f0_0140;
}

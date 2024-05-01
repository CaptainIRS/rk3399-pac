#[doc = "Register `RAW_DST_IMG_SIZE` reader"]
pub type R = crate::R<RawDstImgSizeSpec>;
#[doc = "Field `DST_IMAGE_WIDTH` reader - Destination image width"]
pub type DstImageWidthR = crate::FieldReader<u16>;
#[doc = "Field `DST_IMAGE_HEIGHT` reader - Destination image height"]
pub type DstImageHeightR = crate::FieldReader<u16>;
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
#[doc = "Destination image width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_dst_img_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawDstImgSizeSpec;
impl crate::RegisterSpec for RawDstImgSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_dst_img_size::R`](R) reader structure"]
impl crate::Readable for RawDstImgSizeSpec {}
#[doc = "`reset()` method sets RAW_DST_IMG_SIZE to value 0x00f0_0140"]
impl crate::Resettable for RawDstImgSizeSpec {
    const RESET_VALUE: u32 = 0x00f0_0140;
}

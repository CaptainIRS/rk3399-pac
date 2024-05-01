#[doc = "Register `RAW_VIR_IMG_WIDTH` reader"]
pub type R = crate::R<RawVirImgWidthSpec>;
#[doc = "Field `SRC_VIR_IMAGE_WIDTH` reader - Source virtual image width"]
pub type SrcVirImageWidthR = crate::FieldReader<u16>;
#[doc = "Field `DST_VIR_IMAGE_WIDTH` reader - Destination virtual image width"]
pub type DstVirImageWidthR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Source virtual image width"]
    #[inline(always)]
    pub fn src_vir_image_width(&self) -> SrcVirImageWidthR {
        SrcVirImageWidthR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Destination virtual image width"]
    #[inline(always)]
    pub fn dst_vir_image_width(&self) -> DstVirImageWidthR {
        DstVirImageWidthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Image virtual width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_vir_img_width::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawVirImgWidthSpec;
impl crate::RegisterSpec for RawVirImgWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_vir_img_width::R`](R) reader structure"]
impl crate::Readable for RawVirImgWidthSpec {}
#[doc = "`reset()` method sets RAW_VIR_IMG_WIDTH to value 0x0140_0140"]
impl crate::Resettable for RawVirImgWidthSpec {
    const RESET_VALUE: u32 = 0x0140_0140;
}

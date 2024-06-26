#[doc = "Register `MI_PIXEL_CNT` reader"]
pub type R = crate::R<MiPixelCntSpec>;
#[doc = "Field `pix_cnt` reader - Counter value specifies the number of pixels of the\n\ndefect pixel list generated by DPCC of the last transmitted\n\nframe. Updated at frame end.\n\nA soft reset will set the counter to zero.\n\n"]
pub type PixCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Counter value specifies the number of pixels of the\n\ndefect pixel list generated by DPCC of the last transmitted\n\nframe. Updated at frame end.\n\nA soft reset will set the counter to zero.\n\n"]
    #[inline(always)]
    pub fn pix_cnt(&self) -> PixCntR {
        PixCntR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "Counter value for defect pixel list\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_pixel_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiPixelCntSpec;
impl crate::RegisterSpec for MiPixelCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_pixel_cnt::R`](R) reader structure"]
impl crate::Readable for MiPixelCntSpec {}
#[doc = "`reset()` method sets MI_PIXEL_CNT to value 0"]
impl crate::Resettable for MiPixelCntSpec {
    const RESET_VALUE: u32 = 0;
}

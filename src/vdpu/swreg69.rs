#[doc = "Register `SWREG69` reader"]
pub type R = crate::R<Swreg69Spec>;
#[doc = "Field `SW_LUMA_SUM_INTRA` reader - sum of the luminance 8x8 intra partitons of the picture."]
pub type SwLumaSumIntraR = crate::FieldReader<u16>;
#[doc = "Field `SW_REFBUF_SUM_HIT` reader - sum of the refbufferd hits of the picture"]
pub type SwRefbufSumHitR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - sum of the luminance 8x8 intra partitons of the picture."]
    #[inline(always)]
    pub fn sw_luma_sum_intra(&self) -> SwLumaSumIntraR {
        SwLumaSumIntraR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - sum of the refbufferd hits of the picture"]
    #[inline(always)]
    pub fn sw_refbuf_sum_hit(&self) -> SwRefbufSumHitR {
        SwRefbufSumHitR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "sum information (read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg69::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg69Spec;
impl crate::RegisterSpec for Swreg69Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg69::R`](R) reader structure"]
impl crate::Readable for Swreg69Spec {}
#[doc = "`reset()` method sets SWREG69 to value 0"]
impl crate::Resettable for Swreg69Spec {
    const RESET_VALUE: u32 = 0;
}

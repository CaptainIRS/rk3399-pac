#[doc = "Register `SWREG68` reader"]
pub type R = crate::R<Swreg68Spec>;
#[doc = "Field `SW_REFBUF_SUM_BOT` reader - sum of the bottom partitions"]
pub type SwRefbufSumBotR = crate::FieldReader<u16>;
#[doc = "Field `SW_REFBUF_SUM_TOP` reader - sum of the top partitions"]
pub type SwRefbufSumTopR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - sum of the bottom partitions"]
    #[inline(always)]
    pub fn sw_refbuf_sum_bot(&self) -> SwRefbufSumBotR {
        SwRefbufSumBotR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - sum of the top partitions"]
    #[inline(always)]
    pub fn sw_refbuf_sum_top(&self) -> SwRefbufSumTopR {
        SwRefbufSumTopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "sum of partitions(read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg68::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg68Spec;
impl crate::RegisterSpec for Swreg68Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg68::R`](R) reader structure"]
impl crate::Readable for Swreg68Spec {}
#[doc = "`reset()` method sets SWREG68 to value 0"]
impl crate::Resettable for Swreg68Spec {
    const RESET_VALUE: u32 = 0;
}

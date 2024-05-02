#[doc = "Register `EXP_MEAN_32` reader"]
pub type R = crate::R<ExpMean32Spec>;
#[doc = "Field `isp_exp_mean_32` reader - (x,y)\n\n"]
pub type IspExpMean32R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_32(&self) -> IspExpMean32R {
        IspExpMean32R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_32::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean32Spec;
impl crate::RegisterSpec for ExpMean32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_32::R`](R) reader structure"]
impl crate::Readable for ExpMean32Spec {}
#[doc = "`reset()` method sets EXP_MEAN_32 to value 0"]
impl crate::Resettable for ExpMean32Spec {
    const RESET_VALUE: u32 = 0;
}

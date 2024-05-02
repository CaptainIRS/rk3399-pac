#[doc = "Register `EXP_MEAN_10` reader"]
pub type R = crate::R<ExpMean10Spec>;
#[doc = "Field `isp_exp_mean_10` reader - (x,y)\n\n"]
pub type IspExpMean10R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_10(&self) -> IspExpMean10R {
        IspExpMean10R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean10Spec;
impl crate::RegisterSpec for ExpMean10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_10::R`](R) reader structure"]
impl crate::Readable for ExpMean10Spec {}
#[doc = "`reset()` method sets EXP_MEAN_10 to value 0"]
impl crate::Resettable for ExpMean10Spec {
    const RESET_VALUE: u32 = 0;
}

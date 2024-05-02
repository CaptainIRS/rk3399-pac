#[doc = "Register `EXP_MEAN_33` reader"]
pub type R = crate::R<ExpMean33Spec>;
#[doc = "Field `isp_exp_mean_33` reader - (x,y)\n\n"]
pub type IspExpMean33R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_33(&self) -> IspExpMean33R {
        IspExpMean33R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_33::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean33Spec;
impl crate::RegisterSpec for ExpMean33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_33::R`](R) reader structure"]
impl crate::Readable for ExpMean33Spec {}
#[doc = "`reset()` method sets EXP_MEAN_33 to value 0"]
impl crate::Resettable for ExpMean33Spec {
    const RESET_VALUE: u32 = 0;
}

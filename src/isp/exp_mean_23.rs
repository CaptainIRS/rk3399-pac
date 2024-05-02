#[doc = "Register `EXP_MEAN_23` reader"]
pub type R = crate::R<ExpMean23Spec>;
#[doc = "Field `isp_exp_mean_23` reader - (x,y)\n\n"]
pub type IspExpMean23R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_23(&self) -> IspExpMean23R {
        IspExpMean23R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_23::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean23Spec;
impl crate::RegisterSpec for ExpMean23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_23::R`](R) reader structure"]
impl crate::Readable for ExpMean23Spec {}
#[doc = "`reset()` method sets EXP_MEAN_23 to value 0"]
impl crate::Resettable for ExpMean23Spec {
    const RESET_VALUE: u32 = 0;
}

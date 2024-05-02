#[doc = "Register `EXP_MEAN_43` reader"]
pub type R = crate::R<ExpMean43Spec>;
#[doc = "Field `isp_exp_mean_43` reader - (x,y)"]
pub type IspExpMean43R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)"]
    #[inline(always)]
    pub fn isp_exp_mean_43(&self) -> IspExpMean43R {
        IspExpMean43R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_43::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean43Spec;
impl crate::RegisterSpec for ExpMean43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_43::R`](R) reader structure"]
impl crate::Readable for ExpMean43Spec {}
#[doc = "`reset()` method sets EXP_MEAN_43 to value 0"]
impl crate::Resettable for ExpMean43Spec {
    const RESET_VALUE: u32 = 0;
}

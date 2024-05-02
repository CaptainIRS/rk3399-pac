#[doc = "Register `EXP_MEAN_20` reader"]
pub type R = crate::R<ExpMean20Spec>;
#[doc = "Field `isp_exp_mean_20` reader - (x,y)\n\n"]
pub type IspExpMean20R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_20(&self) -> IspExpMean20R {
        IspExpMean20R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_20::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean20Spec;
impl crate::RegisterSpec for ExpMean20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_20::R`](R) reader structure"]
impl crate::Readable for ExpMean20Spec {}
#[doc = "`reset()` method sets EXP_MEAN_20 to value 0"]
impl crate::Resettable for ExpMean20Spec {
    const RESET_VALUE: u32 = 0;
}

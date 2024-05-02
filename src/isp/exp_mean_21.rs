#[doc = "Register `EXP_MEAN_21` reader"]
pub type R = crate::R<ExpMean21Spec>;
#[doc = "Field `isp_exp_mean_21` reader - (x,y)"]
pub type IspExpMean21R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)"]
    #[inline(always)]
    pub fn isp_exp_mean_21(&self) -> IspExpMean21R {
        IspExpMean21R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_21::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean21Spec;
impl crate::RegisterSpec for ExpMean21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_21::R`](R) reader structure"]
impl crate::Readable for ExpMean21Spec {}
#[doc = "`reset()` method sets EXP_MEAN_21 to value 0"]
impl crate::Resettable for ExpMean21Spec {
    const RESET_VALUE: u32 = 0;
}

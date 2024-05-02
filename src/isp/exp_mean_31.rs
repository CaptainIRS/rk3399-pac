#[doc = "Register `EXP_MEAN_31` reader"]
pub type R = crate::R<ExpMean31Spec>;
#[doc = "Field `isp_exp_mean_31` reader - (x,y)\n\n"]
pub type IspExpMean31R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_31(&self) -> IspExpMean31R {
        IspExpMean31R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_31::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean31Spec;
impl crate::RegisterSpec for ExpMean31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_31::R`](R) reader structure"]
impl crate::Readable for ExpMean31Spec {}
#[doc = "`reset()` method sets EXP_MEAN_31 to value 0"]
impl crate::Resettable for ExpMean31Spec {
    const RESET_VALUE: u32 = 0;
}

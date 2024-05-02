#[doc = "Register `EXP_MEAN_24` reader"]
pub type R = crate::R<ExpMean24Spec>;
#[doc = "Field `isp_exp_mean_24` reader - (x,y)\n\n"]
pub type IspExpMean24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_24(&self) -> IspExpMean24R {
        IspExpMean24R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_24::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean24Spec;
impl crate::RegisterSpec for ExpMean24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_24::R`](R) reader structure"]
impl crate::Readable for ExpMean24Spec {}
#[doc = "`reset()` method sets EXP_MEAN_24 to value 0"]
impl crate::Resettable for ExpMean24Spec {
    const RESET_VALUE: u32 = 0;
}

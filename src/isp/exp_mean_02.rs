#[doc = "Register `EXP_MEAN_02` reader"]
pub type R = crate::R<ExpMean02Spec>;
#[doc = "Field `isp_exp_mean_02` reader - (x,y)\n\n"]
pub type IspExpMean02R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_02(&self) -> IspExpMean02R {
        IspExpMean02R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_02::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean02Spec;
impl crate::RegisterSpec for ExpMean02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_02::R`](R) reader structure"]
impl crate::Readable for ExpMean02Spec {}
#[doc = "`reset()` method sets EXP_MEAN_02 to value 0"]
impl crate::Resettable for ExpMean02Spec {
    const RESET_VALUE: u32 = 0;
}

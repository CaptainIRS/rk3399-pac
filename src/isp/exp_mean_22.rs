#[doc = "Register `EXP_MEAN_22` reader"]
pub type R = crate::R<ExpMean22Spec>;
#[doc = "Field `isp_exp_mean_22` reader - (x,y)\n\n"]
pub type IspExpMean22R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_22(&self) -> IspExpMean22R {
        IspExpMean22R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_22::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean22Spec;
impl crate::RegisterSpec for ExpMean22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_22::R`](R) reader structure"]
impl crate::Readable for ExpMean22Spec {}
#[doc = "`reset()` method sets EXP_MEAN_22 to value 0"]
impl crate::Resettable for ExpMean22Spec {
    const RESET_VALUE: u32 = 0;
}

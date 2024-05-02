#[doc = "Register `EXP_MEAN_04` reader"]
pub type R = crate::R<ExpMean04Spec>;
#[doc = "Field `isp_exp_mean_04` reader - (x,y)\n\n"]
pub type IspExpMean04R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_04(&self) -> IspExpMean04R {
        IspExpMean04R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 04\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_04::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean04Spec;
impl crate::RegisterSpec for ExpMean04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_04::R`](R) reader structure"]
impl crate::Readable for ExpMean04Spec {}
#[doc = "`reset()` method sets EXP_MEAN_04 to value 0"]
impl crate::Resettable for ExpMean04Spec {
    const RESET_VALUE: u32 = 0;
}

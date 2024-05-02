#[doc = "Register `EXP_MEAN_00` reader"]
pub type R = crate::R<ExpMean00Spec>;
#[doc = "Field `isp_exp_mean_00` reader - (x,y)\n\n"]
pub type IspExpMean00R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_00(&self) -> IspExpMean00R {
        IspExpMean00R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_00::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean00Spec;
impl crate::RegisterSpec for ExpMean00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_00::R`](R) reader structure"]
impl crate::Readable for ExpMean00Spec {}
#[doc = "`reset()` method sets EXP_MEAN_00 to value 0"]
impl crate::Resettable for ExpMean00Spec {
    const RESET_VALUE: u32 = 0;
}

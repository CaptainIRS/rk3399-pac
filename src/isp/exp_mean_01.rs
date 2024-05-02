#[doc = "Register `EXP_MEAN_01` reader"]
pub type R = crate::R<ExpMean01Spec>;
#[doc = "Field `isp_exp_mean_01` reader - (x,y)\n\n"]
pub type IspExpMean01R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_01(&self) -> IspExpMean01R {
        IspExpMean01R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_01::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean01Spec;
impl crate::RegisterSpec for ExpMean01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_01::R`](R) reader structure"]
impl crate::Readable for ExpMean01Spec {}
#[doc = "`reset()` method sets EXP_MEAN_01 to value 0"]
impl crate::Resettable for ExpMean01Spec {
    const RESET_VALUE: u32 = 0;
}

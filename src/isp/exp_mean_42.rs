#[doc = "Register `EXP_MEAN_42` reader"]
pub type R = crate::R<ExpMean42Spec>;
#[doc = "Field `isp_exp_mean_42` reader - (x,y)\n\n"]
pub type IspExpMean42R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_42(&self) -> IspExpMean42R {
        IspExpMean42R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_42::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean42Spec;
impl crate::RegisterSpec for ExpMean42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_42::R`](R) reader structure"]
impl crate::Readable for ExpMean42Spec {}
#[doc = "`reset()` method sets EXP_MEAN_42 to value 0"]
impl crate::Resettable for ExpMean42Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `EXP_MEAN_30` reader"]
pub type R = crate::R<ExpMean30Spec>;
#[doc = "Field `isp_exp_mean_30` reader - (x,y)"]
pub type IspExpMean30R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)"]
    #[inline(always)]
    pub fn isp_exp_mean_30(&self) -> IspExpMean30R {
        IspExpMean30R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_30::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean30Spec;
impl crate::RegisterSpec for ExpMean30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_30::R`](R) reader structure"]
impl crate::Readable for ExpMean30Spec {}
#[doc = "`reset()` method sets EXP_MEAN_30 to value 0"]
impl crate::Resettable for ExpMean30Spec {
    const RESET_VALUE: u32 = 0;
}

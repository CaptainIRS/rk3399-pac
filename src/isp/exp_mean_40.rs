#[doc = "Register `EXP_MEAN_40` reader"]
pub type R = crate::R<ExpMean40Spec>;
#[doc = "Field `isp_exp_mean_40` reader - (x,y)\n\n"]
pub type IspExpMean40R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_40(&self) -> IspExpMean40R {
        IspExpMean40R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_40::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean40Spec;
impl crate::RegisterSpec for ExpMean40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_40::R`](R) reader structure"]
impl crate::Readable for ExpMean40Spec {}
#[doc = "`reset()` method sets EXP_MEAN_40 to value 0"]
impl crate::Resettable for ExpMean40Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `EXP_MEAN_44` reader"]
pub type R = crate::R<ExpMean44Spec>;
#[doc = "Field `isp_exp_mean_44` reader - (x,y)\n\n"]
pub type IspExpMean44R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_44(&self) -> IspExpMean44R {
        IspExpMean44R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_44::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean44Spec;
impl crate::RegisterSpec for ExpMean44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_44::R`](R) reader structure"]
impl crate::Readable for ExpMean44Spec {}
#[doc = "`reset()` method sets EXP_MEAN_44 to value 0"]
impl crate::Resettable for ExpMean44Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `EXP_MEAN_11` reader"]
pub type R = crate::R<ExpMean11Spec>;
#[doc = "Field `isp_exp_mean_11` reader - (x,y)\n\n"]
pub type IspExpMean11R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_11(&self) -> IspExpMean11R {
        IspExpMean11R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean11Spec;
impl crate::RegisterSpec for ExpMean11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_11::R`](R) reader structure"]
impl crate::Readable for ExpMean11Spec {}
#[doc = "`reset()` method sets EXP_MEAN_11 to value 0"]
impl crate::Resettable for ExpMean11Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `EXP_MEAN_03` reader"]
pub type R = crate::R<ExpMean03Spec>;
#[doc = "Field `isp_exp_mean_03` reader - (x,y)"]
pub type IspExpMean03R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)"]
    #[inline(always)]
    pub fn isp_exp_mean_03(&self) -> IspExpMean03R {
        IspExpMean03R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_03::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean03Spec;
impl crate::RegisterSpec for ExpMean03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_03::R`](R) reader structure"]
impl crate::Readable for ExpMean03Spec {}
#[doc = "`reset()` method sets EXP_MEAN_03 to value 0"]
impl crate::Resettable for ExpMean03Spec {
    const RESET_VALUE: u32 = 0;
}

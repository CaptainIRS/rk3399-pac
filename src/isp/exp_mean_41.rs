#[doc = "Register `EXP_MEAN_41` reader"]
pub type R = crate::R<ExpMean41Spec>;
#[doc = "Field `isp_exp_mean_41` reader - (x,y)\n\n"]
pub type IspExpMean41R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - (x,y)\n\n"]
    #[inline(always)]
    pub fn isp_exp_mean_41(&self) -> IspExpMean41R {
        IspExpMean41R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Mean luminance value of block 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_mean_41::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpMean41Spec;
impl crate::RegisterSpec for ExpMean41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_mean_41::R`](R) reader structure"]
impl crate::Readable for ExpMean41Spec {}
#[doc = "`reset()` method sets EXP_MEAN_41 to value 0"]
impl crate::Resettable for ExpMean41Spec {
    const RESET_VALUE: u32 = 0;
}

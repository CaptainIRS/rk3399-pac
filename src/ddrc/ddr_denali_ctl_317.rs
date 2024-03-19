#[doc = "Register `DDR_DENALI_CTL_317` reader"]
pub type R = crate::R<DdrDenaliCtl317Spec>;
#[doc = "Field `USER_DEF_REG_RO_0` reader - User-defined input register 0. READ-ONLY"]
pub type UserDefRegRo0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined input register 0. READ-ONLY"]
    #[inline(always)]
    pub fn user_def_reg_ro_0(&self) -> UserDefRegRo0R {
        UserDefRegRo0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_317::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl317Spec;
impl crate::RegisterSpec for DdrDenaliCtl317Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_317::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl317Spec {}
#[doc = "`reset()` method sets DDR_DENALI_CTL_317 to value 0"]
impl crate::Resettable for DdrDenaliCtl317Spec {
    const RESET_VALUE: u32 = 0;
}

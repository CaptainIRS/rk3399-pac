#[doc = "Register `DDR_PI_REG_174` reader"]
pub type R = crate::R<DdrPiReg174Spec>;
#[doc = "Field `PI_INT_STATUS` reader - Indicates status of interrupt features in the PI."]
pub type PiIntStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 8:25 - Indicates status of interrupt features in the PI."]
    #[inline(always)]
    pub fn pi_int_status(&self) -> PiIntStatusR {
        PiIntStatusR::new((self.bits >> 8) & 0x0003_ffff)
    }
}
#[doc = "DDR PHY Independent Register 174\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_174::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg174Spec;
impl crate::RegisterSpec for DdrPiReg174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_174::R`](R) reader structure"]
impl crate::Readable for DdrPiReg174Spec {}
#[doc = "`reset()` method sets DDR_PI_REG_174 to value 0"]
impl crate::Resettable for DdrPiReg174Spec {
    const RESET_VALUE: u32 = 0;
}

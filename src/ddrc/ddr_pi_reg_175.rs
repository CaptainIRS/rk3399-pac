#[doc = "Register `DDR_PI_REG_175` writer"]
pub type W = crate::W<DdrPiReg175Spec>;
#[doc = "Field `PI_INT_ACK` writer - Clears the corresponding interrupt bit of the\n\nPI_REG_174.pi_int_status parameter."]
pub type PiIntAckW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl W {
    #[doc = "Bits 0:17 - Clears the corresponding interrupt bit of the\n\nPI_REG_174.pi_int_status parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_int_ack(&mut self) -> PiIntAckW<DdrPiReg175Spec> {
        PiIntAckW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 175\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_175::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg175Spec;
impl crate::RegisterSpec for DdrPiReg175Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_175::W`](W) writer structure"]
impl crate::Writable for DdrPiReg175Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_175 to value 0"]
impl crate::Resettable for DdrPiReg175Spec {
    const RESET_VALUE: u32 = 0;
}

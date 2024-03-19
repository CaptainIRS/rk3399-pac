#[doc = "Register `DDR_PI_REG_100` reader"]
pub type R = crate::R<DdrPiReg100Spec>;
#[doc = "Register `DDR_PI_REG_100` writer"]
pub type W = crate::W<DdrPiReg100Spec>;
#[doc = "Field `PI_CALVL_RESP_MASK` reader - Indicates mask for the dfi_calvl_resp signal during CA training."]
pub type PiCalvlRespMaskR = crate::BitReader;
#[doc = "Field `PI_CALVL_RESP_MASK` writer - Indicates mask for the dfi_calvl_resp signal during CA training."]
pub type PiCalvlRespMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_EN` reader - Enables the PI CA training module. Bit1 represents the support\n\nwhen non-initialization. Bit0 represents the support when\n\ninitialization. Set to 1 to enable."]
pub type PiCalvlEnR = crate::FieldReader;
#[doc = "Field `PI_CALVL_EN` writer - Enables the PI CA training module. Bit1 represents the support\n\nwhen non-initialization. Bit0 represents the support when\n\ninitialization. Set to 1 to enable."]
pub type PiCalvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_CALVL_ERROR_STATUS` reader - Holds the error that is associated with the CA training error\n\ninterrupt. Bit0 set indicates a PI_REG_98.pi_tdfi_calvl_resp\n\nparameter violation and bit1 set indicates a\n\nPI_REG_99.pi_tdfi_calvl_max parameter violation."]
pub type PiCalvlErrorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicates mask for the dfi_calvl_resp signal during CA training."]
    #[inline(always)]
    pub fn pi_calvl_resp_mask(&self) -> PiCalvlRespMaskR {
        PiCalvlRespMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Enables the PI CA training module. Bit1 represents the support\n\nwhen non-initialization. Bit0 represents the support when\n\ninitialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_calvl_en(&self) -> PiCalvlEnR {
        PiCalvlEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Holds the error that is associated with the CA training error\n\ninterrupt. Bit0 set indicates a PI_REG_98.pi_tdfi_calvl_resp\n\nparameter violation and bit1 set indicates a\n\nPI_REG_99.pi_tdfi_calvl_max parameter violation."]
    #[inline(always)]
    pub fn pi_calvl_error_status(&self) -> PiCalvlErrorStatusR {
        PiCalvlErrorStatusR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates mask for the dfi_calvl_resp signal during CA training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_resp_mask(&mut self) -> PiCalvlRespMaskW<DdrPiReg100Spec> {
        PiCalvlRespMaskW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Enables the PI CA training module. Bit1 represents the support\n\nwhen non-initialization. Bit0 represents the support when\n\ninitialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_en(&mut self) -> PiCalvlEnW<DdrPiReg100Spec> {
        PiCalvlEnW::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_100::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_100::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg100Spec;
impl crate::RegisterSpec for DdrPiReg100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_100::R`](R) reader structure"]
impl crate::Readable for DdrPiReg100Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_100::W`](W) writer structure"]
impl crate::Writable for DdrPiReg100Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_100 to value 0"]
impl crate::Resettable for DdrPiReg100Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DDR_PI_REG_70` reader"]
pub type R = crate::R<DdrPiReg70Spec>;
#[doc = "Register `DDR_PI_REG_70` writer"]
pub type W = crate::W<DdrPiReg70Spec>;
#[doc = "Field `PI_EN_ODT_ASSERT_EXCEPT_RD` reader - Enables controller to assert ODT at all times except during reads. Assumes a single ODT pin is connected. Set to 1 to enable."]
pub type PiEnOdtAssertExceptRdR = crate::BitReader;
#[doc = "Field `PI_EN_ODT_ASSERT_EXCEPT_RD` writer - Enables controller to assert ODT at all times except during reads. Assumes a single ODT pin is connected. Set to 1 to enable."]
pub type PiEnOdtAssertExceptRdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - Enables controller to assert ODT at all times except during reads. Assumes a single ODT pin is connected. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_en_odt_assert_except_rd(&self) -> PiEnOdtAssertExceptRdR {
        PiEnOdtAssertExceptRdR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Enables controller to assert ODT at all times except during reads. Assumes a single ODT pin is connected. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_en_odt_assert_except_rd(&mut self) -> PiEnOdtAssertExceptRdW<DdrPiReg70Spec> {
        PiEnOdtAssertExceptRdW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 70\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_70::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_70::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg70Spec;
impl crate::RegisterSpec for DdrPiReg70Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_70::R`](R) reader structure"]
impl crate::Readable for DdrPiReg70Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_70::W`](W) writer structure"]
impl crate::Writable for DdrPiReg70Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_70 to value 0"]
impl crate::Resettable for DdrPiReg70Spec {
    const RESET_VALUE: u32 = 0;
}

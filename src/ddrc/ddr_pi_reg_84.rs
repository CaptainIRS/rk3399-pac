#[doc = "Register `DDR_PI_REG_84` reader"]
pub type R = crate::R<DdrPiReg84Spec>;
#[doc = "Register `DDR_PI_REG_84` writer"]
pub type W = crate::W<DdrPiReg84Spec>;
#[doc = "Field `PI_RDLVL_STROBE_NUM` reader - Defines the number of back to back MPC command in one read\n\nprocess in read eye training."]
pub type PiRdlvlStrobeNumR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_STROBE_NUM` writer - Defines the number of back to back MPC command in one read\n\nprocess in read eye training."]
pub type PiRdlvlStrobeNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_RDLVL_GATE_STROBE_NUM` reader - Defines the number of back-to-back MPC command in one read\n\nprocess in read gate training."]
pub type PiRdlvlGateStrobeNumR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_GATE_STROBE_NUM` writer - Defines the number of back-to-back MPC command in one read\n\nprocess in read gate training."]
pub type PiRdlvlGateStrobeNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Defines the number of back to back MPC command in one read\n\nprocess in read eye training."]
    #[inline(always)]
    pub fn pi_rdlvl_strobe_num(&self) -> PiRdlvlStrobeNumR {
        PiRdlvlStrobeNumR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Defines the number of back-to-back MPC command in one read\n\nprocess in read gate training."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_strobe_num(&self) -> PiRdlvlGateStrobeNumR {
        PiRdlvlGateStrobeNumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Defines the number of back to back MPC command in one read\n\nprocess in read eye training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_strobe_num(&mut self) -> PiRdlvlStrobeNumW<DdrPiReg84Spec> {
        PiRdlvlStrobeNumW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Defines the number of back-to-back MPC command in one read\n\nprocess in read gate training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_strobe_num(&mut self) -> PiRdlvlGateStrobeNumW<DdrPiReg84Spec> {
        PiRdlvlGateStrobeNumW::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_84::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg84Spec;
impl crate::RegisterSpec for DdrPiReg84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_84::R`](R) reader structure"]
impl crate::Readable for DdrPiReg84Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_84::W`](W) writer structure"]
impl crate::Writable for DdrPiReg84Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_84 to value 0"]
impl crate::Resettable for DdrPiReg84Spec {
    const RESET_VALUE: u32 = 0;
}

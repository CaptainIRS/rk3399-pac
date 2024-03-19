#[doc = "Register `DDR_PI_REG_56` reader"]
pub type R = crate::R<DdrPiReg56Spec>;
#[doc = "Register `DDR_PI_REG_56` writer"]
pub type W = crate::W<DdrPiReg56Spec>;
#[doc = "Field `PI_SW_WDQLVL_RESP_3` reader - Indicates WDQ leveling response for data slice 3."]
pub type PiSwWdqlvlResp3R = crate::FieldReader;
#[doc = "Field `PI_SW_WDQLVL_VREF` reader - Indicates user-defined WDQ vref value in software leveling debug mode."]
pub type PiSwWdqlvlVrefR = crate::FieldReader;
#[doc = "Field `PI_SW_WDQLVL_VREF` writer - Indicates user-defined WDQ vref value in software leveling debug mode."]
pub type PiSwWdqlvlVrefW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_SWLVL_SM2_START` writer - Indicates software leveling start command for stage 2."]
pub type PiSwlvlSm2StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SWLVL_SM2_WR` writer - Indicates software leveling write command for stage 2."]
pub type PiSwlvlSm2WrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Indicates WDQ leveling response for data slice 3."]
    #[inline(always)]
    pub fn pi_sw_wdqlvl_resp_3(&self) -> PiSwWdqlvlResp3R {
        PiSwWdqlvlResp3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:14 - Indicates user-defined WDQ vref value in software leveling debug mode."]
    #[inline(always)]
    pub fn pi_sw_wdqlvl_vref(&self) -> PiSwWdqlvlVrefR {
        PiSwWdqlvlVrefR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - Indicates user-defined WDQ vref value in software leveling debug mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_wdqlvl_vref(&mut self) -> PiSwWdqlvlVrefW<DdrPiReg56Spec> {
        PiSwWdqlvlVrefW::new(self, 8)
    }
    #[doc = "Bit 16 - Indicates software leveling start command for stage 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_sm2_start(&mut self) -> PiSwlvlSm2StartW<DdrPiReg56Spec> {
        PiSwlvlSm2StartW::new(self, 16)
    }
    #[doc = "Bit 24 - Indicates software leveling write command for stage 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_sm2_wr(&mut self) -> PiSwlvlSm2WrW<DdrPiReg56Spec> {
        PiSwlvlSm2WrW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg56Spec;
impl crate::RegisterSpec for DdrPiReg56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_56::R`](R) reader structure"]
impl crate::Readable for DdrPiReg56Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_56::W`](W) writer structure"]
impl crate::Writable for DdrPiReg56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_56 to value 0"]
impl crate::Resettable for DdrPiReg56Spec {
    const RESET_VALUE: u32 = 0;
}

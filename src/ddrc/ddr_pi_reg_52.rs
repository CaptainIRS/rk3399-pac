#[doc = "Register `DDR_PI_REG_52` reader"]
pub type R = crate::R<DdrPiReg52Spec>;
#[doc = "Register `DDR_PI_REG_52` writer"]
pub type W = crate::W<DdrPiReg52Spec>;
#[doc = "Field `PI_SW_RDLVL_RESP_3` reader - Indicates read leveling response for data slice 3."]
pub type PiSwRdlvlResp3R = crate::FieldReader;
#[doc = "Field `PI_SW_CALVL_RESP_0` reader - Indicates CA leveling response for data slice 0."]
pub type PiSwCalvlResp0R = crate::FieldReader;
#[doc = "Field `PI_SW_LEVELING_MODE` reader - Defines the leveling operation for software leveling. Set to 3'b001 for write leveling, set to 3'b010 for read data eye training, set to 3'b011 for read gate training, set to 3'b100 for ca training, set to 3'b101 for WDQ training."]
pub type PiSwLevelingModeR = crate::FieldReader;
#[doc = "Field `PI_SW_LEVELING_MODE` writer - Defines the leveling operation for software leveling. Set to 3'b001 for write leveling, set to 3'b010 for read data eye training, set to 3'b011 for read gate training, set to 3'b100 for ca training, set to 3'b101 for WDQ training."]
pub type PiSwLevelingModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PI_SWLVL_START` writer - Indicates user request to initiate software leveling of type in the PI_REG_52.pi_sw_leveling_mode parameter. Set to 1 to trigger."]
pub type PiSwlvlStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Indicates read leveling response for data slice 3."]
    #[inline(always)]
    pub fn pi_sw_rdlvl_resp_3(&self) -> PiSwRdlvlResp3R {
        PiSwRdlvlResp3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Indicates CA leveling response for data slice 0."]
    #[inline(always)]
    pub fn pi_sw_calvl_resp_0(&self) -> PiSwCalvlResp0R {
        PiSwCalvlResp0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Defines the leveling operation for software leveling. Set to 3'b001 for write leveling, set to 3'b010 for read data eye training, set to 3'b011 for read gate training, set to 3'b100 for ca training, set to 3'b101 for WDQ training."]
    #[inline(always)]
    pub fn pi_sw_leveling_mode(&self) -> PiSwLevelingModeR {
        PiSwLevelingModeR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Defines the leveling operation for software leveling. Set to 3'b001 for write leveling, set to 3'b010 for read data eye training, set to 3'b011 for read gate training, set to 3'b100 for ca training, set to 3'b101 for WDQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_leveling_mode(&mut self) -> PiSwLevelingModeW<DdrPiReg52Spec> {
        PiSwLevelingModeW::new(self, 16)
    }
    #[doc = "Bit 24 - Indicates user request to initiate software leveling of type in the PI_REG_52.pi_sw_leveling_mode parameter. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_start(&mut self) -> PiSwlvlStartW<DdrPiReg52Spec> {
        PiSwlvlStartW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_52::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg52Spec;
impl crate::RegisterSpec for DdrPiReg52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_52::R`](R) reader structure"]
impl crate::Readable for DdrPiReg52Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_52::W`](W) writer structure"]
impl crate::Writable for DdrPiReg52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_52 to value 0"]
impl crate::Resettable for DdrPiReg52Spec {
    const RESET_VALUE: u32 = 0;
}

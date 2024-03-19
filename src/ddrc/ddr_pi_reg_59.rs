#[doc = "Register `DDR_PI_REG_59` reader"]
pub type R = crate::R<DdrPiReg59Spec>;
#[doc = "Register `DDR_PI_REG_59` writer"]
pub type W = crate::W<DdrPiReg59Spec>;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F2` reader - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCtrlDelayF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F2` writer - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCtrlDelayF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_WRLVL_REQ` writer - Indicates user request to initiate write leveling. Set to 1 to trigger."]
pub type PiWrlvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WRLVL_CS` reader - Specifies the target chip select for the write leveling operation that is initiated through the PI_REG_59.pi_wrlvl_req parameter."]
pub type PiWrlvlCsR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_CS` writer - Specifies the target chip select for the write leveling operation that is initiated through the PI_REG_59.pi_wrlvl_req parameter."]
pub type PiWrlvlCsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WLDQSEN` reader - Indicates delay from the issuing MRS to the first DQS strobe for write leveling."]
pub type PiWldqsenR = crate::FieldReader;
#[doc = "Field `PI_WLDQSEN` writer - Indicates delay from the issuing MRS to the first DQS strobe for write leveling."]
pub type PiWldqsenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_ctrl_delay_f2(&self) -> PiTdfiCtrlDelayF2R {
        PiTdfiCtrlDelayF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Specifies the target chip select for the write leveling operation that is initiated through the PI_REG_59.pi_wrlvl_req parameter."]
    #[inline(always)]
    pub fn pi_wrlvl_cs(&self) -> PiWrlvlCsR {
        PiWrlvlCsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Indicates delay from the issuing MRS to the first DQS strobe for write leveling."]
    #[inline(always)]
    pub fn pi_wldqsen(&self) -> PiWldqsenR {
        PiWldqsenR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrl_delay_f2(&mut self) -> PiTdfiCtrlDelayF2W<DdrPiReg59Spec> {
        PiTdfiCtrlDelayF2W::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates user request to initiate write leveling. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_req(&mut self) -> PiWrlvlReqW<DdrPiReg59Spec> {
        PiWrlvlReqW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Specifies the target chip select for the write leveling operation that is initiated through the PI_REG_59.pi_wrlvl_req parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_cs(&mut self) -> PiWrlvlCsW<DdrPiReg59Spec> {
        PiWrlvlCsW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Indicates delay from the issuing MRS to the first DQS strobe for write leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wldqsen(&mut self) -> PiWldqsenW<DdrPiReg59Spec> {
        PiWldqsenW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_59::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_59::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg59Spec;
impl crate::RegisterSpec for DdrPiReg59Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_59::R`](R) reader structure"]
impl crate::Readable for DdrPiReg59Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_59::W`](W) writer structure"]
impl crate::Writable for DdrPiReg59Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_59 to value 0"]
impl crate::Resettable for DdrPiReg59Spec {
    const RESET_VALUE: u32 = 0;
}

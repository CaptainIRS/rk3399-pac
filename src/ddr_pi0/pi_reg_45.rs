#[doc = "Register `PI_REG_45` reader"]
pub type R = crate::R<PiReg45Spec>;
#[doc = "Register `PI_REG_45` writer"]
pub type W = crate::W<PiReg45Spec>;
#[doc = "Field `PI_CASLAT_LIN_F2` reader - Sets latency from read command send to data receive from/to controller. Bit0 is half-cycle increment and the upper bits define memory CAS latency for the controller. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiCaslatLinF2R = crate::FieldReader;
#[doc = "Field `PI_CASLAT_LIN_F2` writer - Sets latency from read command send to data receive from/to controller. Bit0 is half-cycle increment and the upper bits define memory CAS latency for the controller. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiCaslatLinF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_PREAMBLE_SUPPORT` reader - Indicates selection of one or two-cycle preamble for read and write burst transfers."]
pub type PiPreambleSupportR = crate::FieldReader;
#[doc = "Field `PI_PREAMBLE_SUPPORT` writer - Indicates selection of one or two-cycle preamble for read and write burst transfers."]
pub type PiPreambleSupportW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_AREFRESH` writer - Initiates auto-refresh at the end of the current burst boundary. Set to 1 to trigger."]
pub type PiArefreshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MCAREF_FORWARD_ONLY` reader - Pass mc auto refresh command to DRAM device, not generate auto-refresh by PI. It is recommended to set to 1."]
pub type PiMcarefForwardOnlyR = crate::BitReader;
#[doc = "Field `PI_MCAREF_FORWARD_ONLY` writer - Pass mc auto refresh command to DRAM device, not generate auto-refresh by PI. It is recommended to set to 1."]
pub type PiMcarefForwardOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Sets latency from read command send to data receive from/to controller. Bit0 is half-cycle increment and the upper bits define memory CAS latency for the controller. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_caslat_lin_f2(&self) -> PiCaslatLinF2R {
        PiCaslatLinF2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Indicates selection of one or two-cycle preamble for read and write burst transfers."]
    #[inline(always)]
    pub fn pi_preamble_support(&self) -> PiPreambleSupportR {
        PiPreambleSupportR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 24 - Pass mc auto refresh command to DRAM device, not generate auto-refresh by PI. It is recommended to set to 1."]
    #[inline(always)]
    pub fn pi_mcaref_forward_only(&self) -> PiMcarefForwardOnlyR {
        PiMcarefForwardOnlyR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets latency from read command send to data receive from/to controller. Bit0 is half-cycle increment and the upper bits define memory CAS latency for the controller. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_caslat_lin_f2(&mut self) -> PiCaslatLinF2W<PiReg45Spec> {
        PiCaslatLinF2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Indicates selection of one or two-cycle preamble for read and write burst transfers."]
    #[inline(always)]
    #[must_use]
    pub fn pi_preamble_support(&mut self) -> PiPreambleSupportW<PiReg45Spec> {
        PiPreambleSupportW::new(self, 8)
    }
    #[doc = "Bit 16 - Initiates auto-refresh at the end of the current burst boundary. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn pi_arefresh(&mut self) -> PiArefreshW<PiReg45Spec> {
        PiArefreshW::new(self, 16)
    }
    #[doc = "Bit 24 - Pass mc auto refresh command to DRAM device, not generate auto-refresh by PI. It is recommended to set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mcaref_forward_only(&mut self) -> PiMcarefForwardOnlyW<PiReg45Spec> {
        PiMcarefForwardOnlyW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg45Spec;
impl crate::RegisterSpec for PiReg45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_45::R`](R) reader structure"]
impl crate::Readable for PiReg45Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_45::W`](W) writer structure"]
impl crate::Writable for PiReg45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_45 to value 0"]
impl crate::Resettable for PiReg45Spec {
    const RESET_VALUE: u32 = 0;
}

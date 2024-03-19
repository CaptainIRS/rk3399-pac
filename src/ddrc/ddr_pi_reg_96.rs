#[doc = "Register `DDR_PI_REG_96` reader"]
pub type R = crate::R<DdrPiReg96Spec>;
#[doc = "Register `DDR_PI_REG_96` writer"]
pub type W = crate::W<DdrPiReg96Spec>;
#[doc = "Field `PI_TDFI_CALVL_CC_F1` reader - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlCcF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CALVL_CC_F1` writer - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlCcF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TDFI_CALVL_CAPTURE_F1` reader - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlCaptureF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CALVL_CAPTURE_F1` writer - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlCaptureF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_cc_f1(&self) -> PiTdfiCalvlCcF1R {
        PiTdfiCalvlCcF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_capture_f1(&self) -> PiTdfiCalvlCaptureF1R {
        PiTdfiCalvlCaptureF1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_cc_f1(&mut self) -> PiTdfiCalvlCcF1W<DdrPiReg96Spec> {
        PiTdfiCalvlCcF1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_capture_f1(&mut self) -> PiTdfiCalvlCaptureF1W<DdrPiReg96Spec> {
        PiTdfiCalvlCaptureF1W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_96::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_96::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg96Spec;
impl crate::RegisterSpec for DdrPiReg96Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_96::R`](R) reader structure"]
impl crate::Readable for DdrPiReg96Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_96::W`](W) writer structure"]
impl crate::Writable for DdrPiReg96Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_96 to value 0"]
impl crate::Resettable for DdrPiReg96Spec {
    const RESET_VALUE: u32 = 0;
}

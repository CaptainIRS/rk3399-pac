#[doc = "Register `DDR_PI_REG_97` reader"]
pub type R = crate::R<DdrPiReg97Spec>;
#[doc = "Register `DDR_PI_REG_97` writer"]
pub type W = crate::W<DdrPiReg97Spec>;
#[doc = "Field `PI_TDFI_CALVL_CC_F2` reader - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the\n\nminimum cycles between calibration commands. The suffix \"_f2\" of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlCcF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CALVL_CC_F2` writer - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the\n\nminimum cycles between calibration commands. The suffix \"_f2\" of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlCcF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TDFI_CALVL_CAPTURE_F2` reader - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI\n\nclocks), the minimum cycles between a calibration command and a\n\ndfi_calvl_capture pulse. The suffix \"_f2\" of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdfiCalvlCaptureF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CALVL_CAPTURE_F2` writer - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI\n\nclocks), the minimum cycles between a calibration command and a\n\ndfi_calvl_capture pulse. The suffix \"_f2\" of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdfiCalvlCaptureF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the\n\nminimum cycles between calibration commands. The suffix \"_f2\" of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_cc_f2(&self) -> PiTdfiCalvlCcF2R {
        PiTdfiCalvlCcF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI\n\nclocks), the minimum cycles between a calibration command and a\n\ndfi_calvl_capture pulse. The suffix \"_f2\" of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_capture_f2(&self) -> PiTdfiCalvlCaptureF2R {
        PiTdfiCalvlCaptureF2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the\n\nminimum cycles between calibration commands. The suffix \"_f2\" of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_cc_f2(&mut self) -> PiTdfiCalvlCcF2W<DdrPiReg97Spec> {
        PiTdfiCalvlCcF2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI\n\nclocks), the minimum cycles between a calibration command and a\n\ndfi_calvl_capture pulse. The suffix \"_f2\" of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_capture_f2(&mut self) -> PiTdfiCalvlCaptureF2W<DdrPiReg97Spec> {
        PiTdfiCalvlCaptureF2W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 97\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_97::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_97::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg97Spec;
impl crate::RegisterSpec for DdrPiReg97Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_97::R`](R) reader structure"]
impl crate::Readable for DdrPiReg97Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_97::W`](W) writer structure"]
impl crate::Writable for DdrPiReg97Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_97 to value 0"]
impl crate::Resettable for DdrPiReg97Spec {
    const RESET_VALUE: u32 = 0;
}

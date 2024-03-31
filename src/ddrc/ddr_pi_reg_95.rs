#[doc = "Register `DDR_PI_REG_95` reader"]
pub type R = crate::R<DdrPiReg95Spec>;
#[doc = "Register `DDR_PI_REG_95` writer"]
pub type W = crate::W<DdrPiReg95Spec>;
#[doc = "Field `PI_TDFI_CALVL_CC_F0` reader - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the\n\nminimum cycles between calibration commands. The suffix '_f0' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlCcF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CALVL_CC_F0` writer - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the\n\nminimum cycles between calibration commands. The suffix '_f0' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiCalvlCcF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TDFI_CALVL_CAPTURE_F0` reader - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI\n\nclocks), the minimum cycles between a calibration command and a\n\ndfi_calvl_capture pulse. The suffix '_f0' of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdfiCalvlCaptureF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CALVL_CAPTURE_F0` writer - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI\n\nclocks), the minimum cycles between a calibration command and a\n\ndfi_calvl_capture pulse. The suffix '_f0' of the parameter name is\n\nomitted when in non-DFS mode."]
pub type PiTdfiCalvlCaptureF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the\n\nminimum cycles between calibration commands. The suffix '_f0' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_cc_f0(&self) -> PiTdfiCalvlCcF0R {
        PiTdfiCalvlCcF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI\n\nclocks), the minimum cycles between a calibration command and a\n\ndfi_calvl_capture pulse. The suffix '_f0' of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_capture_f0(&self) -> PiTdfiCalvlCaptureF0R {
        PiTdfiCalvlCaptureF0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the\n\nminimum cycles between calibration commands. The suffix '_f0' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_cc_f0(&mut self) -> PiTdfiCalvlCcF0W<DdrPiReg95Spec> {
        PiTdfiCalvlCcF0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI\n\nclocks), the minimum cycles between a calibration command and a\n\ndfi_calvl_capture pulse. The suffix '_f0' of the parameter name is\n\nomitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_capture_f0(&mut self) -> PiTdfiCalvlCaptureF0W<DdrPiReg95Spec> {
        PiTdfiCalvlCaptureF0W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_95::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_95::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg95Spec;
impl crate::RegisterSpec for DdrPiReg95Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_95::R`](R) reader structure"]
impl crate::Readable for DdrPiReg95Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_95::W`](W) writer structure"]
impl crate::Writable for DdrPiReg95Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_95 to value 0"]
impl crate::Resettable for DdrPiReg95Spec {
    const RESET_VALUE: u32 = 0;
}

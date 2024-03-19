#[doc = "Register `DDR_DENALI_CTL_308` reader"]
pub type R = crate::R<DdrDenaliCtl308Spec>;
#[doc = "Register `DDR_DENALI_CTL_308` writer"]
pub type W = crate::W<DdrDenaliCtl308Spec>;
#[doc = "Field `TDFI_CALVL_CC_F0` reader - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
pub type TdfiCalvlCcF0R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CALVL_CC_F0` writer - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
pub type TdfiCalvlCcF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TDFI_CALVL_CAPTURE_F0` reader - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
pub type TdfiCalvlCaptureF0R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CALVL_CAPTURE_F0` writer - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
pub type TdfiCalvlCaptureF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
    #[inline(always)]
    pub fn tdfi_calvl_cc_f0(&self) -> TdfiCalvlCcF0R {
        TdfiCalvlCcF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
    #[inline(always)]
    pub fn tdfi_calvl_capture_f0(&self) -> TdfiCalvlCaptureF0R {
        TdfiCalvlCaptureF0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_calvl_cc_f0(&mut self) -> TdfiCalvlCcF0W<DdrDenaliCtl308Spec> {
        TdfiCalvlCcF0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_calvl_capture_f0(&mut self) -> TdfiCalvlCaptureF0W<DdrDenaliCtl308Spec> {
        TdfiCalvlCaptureF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_308::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_308::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl308Spec;
impl crate::RegisterSpec for DdrDenaliCtl308Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_308::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl308Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_308::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl308Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_308 to value 0"]
impl crate::Resettable for DdrDenaliCtl308Spec {
    const RESET_VALUE: u32 = 0;
}

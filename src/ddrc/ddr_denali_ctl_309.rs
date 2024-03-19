#[doc = "Register `DDR_DENALI_CTL_309` reader"]
pub type R = crate::R<DdrDenaliCtl309Spec>;
#[doc = "Register `DDR_DENALI_CTL_309` writer"]
pub type W = crate::W<DdrDenaliCtl309Spec>;
#[doc = "Field `TDFI_CALVL_CC_F1` reader - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
pub type TdfiCalvlCcF1R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CALVL_CC_F1` writer - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
pub type TdfiCalvlCcF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TDFI_CALVL_CAPTURE_F1` reader - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
pub type TdfiCalvlCaptureF1R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CALVL_CAPTURE_F1` writer - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
pub type TdfiCalvlCaptureF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
    #[inline(always)]
    pub fn tdfi_calvl_cc_f1(&self) -> TdfiCalvlCcF1R {
        TdfiCalvlCcF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
    #[inline(always)]
    pub fn tdfi_calvl_capture_f1(&self) -> TdfiCalvlCaptureF1R {
        TdfiCalvlCaptureF1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_calvl_cc_f1(&mut self) -> TdfiCalvlCcF1W<DdrDenaliCtl309Spec> {
        TdfiCalvlCcF1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_calvl_capture_f1(&mut self) -> TdfiCalvlCaptureF1W<DdrDenaliCtl309Spec> {
        TdfiCalvlCaptureF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_309::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_309::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl309Spec;
impl crate::RegisterSpec for DdrDenaliCtl309Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_309::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl309Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_309::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl309Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_309 to value 0"]
impl crate::Resettable for DdrDenaliCtl309Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DENALI_CTL_310` reader"]
pub type R = crate::R<DenaliCtl310Spec>;
#[doc = "Register `DENALI_CTL_310` writer"]
pub type W = crate::W<DenaliCtl310Spec>;
#[doc = "Field `TDFI_CALVL_CC_F2` reader - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
pub type TdfiCalvlCcF2R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CALVL_CC_F2` writer - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
pub type TdfiCalvlCcF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TDFI_CALVL_CAPTURE_F2` reader - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
pub type TdfiCalvlCaptureF2R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_CALVL_CAPTURE_F2` writer - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
pub type TdfiCalvlCaptureF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
    #[inline(always)]
    pub fn tdfi_calvl_cc_f2(&self) -> TdfiCalvlCcF2R {
        TdfiCalvlCcF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
    #[inline(always)]
    pub fn tdfi_calvl_capture_f2(&self) -> TdfiCalvlCaptureF2R {
        TdfiCalvlCaptureF2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Defines the DFI tCALVL_CC timing parameter (in DFI clocks), the minimum cycles between calibration commands."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_calvl_cc_f2(&mut self) -> TdfiCalvlCcF2W<DenaliCtl310Spec> {
        TdfiCalvlCcF2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Defines the DFI tCALVL_CAPTURE timing parameter (in DFI clocks), the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_calvl_capture_f2(&mut self) -> TdfiCalvlCaptureF2W<DenaliCtl310Spec> {
        TdfiCalvlCaptureF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_310::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_310::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl310Spec;
impl crate::RegisterSpec for DenaliCtl310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_310::R`](R) reader structure"]
impl crate::Readable for DenaliCtl310Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_310::W`](W) writer structure"]
impl crate::Writable for DenaliCtl310Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_310 to value 0"]
impl crate::Resettable for DenaliCtl310Spec {
    const RESET_VALUE: u32 = 0;
}

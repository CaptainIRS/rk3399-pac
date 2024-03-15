#[doc = "Register `DENALI_CTL_313` reader"]
pub type R = crate::R<DenaliCtl313Spec>;
#[doc = "Register `DENALI_CTL_313` writer"]
pub type W = crate::W<DenaliCtl313Spec>;
#[doc = "Field `CALVL_RESP_MASK` reader - Mask for the dfi_calvl_resp signal during CA training."]
pub type CalvlRespMaskR = crate::BitReader;
#[doc = "Field `CALVL_RESP_MASK` writer - Mask for the dfi_calvl_resp signal during CA training."]
pub type CalvlRespMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALVL_EN` reader - Enable the MC CA training module. Set to 1 to enable."]
pub type CalvlEnR = crate::BitReader;
#[doc = "Field `CALVL_EN` writer - Enable the MC CA training module. Set to 1 to enable."]
pub type CalvlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALVL_ERROR_STATUS` reader - Holds the error associated with the CA training error interrupt. Bit (0) set indicates a TDFI_CALVL_RESP parameter violation and bit (1) set indicates a TDFI_CALVL_MAX parameter violation. READ-ONLY"]
pub type CalvlErrorStatusR = crate::FieldReader;
#[doc = "Field `TDFI_PHY_WRDATA` reader - Defines the DFI tPHY_WRDATA timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type TdfiPhyWrdataR = crate::FieldReader;
#[doc = "Field `TDFI_PHY_WRDATA` writer - Defines the DFI tPHY_WRDATA timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
pub type TdfiPhyWrdataW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Mask for the dfi_calvl_resp signal during CA training."]
    #[inline(always)]
    pub fn calvl_resp_mask(&self) -> CalvlRespMaskR {
        CalvlRespMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable the MC CA training module. Set to 1 to enable."]
    #[inline(always)]
    pub fn calvl_en(&self) -> CalvlEnR {
        CalvlEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Holds the error associated with the CA training error interrupt. Bit (0) set indicates a TDFI_CALVL_RESP parameter violation and bit (1) set indicates a TDFI_CALVL_MAX parameter violation. READ-ONLY"]
    #[inline(always)]
    pub fn calvl_error_status(&self) -> CalvlErrorStatusR {
        CalvlErrorStatusR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Defines the DFI tPHY_WRDATA timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    pub fn tdfi_phy_wrdata(&self) -> TdfiPhyWrdataR {
        TdfiPhyWrdataR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for the dfi_calvl_resp signal during CA training."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_resp_mask(&mut self) -> CalvlRespMaskW<DenaliCtl313Spec> {
        CalvlRespMaskW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable the MC CA training module. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_en(&mut self) -> CalvlEnW<DenaliCtl313Spec> {
        CalvlEnW::new(self, 8)
    }
    #[doc = "Bits 24:26 - Defines the DFI tPHY_WRDATA timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_wrdata(&mut self) -> TdfiPhyWrdataW<DenaliCtl313Spec> {
        TdfiPhyWrdataW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_313::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_313::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl313Spec;
impl crate::RegisterSpec for DenaliCtl313Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_313::R`](R) reader structure"]
impl crate::Readable for DenaliCtl313Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_313::W`](W) writer structure"]
impl crate::Writable for DenaliCtl313Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_313 to value 0"]
impl crate::Resettable for DenaliCtl313Spec {
    const RESET_VALUE: u32 = 0;
}

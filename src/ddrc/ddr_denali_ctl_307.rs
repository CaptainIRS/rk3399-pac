#[doc = "Register `DDR_DENALI_CTL_307` reader"]
pub type R = crate::R<DdrDenaliCtl307Spec>;
#[doc = "Register `DDR_DENALI_CTL_307` writer"]
pub type W = crate::W<DdrDenaliCtl307Spec>;
#[doc = "Field `RDLVL_ERROR_STATUS` reader - Holds the error associated with the data eye training error or gate training error interrupt. Uppermost bit set indicates a TDFI_RDLVL_RESP parameter violation. Next uppermost bit set indicates a TDFI_RDLVL_MAX parameter violation. Lower bits are reserved."]
pub type RdlvlErrorStatusR = crate::FieldReader;
#[doc = "Field `RDLVL_GATE_ERROR_STATUS` reader - Holds the error associated with the read gate training error or gate training error interrupt. Uppermost bit set indicates a TDFI_RDLVL_RESP parameter violation. Next uppermost bit set indicates a TDFI_RDLVL_MAX parameter violation. Lower bits are reserved."]
pub type RdlvlGateErrorStatusR = crate::FieldReader;
#[doc = "Field `TDFI_CALVL_EN` reader - Defines the DFI tCALVL_EN timing parameter (in DFI clocks), the minimum cycles between a dfi_calvl_en assertion and a dfi_cke de-assertion."]
pub type TdfiCalvlEnR = crate::FieldReader;
#[doc = "Field `TDFI_CALVL_EN` writer - Defines the DFI tCALVL_EN timing parameter (in DFI clocks), the minimum cycles between a dfi_calvl_en assertion and a dfi_cke de-assertion."]
pub type TdfiCalvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Holds the error associated with the data eye training error or gate training error interrupt. Uppermost bit set indicates a TDFI_RDLVL_RESP parameter violation. Next uppermost bit set indicates a TDFI_RDLVL_MAX parameter violation. Lower bits are reserved."]
    #[inline(always)]
    pub fn rdlvl_error_status(&self) -> RdlvlErrorStatusR {
        RdlvlErrorStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Holds the error associated with the read gate training error or gate training error interrupt. Uppermost bit set indicates a TDFI_RDLVL_RESP parameter violation. Next uppermost bit set indicates a TDFI_RDLVL_MAX parameter violation. Lower bits are reserved."]
    #[inline(always)]
    pub fn rdlvl_gate_error_status(&self) -> RdlvlGateErrorStatusR {
        RdlvlGateErrorStatusR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Defines the DFI tCALVL_EN timing parameter (in DFI clocks), the minimum cycles between a dfi_calvl_en assertion and a dfi_cke de-assertion."]
    #[inline(always)]
    pub fn tdfi_calvl_en(&self) -> TdfiCalvlEnR {
        TdfiCalvlEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Defines the DFI tCALVL_EN timing parameter (in DFI clocks), the minimum cycles between a dfi_calvl_en assertion and a dfi_cke de-assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_calvl_en(&mut self) -> TdfiCalvlEnW<DdrDenaliCtl307Spec> {
        TdfiCalvlEnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_307::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_307::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl307Spec;
impl crate::RegisterSpec for DdrDenaliCtl307Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_307::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl307Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_307::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl307Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_307 to value 0"]
impl crate::Resettable for DdrDenaliCtl307Spec {
    const RESET_VALUE: u32 = 0;
}

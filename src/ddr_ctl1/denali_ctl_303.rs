#[doc = "Register `DENALI_CTL_303` reader"]
pub type R = crate::R<DenaliCtl303Spec>;
#[doc = "Register `DENALI_CTL_303` writer"]
pub type W = crate::W<DenaliCtl303Spec>;
#[doc = "Field `TDFI_RDLVL_EN` reader - Defines the DFI tRDLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR."]
pub type TdfiRdlvlEnR = crate::FieldReader;
#[doc = "Field `TDFI_RDLVL_EN` writer - Defines the DFI tRDLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR."]
pub type TdfiRdlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_RDLVL_RR` reader - Defines the DFI tRDLVL_RR timing parameter (in DFI clocks), the minimum cycles between read commands."]
pub type TdfiRdlvlRrR = crate::FieldReader<u16>;
#[doc = "Field `TDFI_RDLVL_RR` writer - Defines the DFI tRDLVL_RR timing parameter (in DFI clocks), the minimum cycles between read commands."]
pub type TdfiRdlvlRrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - Defines the DFI tRDLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR."]
    #[inline(always)]
    pub fn tdfi_rdlvl_en(&self) -> TdfiRdlvlEnR {
        TdfiRdlvlEnR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - Defines the DFI tRDLVL_RR timing parameter (in DFI clocks), the minimum cycles between read commands."]
    #[inline(always)]
    pub fn tdfi_rdlvl_rr(&self) -> TdfiRdlvlRrR {
        TdfiRdlvlRrR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the DFI tRDLVL_EN timing parameter (in DFI clocks), the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdlvl_en(&mut self) -> TdfiRdlvlEnW<DenaliCtl303Spec> {
        TdfiRdlvlEnW::new(self, 0)
    }
    #[doc = "Bits 8:17 - Defines the DFI tRDLVL_RR timing parameter (in DFI clocks), the minimum cycles between read commands."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdlvl_rr(&mut self) -> TdfiRdlvlRrW<DenaliCtl303Spec> {
        TdfiRdlvlRrW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_303::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_303::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl303Spec;
impl crate::RegisterSpec for DenaliCtl303Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_303::R`](R) reader structure"]
impl crate::Readable for DenaliCtl303Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_303::W`](W) writer structure"]
impl crate::Writable for DenaliCtl303Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_303 to value 0"]
impl crate::Resettable for DenaliCtl303Spec {
    const RESET_VALUE: u32 = 0;
}

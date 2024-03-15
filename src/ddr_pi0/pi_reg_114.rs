#[doc = "Register `PI_REG_114` reader"]
pub type R = crate::R<PiReg114Spec>;
#[doc = "Register `PI_REG_114` writer"]
pub type W = crate::W<PiReg114Spec>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_F1` reader - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks), the maximum cycles between a dfi_init_start de-assertion and a dfi_init_complete assertion from the PHY. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode"]
pub type PiTdfiInitCompleteF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_F1` writer - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks), the maximum cycles between a dfi_init_start de-assertion and a dfi_init_complete assertion from the PHY. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode"]
pub type PiTdfiInitCompleteF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_TDFI_INIT_START_F2` reader - Defines the DFI tINIT_START timing parameter (in DFI clocks), the maximum number or cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiInitStartF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_INIT_START_F2` writer - Defines the DFI tINIT_START timing parameter (in DFI clocks), the maximum number or cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTdfiInitStartF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks), the maximum cycles between a dfi_init_start de-assertion and a dfi_init_complete assertion from the PHY. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode"]
    #[inline(always)]
    pub fn pi_tdfi_init_complete_f1(&self) -> PiTdfiInitCompleteF1R {
        PiTdfiInitCompleteF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Defines the DFI tINIT_START timing parameter (in DFI clocks), the maximum number or cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_init_start_f2(&self) -> PiTdfiInitStartF2R {
        PiTdfiInitStartF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks), the maximum cycles between a dfi_init_start de-assertion and a dfi_init_complete assertion from the PHY. The suffix \"_f1\" of the parameter name is omitted when in non-DFS mode"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_complete_f1(&mut self) -> PiTdfiInitCompleteF1W<PiReg114Spec> {
        PiTdfiInitCompleteF1W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Defines the DFI tINIT_START timing parameter (in DFI clocks), the maximum number or cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_start_f2(&mut self) -> PiTdfiInitStartF2W<PiReg114Spec> {
        PiTdfiInitStartF2W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 114\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_114::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_114::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg114Spec;
impl crate::RegisterSpec for PiReg114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_114::R`](R) reader structure"]
impl crate::Readable for PiReg114Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_114::W`](W) writer structure"]
impl crate::Writable for PiReg114Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_114 to value 0"]
impl crate::Resettable for PiReg114Spec {
    const RESET_VALUE: u32 = 0;
}

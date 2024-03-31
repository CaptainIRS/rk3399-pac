#[doc = "Register `DDR_PI_REG_113` reader"]
pub type R = crate::R<DdrPiReg113Spec>;
#[doc = "Register `DDR_PI_REG_113` writer"]
pub type W = crate::W<DdrPiReg113Spec>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_F0` reader - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_init_start de-assertion and a\n\ndfi_init_complete assertion from the PHY. The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode"]
pub type PiTdfiInitCompleteF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_F0` writer - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_init_start de-assertion and a\n\ndfi_init_complete assertion from the PHY. The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode"]
pub type PiTdfiInitCompleteF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_TDFI_INIT_START_F1` reader - Defines the DFI tINIT_START timing parameter (in DFI clocks), the\n\nmaximum number or cycles between a dfi_init_start assertion and\n\na dfi_init_complete de-assertion from the PHY. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiInitStartF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_INIT_START_F1` writer - Defines the DFI tINIT_START timing parameter (in DFI clocks), the\n\nmaximum number or cycles between a dfi_init_start assertion and\n\na dfi_init_complete de-assertion from the PHY. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
pub type PiTdfiInitStartF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_init_start de-assertion and a\n\ndfi_init_complete assertion from the PHY. The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode"]
    #[inline(always)]
    pub fn pi_tdfi_init_complete_f0(&self) -> PiTdfiInitCompleteF0R {
        PiTdfiInitCompleteF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Defines the DFI tINIT_START timing parameter (in DFI clocks), the\n\nmaximum number or cycles between a dfi_init_start assertion and\n\na dfi_init_complete de-assertion from the PHY. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tdfi_init_start_f1(&self) -> PiTdfiInitStartF1R {
        PiTdfiInitStartF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tINIT_COMPLETE timing parameter (in DFI clocks),\n\nthe maximum cycles between a dfi_init_start de-assertion and a\n\ndfi_init_complete assertion from the PHY. The suffix '_f0' of the\n\nparameter name is omitted when in non-DFS mode"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_complete_f0(&mut self) -> PiTdfiInitCompleteF0W<DdrPiReg113Spec> {
        PiTdfiInitCompleteF0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Defines the DFI tINIT_START timing parameter (in DFI clocks), the\n\nmaximum number or cycles between a dfi_init_start assertion and\n\na dfi_init_complete de-assertion from the PHY. The suffix '_f1' of\n\nthe parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_start_f1(&mut self) -> PiTdfiInitStartF1W<DdrPiReg113Spec> {
        PiTdfiInitStartF1W::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 113\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_113::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_113::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg113Spec;
impl crate::RegisterSpec for DdrPiReg113Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_113::R`](R) reader structure"]
impl crate::Readable for DdrPiReg113Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_113::W`](W) writer structure"]
impl crate::Writable for DdrPiReg113Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_113 to value 0"]
impl crate::Resettable for DdrPiReg113Spec {
    const RESET_VALUE: u32 = 0;
}

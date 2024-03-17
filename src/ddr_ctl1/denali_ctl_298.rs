#[doc = "Register `DENALI_CTL_298` reader"]
pub type R = crate::R<DenaliCtl298Spec>;
#[doc = "Register `DENALI_CTL_298` writer"]
pub type W = crate::W<DenaliCtl298Spec>;
#[doc = "Field `RDLAT_ADJ_F2` reader - Adjustment value for PHY read timing."]
pub type RdlatAdjF2R = crate::FieldReader;
#[doc = "Field `RDLAT_ADJ_F2` writer - Adjustment value for PHY read timing."]
pub type RdlatAdjF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRLAT_ADJ_F2` reader - Adjustment value for PHY write timing."]
pub type WrlatAdjF2R = crate::FieldReader;
#[doc = "Field `WRLAT_ADJ_F2` writer - Adjustment value for PHY write timing."]
pub type WrlatAdjF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_CTRL_DELAY_F0` reader - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
pub type TdfiCtrlDelayF0R = crate::FieldReader;
#[doc = "Field `TDFI_CTRL_DELAY_F0` writer - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
pub type TdfiCtrlDelayF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDFI_CTRL_DELAY_F1` reader - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
pub type TdfiCtrlDelayF1R = crate::FieldReader;
#[doc = "Field `TDFI_CTRL_DELAY_F1` writer - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
pub type TdfiCtrlDelayF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Adjustment value for PHY read timing."]
    #[inline(always)]
    pub fn rdlat_adj_f2(&self) -> RdlatAdjF2R {
        RdlatAdjF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Adjustment value for PHY write timing."]
    #[inline(always)]
    pub fn wrlat_adj_f2(&self) -> WrlatAdjF2R {
        WrlatAdjF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
    #[inline(always)]
    pub fn tdfi_ctrl_delay_f0(&self) -> TdfiCtrlDelayF0R {
        TdfiCtrlDelayF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
    #[inline(always)]
    pub fn tdfi_ctrl_delay_f1(&self) -> TdfiCtrlDelayF1R {
        TdfiCtrlDelayF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Adjustment value for PHY read timing."]
    #[inline(always)]
    #[must_use]
    pub fn rdlat_adj_f2(&mut self) -> RdlatAdjF2W<DenaliCtl298Spec> {
        RdlatAdjF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Adjustment value for PHY write timing."]
    #[inline(always)]
    #[must_use]
    pub fn wrlat_adj_f2(&mut self) -> WrlatAdjF2W<DenaliCtl298Spec> {
        WrlatAdjF2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrl_delay_f0(&mut self) -> TdfiCtrlDelayF0W<DenaliCtl298Spec> {
        TdfiCtrlDelayF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Defines the DFI tCTRL_DELAY timing parameter (in DFI clocks), the delay between a DFI command change and a memory command."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrl_delay_f1(&mut self) -> TdfiCtrlDelayF1W<DenaliCtl298Spec> {
        TdfiCtrlDelayF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_298::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_298::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl298Spec;
impl crate::RegisterSpec for DenaliCtl298Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_298::R`](R) reader structure"]
impl crate::Readable for DenaliCtl298Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_298::W`](W) writer structure"]
impl crate::Writable for DenaliCtl298Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_298 to value 0x0202_0000"]
impl crate::Resettable for DenaliCtl298Spec {
    const RESET_VALUE: u32 = 0x0202_0000;
}

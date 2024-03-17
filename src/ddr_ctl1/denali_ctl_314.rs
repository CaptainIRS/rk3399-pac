#[doc = "Register `DENALI_CTL_314` reader"]
pub type R = crate::R<DenaliCtl314Spec>;
#[doc = "Register `DENALI_CTL_314` writer"]
pub type W = crate::W<DenaliCtl314Spec>;
#[doc = "Field `TDFI_RDCSLAT_F0` reader - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
pub type TdfiRdcslatF0R = crate::FieldReader;
#[doc = "Field `TDFI_RDCSLAT_F0` writer - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
pub type TdfiRdcslatF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_WRCSLAT_F0` reader - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
pub type TdfiWrcslatF0R = crate::FieldReader;
#[doc = "Field `TDFI_WRCSLAT_F0` writer - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
pub type TdfiWrcslatF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_RDCSLAT_F1` reader - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
pub type TdfiRdcslatF1R = crate::FieldReader;
#[doc = "Field `TDFI_RDCSLAT_F1` writer - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
pub type TdfiRdcslatF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_WRCSLAT_F1` reader - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
pub type TdfiWrcslatF1R = crate::FieldReader;
#[doc = "Field `TDFI_WRCSLAT_F1` writer - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
pub type TdfiWrcslatF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
    #[inline(always)]
    pub fn tdfi_rdcslat_f0(&self) -> TdfiRdcslatF0R {
        TdfiRdcslatF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
    #[inline(always)]
    pub fn tdfi_wrcslat_f0(&self) -> TdfiWrcslatF0R {
        TdfiWrcslatF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
    #[inline(always)]
    pub fn tdfi_rdcslat_f1(&self) -> TdfiRdcslatF1R {
        TdfiRdcslatF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
    #[inline(always)]
    pub fn tdfi_wrcslat_f1(&self) -> TdfiWrcslatF1R {
        TdfiWrcslatF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdcslat_f0(&mut self) -> TdfiRdcslatF0W<DenaliCtl314Spec> {
        TdfiRdcslatF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrcslat_f0(&mut self) -> TdfiWrcslatF0W<DenaliCtl314Spec> {
        TdfiWrcslatF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdcslat_f1(&mut self) -> TdfiRdcslatF1W<DenaliCtl314Spec> {
        TdfiRdcslatF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrcslat_f1(&mut self) -> TdfiWrcslatF1W<DenaliCtl314Spec> {
        TdfiWrcslatF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_314::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_314::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl314Spec;
impl crate::RegisterSpec for DenaliCtl314Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_314::R`](R) reader structure"]
impl crate::Readable for DenaliCtl314Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_314::W`](W) writer structure"]
impl crate::Writable for DenaliCtl314Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_314 to value 0"]
impl crate::Resettable for DenaliCtl314Spec {
    const RESET_VALUE: u32 = 0;
}

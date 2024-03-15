#[doc = "Register `DENALI_CTL_17` reader"]
pub type R = crate::R<DenaliCtl17Spec>;
#[doc = "Register `DENALI_CTL_17` writer"]
pub type W = crate::W<DenaliCtl17Spec>;
#[doc = "Field `MRR_ERROR_STATUS` reader - Identifies the source of any MRR errors. Value of 1 indicates a violation. READ-ONLY"]
pub type MrrErrorStatusR = crate::FieldReader;
#[doc = "Field `DFI_INV_DATA_CS` reader - Forces the inversion of the dfi_rddata_cs_n_X and dfi_wrdata_cs_n_X signals. Set to 1 to force inversion."]
pub type DfiInvDataCsR = crate::BitReader;
#[doc = "Field `DFI_INV_DATA_CS` writer - Forces the inversion of the dfi_rddata_cs_n_X and dfi_wrdata_cs_n_X signals. Set to 1 to force inversion."]
pub type DfiInvDataCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_MRW_BT_INIT` reader - Disable MRW commands before training during initialization. Set to 1 to disable."]
pub type NoMrwBtInitR = crate::BitReader;
#[doc = "Field `NO_MRW_BT_INIT` writer - Disable MRW commands before training during initialization. Set to 1 to disable."]
pub type NoMrwBtInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_MRW_INIT` reader - Disable MRW commands after training during initialization. Set to 1 to disable."]
pub type NoMrwInitR = crate::BitReader;
#[doc = "Field `NO_MRW_INIT` writer - Disable MRW commands after training during initialization. Set to 1 to disable."]
pub type NoMrwInitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Identifies the source of any MRR errors. Value of 1 indicates a violation. READ-ONLY"]
    #[inline(always)]
    pub fn mrr_error_status(&self) -> MrrErrorStatusR {
        MrrErrorStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Forces the inversion of the dfi_rddata_cs_n_X and dfi_wrdata_cs_n_X signals. Set to 1 to force inversion."]
    #[inline(always)]
    pub fn dfi_inv_data_cs(&self) -> DfiInvDataCsR {
        DfiInvDataCsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable MRW commands before training during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn no_mrw_bt_init(&self) -> NoMrwBtInitR {
        NoMrwBtInitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable MRW commands after training during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn no_mrw_init(&self) -> NoMrwInitR {
        NoMrwInitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Forces the inversion of the dfi_rddata_cs_n_X and dfi_wrdata_cs_n_X signals. Set to 1 to force inversion."]
    #[inline(always)]
    #[must_use]
    pub fn dfi_inv_data_cs(&mut self) -> DfiInvDataCsW<DenaliCtl17Spec> {
        DfiInvDataCsW::new(self, 8)
    }
    #[doc = "Bit 16 - Disable MRW commands before training during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn no_mrw_bt_init(&mut self) -> NoMrwBtInitW<DenaliCtl17Spec> {
        NoMrwBtInitW::new(self, 16)
    }
    #[doc = "Bit 24 - Disable MRW commands after training during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn no_mrw_init(&mut self) -> NoMrwInitW<DenaliCtl17Spec> {
        NoMrwInitW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl17Spec;
impl crate::RegisterSpec for DenaliCtl17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_17::R`](R) reader structure"]
impl crate::Readable for DenaliCtl17Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_17::W`](W) writer structure"]
impl crate::Writable for DenaliCtl17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_17 to value 0"]
impl crate::Resettable for DenaliCtl17Spec {
    const RESET_VALUE: u32 = 0;
}

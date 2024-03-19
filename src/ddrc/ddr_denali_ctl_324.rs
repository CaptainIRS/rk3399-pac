#[doc = "Register `DDR_DENALI_CTL_324` reader"]
pub type R = crate::R<DdrDenaliCtl324Spec>;
#[doc = "Register `DDR_DENALI_CTL_324` writer"]
pub type W = crate::W<DdrDenaliCtl324Spec>;
#[doc = "Field `EN_1T_TIMING` reader - Enable 1T timing in a system supporting both 1T and 2T timing. Set to 1 to enable."]
pub type En1tTimingR = crate::BitReader;
#[doc = "Field `EN_1T_TIMING` writer - Enable 1T timing in a system supporting both 1T and 2T timing. Set to 1 to enable."]
pub type En1tTimingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_MEMORY_MASKED_WRITE` reader - Restricts the controller from masked write commands. Set to 1 to not issue these commands."]
pub type DisableMemoryMaskedWriteR = crate::BitReader;
#[doc = "Field `DISABLE_MEMORY_MASKED_WRITE` writer - Restricts the controller from masked write commands. Set to 1 to not issue these commands."]
pub type DisableMemoryMaskedWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL_ON_FLY_ENABLE` reader - Enables the burst length on the fly feature. Set to 1 to enable."]
pub type BlOnFlyEnableR = crate::BitReader;
#[doc = "Field `BL_ON_FLY_ENABLE` writer - Enables the burst length on the fly feature. Set to 1 to enable."]
pub type BlOnFlyEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTI_CHANNEL_ZQ_CAL_MASTER` reader - In a two controller scenario, defines if this controller will issue ZQ calibration start commands when neither controller is in a low power mode. Set to 1 to define this controller as the master that issues ZQ calibration start commands or clear to 0 to define this controller as the slave. This parameter should only be programmed once during initialization."]
pub type MultiChannelZqCalMasterR = crate::BitReader;
#[doc = "Field `MULTI_CHANNEL_ZQ_CAL_MASTER` writer - In a two controller scenario, defines if this controller will issue ZQ calibration start commands when neither controller is in a low power mode. Set to 1 to define this controller as the master that issues ZQ calibration start commands or clear to 0 to define this controller as the slave. This parameter should only be programmed once during initialization."]
pub type MultiChannelZqCalMasterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable 1T timing in a system supporting both 1T and 2T timing. Set to 1 to enable."]
    #[inline(always)]
    pub fn en_1t_timing(&self) -> En1tTimingR {
        En1tTimingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Restricts the controller from masked write commands. Set to 1 to not issue these commands."]
    #[inline(always)]
    pub fn disable_memory_masked_write(&self) -> DisableMemoryMaskedWriteR {
        DisableMemoryMaskedWriteR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the burst length on the fly feature. Set to 1 to enable."]
    #[inline(always)]
    pub fn bl_on_fly_enable(&self) -> BlOnFlyEnableR {
        BlOnFlyEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - In a two controller scenario, defines if this controller will issue ZQ calibration start commands when neither controller is in a low power mode. Set to 1 to define this controller as the master that issues ZQ calibration start commands or clear to 0 to define this controller as the slave. This parameter should only be programmed once during initialization."]
    #[inline(always)]
    pub fn multi_channel_zq_cal_master(&self) -> MultiChannelZqCalMasterR {
        MultiChannelZqCalMasterR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable 1T timing in a system supporting both 1T and 2T timing. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn en_1t_timing(&mut self) -> En1tTimingW<DdrDenaliCtl324Spec> {
        En1tTimingW::new(self, 0)
    }
    #[doc = "Bit 8 - Restricts the controller from masked write commands. Set to 1 to not issue these commands."]
    #[inline(always)]
    #[must_use]
    pub fn disable_memory_masked_write(
        &mut self,
    ) -> DisableMemoryMaskedWriteW<DdrDenaliCtl324Spec> {
        DisableMemoryMaskedWriteW::new(self, 8)
    }
    #[doc = "Bit 16 - Enables the burst length on the fly feature. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn bl_on_fly_enable(&mut self) -> BlOnFlyEnableW<DdrDenaliCtl324Spec> {
        BlOnFlyEnableW::new(self, 16)
    }
    #[doc = "Bit 24 - In a two controller scenario, defines if this controller will issue ZQ calibration start commands when neither controller is in a low power mode. Set to 1 to define this controller as the master that issues ZQ calibration start commands or clear to 0 to define this controller as the slave. This parameter should only be programmed once during initialization."]
    #[inline(always)]
    #[must_use]
    pub fn multi_channel_zq_cal_master(&mut self) -> MultiChannelZqCalMasterW<DdrDenaliCtl324Spec> {
        MultiChannelZqCalMasterW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_324::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_324::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl324Spec;
impl crate::RegisterSpec for DdrDenaliCtl324Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_324::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl324Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_324::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl324Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_324 to value 0"]
impl crate::Resettable for DdrDenaliCtl324Spec {
    const RESET_VALUE: u32 = 0;
}

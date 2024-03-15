#[doc = "Register `COMMON_INT_STA_4` reader"]
pub type R = crate::R<CommonIntSta4Spec>;
#[doc = "Register `COMMON_INT_STA_4` writer"]
pub type W = crate::W<CommonIntSta4Spec>;
#[doc = "Hot plug detect signal lost time is larger than 2ms before cable plugged, it means cable is plugged in:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plug {
    #[doc = "1: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B1 = 1,
    #[doc = "0: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B0 = 0,
}
impl From<Plug> for bool {
    #[inline(always)]
    fn from(variant: Plug) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLUG` reader - Hot plug detect signal lost time is larger than 2ms before cable plugged, it means cable is plugged in:"]
pub type PlugR = crate::BitReader<Plug>;
impl PlugR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plug {
        match self.bits {
            true => Plug::B1,
            false => Plug::B0,
        }
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Plug::B1
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Plug::B0
    }
}
#[doc = "Field `PLUG` writer - Hot plug detect signal lost time is larger than 2ms before cable plugged, it means cable is plugged in:"]
pub type PlugW<'a, REG> = crate::BitWriter<'a, REG, Plug>;
impl<'a, REG> PlugW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Plug::B1)
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Plug::B0)
    }
}
#[doc = "Hot plug detect signal lost timer larger than 2ms, that means cable is plugged out:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HpdLost {
    #[doc = "1: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B1 = 1,
    #[doc = "0: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B0 = 0,
}
impl From<HpdLost> for bool {
    #[inline(always)]
    fn from(variant: HpdLost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPD_LOST` reader - Hot plug detect signal lost timer larger than 2ms, that means cable is plugged out:"]
pub type HpdLostR = crate::BitReader<HpdLost>;
impl HpdLostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HpdLost {
        match self.bits {
            true => HpdLost::B1,
            false => HpdLost::B0,
        }
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HpdLost::B1
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HpdLost::B0
    }
}
#[doc = "Field `HPD_LOST` writer - Hot plug detect signal lost timer larger than 2ms, that means cable is plugged out:"]
pub type HpdLostW<'a, REG> = crate::BitWriter<'a, REG, HpdLost>;
impl<'a, REG> HpdLostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HpdLost::B1)
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HpdLost::B0)
    }
}
#[doc = "Field `HOTPLUG_CHG` reader - 1: Hot plug change detected. Write 1 to clear. HOTPLUG_CHG happens whenever the pin I_DP_HDP changes and the change remains for at least hot plug deglitch time. And the hot plug deglitch time is defined in HPD_DEGLITCH_L and HPD_DEGLITCH_H. When HOTPLUG_CHG is high, software shall check the status of HPD signal on register HPD_STATUS."]
pub type HotplugChgR = crate::BitReader;
#[doc = "Field `HOTPLUG_CHG` writer - 1: Hot plug change detected. Write 1 to clear. HOTPLUG_CHG happens whenever the pin I_DP_HDP changes and the change remains for at least hot plug deglitch time. And the hot plug deglitch time is defined in HPD_DEGLITCH_L and HPD_DEGLITCH_H. When HOTPLUG_CHG is high, software shall check the status of HPD signal on register HPD_STATUS."]
pub type HotplugChgW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hot plug detect signal lost time is larger than 2ms before cable plugged, it means cable is plugged in:"]
    #[inline(always)]
    pub fn plug(&self) -> PlugR {
        PlugR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hot plug detect signal lost timer larger than 2ms, that means cable is plugged out:"]
    #[inline(always)]
    pub fn hpd_lost(&self) -> HpdLostR {
        HpdLostR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Hot plug change detected. Write 1 to clear. HOTPLUG_CHG happens whenever the pin I_DP_HDP changes and the change remains for at least hot plug deglitch time. And the hot plug deglitch time is defined in HPD_DEGLITCH_L and HPD_DEGLITCH_H. When HOTPLUG_CHG is high, software shall check the status of HPD signal on register HPD_STATUS."]
    #[inline(always)]
    pub fn hotplug_chg(&self) -> HotplugChgR {
        HotplugChgR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hot plug detect signal lost time is larger than 2ms before cable plugged, it means cable is plugged in:"]
    #[inline(always)]
    #[must_use]
    pub fn plug(&mut self) -> PlugW<CommonIntSta4Spec> {
        PlugW::new(self, 0)
    }
    #[doc = "Bit 1 - Hot plug detect signal lost timer larger than 2ms, that means cable is plugged out:"]
    #[inline(always)]
    #[must_use]
    pub fn hpd_lost(&mut self) -> HpdLostW<CommonIntSta4Spec> {
        HpdLostW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Hot plug change detected. Write 1 to clear. HOTPLUG_CHG happens whenever the pin I_DP_HDP changes and the change remains for at least hot plug deglitch time. And the hot plug deglitch time is defined in HPD_DEGLITCH_L and HPD_DEGLITCH_H. When HOTPLUG_CHG is high, software shall check the status of HPD signal on register HPD_STATUS."]
    #[inline(always)]
    #[must_use]
    pub fn hotplug_chg(&mut self) -> HotplugChgW<CommonIntSta4Spec> {
        HotplugChgW::new(self, 2)
    }
}
#[doc = "Common Interrupt Status Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_sta_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_sta_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommonIntSta4Spec;
impl crate::RegisterSpec for CommonIntSta4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`common_int_sta_4::R`](R) reader structure"]
impl crate::Readable for CommonIntSta4Spec {}
#[doc = "`write(|w| ..)` method takes [`common_int_sta_4::W`](W) writer structure"]
impl crate::Writable for CommonIntSta4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x04;
}
#[doc = "`reset()` method sets COMMON_INT_STA_4 to value 0"]
impl crate::Resettable for CommonIntSta4Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `HDCP22REG_CTRL` reader"]
pub type R = crate::R<Hdcp22regCtrlSpec>;
#[doc = "Register `HDCP22REG_CTRL` writer"]
pub type W = crate::W<Hdcp22regCtrlSpec>;
#[doc = "HDCP 2.2 switch lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22SwitchLck {
    #[doc = "0: You can still write to hdcp22_ovr_en and hdcp22_ovr_val but has no effect over the HDCP 2.2 versus 1.4 switch, that keeps as it was configured by hdcp22_ovr_en and hdcp22_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    B0 = 0,
    #[doc = "1: You can still write to hdcp22_ovr_en and hdcp22_ovr_val but has no effect over the HDCP 2.2 versus 1.4 switch, that keeps as it was configured by hdcp22_ovr_en and hdcp22_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    B1 = 1,
}
impl From<Hdcp22SwitchLck> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22SwitchLck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_SWITCH_LCK` reader - HDCP 2.2 switch lock"]
pub type Hdcp22SwitchLckR = crate::BitReader<Hdcp22SwitchLck>;
impl Hdcp22SwitchLckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22SwitchLck {
        match self.bits {
            false => Hdcp22SwitchLck::B0,
            true => Hdcp22SwitchLck::B1,
        }
    }
    #[doc = "You can still write to hdcp22_ovr_en and hdcp22_ovr_val but has no effect over the HDCP 2.2 versus 1.4 switch, that keeps as it was configured by hdcp22_ovr_en and hdcp22_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22SwitchLck::B0
    }
    #[doc = "You can still write to hdcp22_ovr_en and hdcp22_ovr_val but has no effect over the HDCP 2.2 versus 1.4 switch, that keeps as it was configured by hdcp22_ovr_en and hdcp22_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22SwitchLck::B1
    }
}
#[doc = "Field `HDCP22_SWITCH_LCK` writer - HDCP 2.2 switch lock"]
pub type Hdcp22SwitchLckW<'a, REG> = crate::BitWriter<'a, REG, Hdcp22SwitchLck>;
impl<'a, REG> Hdcp22SwitchLckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "You can still write to hdcp22_ovr_en and hdcp22_ovr_val but has no effect over the HDCP 2.2 versus 1.4 switch, that keeps as it was configured by hdcp22_ovr_en and hdcp22_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwitchLck::B0)
    }
    #[doc = "You can still write to hdcp22_ovr_en and hdcp22_ovr_val but has no effect over the HDCP 2.2 versus 1.4 switch, that keeps as it was configured by hdcp22_ovr_en and hdcp22_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwitchLck::B1)
    }
}
#[doc = "HDCP 2.2 versus 1.4 switch override enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22OvrEn {
    #[doc = "0: The HDCP 2.2 ist_hdcp2_capable and ist_hdcp2_not_capable values are ignored, and the direction of the HDCP 2.2 versus 1.4 switch is directly controlled by the hdcp22_ovr_val."]
    B0 = 0,
    #[doc = "1: The HDCP 2.2 ist_hdcp2_capable and ist_hdcp2_not_capable values are ignored, and the direction of the HDCP 2.2 versus 1.4 switch is directly controlled by the hdcp22_ovr_val."]
    B1 = 1,
}
impl From<Hdcp22OvrEn> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22OvrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_OVR_EN` reader - HDCP 2.2 versus 1.4 switch override enable"]
pub type Hdcp22OvrEnR = crate::BitReader<Hdcp22OvrEn>;
impl Hdcp22OvrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22OvrEn {
        match self.bits {
            false => Hdcp22OvrEn::B0,
            true => Hdcp22OvrEn::B1,
        }
    }
    #[doc = "The HDCP 2.2 ist_hdcp2_capable and ist_hdcp2_not_capable values are ignored, and the direction of the HDCP 2.2 versus 1.4 switch is directly controlled by the hdcp22_ovr_val."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22OvrEn::B0
    }
    #[doc = "The HDCP 2.2 ist_hdcp2_capable and ist_hdcp2_not_capable values are ignored, and the direction of the HDCP 2.2 versus 1.4 switch is directly controlled by the hdcp22_ovr_val."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22OvrEn::B1
    }
}
#[doc = "Field `HDCP22_OVR_EN` writer - HDCP 2.2 versus 1.4 switch override enable"]
pub type Hdcp22OvrEnW<'a, REG> = crate::BitWriter<'a, REG, Hdcp22OvrEn>;
impl<'a, REG> Hdcp22OvrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The HDCP 2.2 ist_hdcp2_capable and ist_hdcp2_not_capable values are ignored, and the direction of the HDCP 2.2 versus 1.4 switch is directly controlled by the hdcp22_ovr_val."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22OvrEn::B0)
    }
    #[doc = "The HDCP 2.2 ist_hdcp2_capable and ist_hdcp2_not_capable values are ignored, and the direction of the HDCP 2.2 versus 1.4 switch is directly controlled by the hdcp22_ovr_val."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22OvrEn::B1)
    }
}
#[doc = "HDCP 2.2 versus 1.4 switch override value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22OvrVal {
    #[doc = "0: The switch is routed to HDCP 2.2 signals when hdcp22_ovr_en is 1'b1 and hdcp22_switch_lock is not set to 1'b1."]
    B0 = 0,
    #[doc = "1: The switch is routed to HDCP 2.2 signals when hdcp22_ovr_en is 1'b1 and hdcp22_switch_lock is not set to 1'b1."]
    B1 = 1,
}
impl From<Hdcp22OvrVal> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22OvrVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_OVR_VAL` reader - HDCP 2.2 versus 1.4 switch override value"]
pub type Hdcp22OvrValR = crate::BitReader<Hdcp22OvrVal>;
impl Hdcp22OvrValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22OvrVal {
        match self.bits {
            false => Hdcp22OvrVal::B0,
            true => Hdcp22OvrVal::B1,
        }
    }
    #[doc = "The switch is routed to HDCP 2.2 signals when hdcp22_ovr_en is 1'b1 and hdcp22_switch_lock is not set to 1'b1."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22OvrVal::B0
    }
    #[doc = "The switch is routed to HDCP 2.2 signals when hdcp22_ovr_en is 1'b1 and hdcp22_switch_lock is not set to 1'b1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22OvrVal::B1
    }
}
#[doc = "Field `HDCP22_OVR_VAL` writer - HDCP 2.2 versus 1.4 switch override value"]
pub type Hdcp22OvrValW<'a, REG> = crate::BitWriter<'a, REG, Hdcp22OvrVal>;
impl<'a, REG> Hdcp22OvrValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The switch is routed to HDCP 2.2 signals when hdcp22_ovr_en is 1'b1 and hdcp22_switch_lock is not set to 1'b1."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22OvrVal::B0)
    }
    #[doc = "The switch is routed to HDCP 2.2 signals when hdcp22_ovr_en is 1'b1 and hdcp22_switch_lock is not set to 1'b1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22OvrVal::B1)
    }
}
#[doc = "HPD Override enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HpdOvrEn {
    #[doc = "0: The HPD value to the HDCP 2.2 external interface comes from hpd_ovr_val bit field."]
    B0 = 0,
    #[doc = "1: The HPD value to the HDCP 2.2 external interface comes from hpd_ovr_val bit field."]
    B1 = 1,
}
impl From<HpdOvrEn> for bool {
    #[inline(always)]
    fn from(variant: HpdOvrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPD_OVR_EN` reader - HPD Override enable"]
pub type HpdOvrEnR = crate::BitReader<HpdOvrEn>;
impl HpdOvrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HpdOvrEn {
        match self.bits {
            false => HpdOvrEn::B0,
            true => HpdOvrEn::B1,
        }
    }
    #[doc = "The HPD value to the HDCP 2.2 external interface comes from hpd_ovr_val bit field."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HpdOvrEn::B0
    }
    #[doc = "The HPD value to the HDCP 2.2 external interface comes from hpd_ovr_val bit field."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HpdOvrEn::B1
    }
}
#[doc = "Field `HPD_OVR_EN` writer - HPD Override enable"]
pub type HpdOvrEnW<'a, REG> = crate::BitWriter<'a, REG, HpdOvrEn>;
impl<'a, REG> HpdOvrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The HPD value to the HDCP 2.2 external interface comes from hpd_ovr_val bit field."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HpdOvrEn::B0)
    }
    #[doc = "The HPD value to the HDCP 2.2 external interface comes from hpd_ovr_val bit field."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HpdOvrEn::B1)
    }
}
#[doc = "HPD Override Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HpdOvrVal {
    #[doc = "0: If hpd_ovr_en is 1'b1 the HPD value to the HDCP 2.2 external interface is set to 1'b1."]
    B0 = 0,
    #[doc = "1: If hpd_ovr_en is 1'b1 the HPD value to the HDCP 2.2 external interface is set to 1'b1."]
    B1 = 1,
}
impl From<HpdOvrVal> for bool {
    #[inline(always)]
    fn from(variant: HpdOvrVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPD_OVR_VAL` reader - HPD Override Value"]
pub type HpdOvrValR = crate::BitReader<HpdOvrVal>;
impl HpdOvrValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HpdOvrVal {
        match self.bits {
            false => HpdOvrVal::B0,
            true => HpdOvrVal::B1,
        }
    }
    #[doc = "If hpd_ovr_en is 1'b1 the HPD value to the HDCP 2.2 external interface is set to 1'b1."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HpdOvrVal::B0
    }
    #[doc = "If hpd_ovr_en is 1'b1 the HPD value to the HDCP 2.2 external interface is set to 1'b1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HpdOvrVal::B1
    }
}
#[doc = "Field `HPD_OVR_VAL` writer - HPD Override Value"]
pub type HpdOvrValW<'a, REG> = crate::BitWriter<'a, REG, HpdOvrVal>;
impl<'a, REG> HpdOvrValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If hpd_ovr_en is 1'b1 the HPD value to the HDCP 2.2 external interface is set to 1'b1."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HpdOvrVal::B0)
    }
    #[doc = "If hpd_ovr_en is 1'b1 the HPD value to the HDCP 2.2 external interface is set to 1'b1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HpdOvrVal::B1)
    }
}
impl R {
    #[doc = "Bit 0 - HDCP 2.2 switch lock"]
    #[inline(always)]
    pub fn hdcp22_switch_lck(&self) -> Hdcp22SwitchLckR {
        Hdcp22SwitchLckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HDCP 2.2 versus 1.4 switch override enable"]
    #[inline(always)]
    pub fn hdcp22_ovr_en(&self) -> Hdcp22OvrEnR {
        Hdcp22OvrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HDCP 2.2 versus 1.4 switch override value"]
    #[inline(always)]
    pub fn hdcp22_ovr_val(&self) -> Hdcp22OvrValR {
        Hdcp22OvrValR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - HPD Override enable"]
    #[inline(always)]
    pub fn hpd_ovr_en(&self) -> HpdOvrEnR {
        HpdOvrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HPD Override Value"]
    #[inline(always)]
    pub fn hpd_ovr_val(&self) -> HpdOvrValR {
        HpdOvrValR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HDCP 2.2 switch lock"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_switch_lck(&mut self) -> Hdcp22SwitchLckW<Hdcp22regCtrlSpec> {
        Hdcp22SwitchLckW::new(self, 0)
    }
    #[doc = "Bit 1 - HDCP 2.2 versus 1.4 switch override enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_ovr_en(&mut self) -> Hdcp22OvrEnW<Hdcp22regCtrlSpec> {
        Hdcp22OvrEnW::new(self, 1)
    }
    #[doc = "Bit 2 - HDCP 2.2 versus 1.4 switch override value"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_ovr_val(&mut self) -> Hdcp22OvrValW<Hdcp22regCtrlSpec> {
        Hdcp22OvrValW::new(self, 2)
    }
    #[doc = "Bit 4 - HPD Override enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpd_ovr_en(&mut self) -> HpdOvrEnW<Hdcp22regCtrlSpec> {
        HpdOvrEnW::new(self, 4)
    }
    #[doc = "Bit 5 - HPD Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn hpd_ovr_val(&mut self) -> HpdOvrValW<Hdcp22regCtrlSpec> {
        HpdOvrValW::new(self, 5)
    }
}
#[doc = "HDCP 2.2 switch lock 1'b0: Enables you to change the direction of the HDCP 2.2 versus 1.4 switch by using the hdcp22_ovr_en and hdcp22_ovr_val. 1'b1: You can still write to hdcp22_ovr_en and hdcp22_ovr_val but has no effect over the HDCP 2.2 versus 1.4 switch, that keeps as it was configured by hdcp22_ovr_en and hdcp22_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22reg_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hdcp22regCtrlSpec;
impl crate::RegisterSpec for Hdcp22regCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp22reg_ctrl::R`](R) reader structure"]
impl crate::Readable for Hdcp22regCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcp22reg_ctrl::W`](W) writer structure"]
impl crate::Writable for Hdcp22regCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP22REG_CTRL to value 0"]
impl crate::Resettable for Hdcp22regCtrlSpec {
    const RESET_VALUE: u8 = 0;
}

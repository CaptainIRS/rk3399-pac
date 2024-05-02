#[doc = "Register `TCPC_CONTROL` reader"]
pub type R = crate::R<TcpcControlSpec>;
#[doc = "Register `TCPC_CONTROL` writer"]
pub type W = crate::W<TcpcControlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlugOrientation {
    #[doc = "0: When Vconn is enabled, apply it to the CC2 pin. Monitor the CC1 pin for BMC communications if PD messaging is enabled."]
    B0 = 0,
    #[doc = "1: When Vconn is enabled, apply it to the CC1pin. Monitor the CC2 pin for BMC communications if PD messaging is enabled. Required"]
    B1 = 1,
}
impl From<PlugOrientation> for bool {
    #[inline(always)]
    fn from(variant: PlugOrientation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Plug_Orientation` reader - "]
pub type PlugOrientationR = crate::BitReader<PlugOrientation>;
impl PlugOrientationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PlugOrientation {
        match self.bits {
            false => PlugOrientation::B0,
            true => PlugOrientation::B1,
        }
    }
    #[doc = "When Vconn is enabled, apply it to the CC2 pin. Monitor the CC1 pin for BMC communications if PD messaging is enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PlugOrientation::B0
    }
    #[doc = "When Vconn is enabled, apply it to the CC1pin. Monitor the CC2 pin for BMC communications if PD messaging is enabled. Required"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PlugOrientation::B1
    }
}
#[doc = "Field `Plug_Orientation` writer - "]
pub type PlugOrientationW<'a, REG> = crate::BitWriter<'a, REG, PlugOrientation>;
impl<'a, REG> PlugOrientationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When Vconn is enabled, apply it to the CC2 pin. Monitor the CC1 pin for BMC communications if PD messaging is enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PlugOrientation::B0)
    }
    #[doc = "When Vconn is enabled, apply it to the CC1pin. Monitor the CC2 pin for BMC communications if PD messaging is enabled. Required"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PlugOrientation::B1)
    }
}
#[doc = "Setting this bit to 1 is intended to be used \n\nonly when a USB compliance tester is using \n\nUSB BIST Test Data to test the PHY layer of \n\nthe TCPC. The TCPM should clear it when a \n\ndetach is detected.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BistTestMode {
    #[doc = "0: Normal Operation. Incoming messages enabled by RECEIVE_DETECT passed to TCPM via Alert."]
    B0 = 0,
    #[doc = "1: BIST Test Mode. Incoming messages enabled by RECEIVE_DETECT result in GoodCRC response but may not be passed to the TCPM via Alert. TCPC may temporarily store incoming messages in the Receive Message Buffer, but this may or may not result in a Receive SOP* Message Status or a Rx Buffer Overflow alert. The TCPM can mask or ignore received message alerts when this bit is set to 1 since the TCPC may or may not assert the alert. The TCPM may also treat received messages in this mode in the same way as received messages during normal operation."]
    B1 = 1,
}
impl From<BistTestMode> for bool {
    #[inline(always)]
    fn from(variant: BistTestMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIST_Test_Mode` reader - Setting this bit to 1 is intended to be used \n\nonly when a USB compliance tester is using \n\nUSB BIST Test Data to test the PHY layer of \n\nthe TCPC. The TCPM should clear it when a \n\ndetach is detected."]
pub type BistTestModeR = crate::BitReader<BistTestMode>;
impl BistTestModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BistTestMode {
        match self.bits {
            false => BistTestMode::B0,
            true => BistTestMode::B1,
        }
    }
    #[doc = "Normal Operation. Incoming messages enabled by RECEIVE_DETECT passed to TCPM via Alert."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BistTestMode::B0
    }
    #[doc = "BIST Test Mode. Incoming messages enabled by RECEIVE_DETECT result in GoodCRC response but may not be passed to the TCPM via Alert. TCPC may temporarily store incoming messages in the Receive Message Buffer, but this may or may not result in a Receive SOP* Message Status or a Rx Buffer Overflow alert. The TCPM can mask or ignore received message alerts when this bit is set to 1 since the TCPC may or may not assert the alert. The TCPM may also treat received messages in this mode in the same way as received messages during normal operation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BistTestMode::B1
    }
}
#[doc = "Field `BIST_Test_Mode` writer - Setting this bit to 1 is intended to be used \n\nonly when a USB compliance tester is using \n\nUSB BIST Test Data to test the PHY layer of \n\nthe TCPC. The TCPM should clear it when a \n\ndetach is detected."]
pub type BistTestModeW<'a, REG> = crate::BitWriter<'a, REG, BistTestMode>;
impl<'a, REG> BistTestModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Operation. Incoming messages enabled by RECEIVE_DETECT passed to TCPM via Alert."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BistTestMode::B0)
    }
    #[doc = "BIST Test Mode. Incoming messages enabled by RECEIVE_DETECT result in GoodCRC response but may not be passed to the TCPM via Alert. TCPC may temporarily store incoming messages in the Receive Message Buffer, but this may or may not result in a Receive SOP* Message Status or a Rx Buffer Overflow alert. The TCPM can mask or ignore received message alerts when this bit is set to 1 since the TCPC may or may not assert the alert. The TCPM may also treat received messages in this mode in the same way as received messages during normal operation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BistTestMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugAccessoryControl {
    #[doc = "0: Controlled by TCPC (power on default)"]
    B0 = 0,
    #[doc = "1: Controlled by TCPM. The TCPM writes 1b to this register to take over control of asserting the DebugAccessoryConnected#. Required (Register is required but output is not required)"]
    B1 = 1,
}
impl From<DebugAccessoryControl> for bool {
    #[inline(always)]
    fn from(variant: DebugAccessoryControl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Debug_Accessory_Control` reader - "]
pub type DebugAccessoryControlR = crate::BitReader<DebugAccessoryControl>;
impl DebugAccessoryControlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DebugAccessoryControl {
        match self.bits {
            false => DebugAccessoryControl::B0,
            true => DebugAccessoryControl::B1,
        }
    }
    #[doc = "Controlled by TCPC (power on default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DebugAccessoryControl::B0
    }
    #[doc = "Controlled by TCPM. The TCPM writes 1b to this register to take over control of asserting the DebugAccessoryConnected#. Required (Register is required but output is not required)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DebugAccessoryControl::B1
    }
}
#[doc = "Field `Debug_Accessory_Control` writer - "]
pub type DebugAccessoryControlW<'a, REG> = crate::BitWriter<'a, REG, DebugAccessoryControl>;
impl<'a, REG> DebugAccessoryControlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Controlled by TCPC (power on default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DebugAccessoryControl::B0)
    }
    #[doc = "Controlled by TCPM. The TCPM writes 1b to this register to take over control of asserting the DebugAccessoryConnected#. Required (Register is required but output is not required)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DebugAccessoryControl::B1)
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn plug_orientation(&self) -> PlugOrientationR {
        PlugOrientationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 is intended to be used \n\nonly when a USB compliance tester is using \n\nUSB BIST Test Data to test the PHY layer of \n\nthe TCPC. The TCPM should clear it when a \n\ndetach is detected."]
    #[inline(always)]
    pub fn bist_test_mode(&self) -> BistTestModeR {
        BistTestModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn debug_accessory_control(&self) -> DebugAccessoryControlR {
        DebugAccessoryControlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn plug_orientation(&mut self) -> PlugOrientationW<TcpcControlSpec> {
        PlugOrientationW::new(self, 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 is intended to be used \n\nonly when a USB compliance tester is using \n\nUSB BIST Test Data to test the PHY layer of \n\nthe TCPC. The TCPM should clear it when a \n\ndetach is detected."]
    #[inline(always)]
    #[must_use]
    pub fn bist_test_mode(&mut self) -> BistTestModeW<TcpcControlSpec> {
        BistTestModeW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn debug_accessory_control(&mut self) -> DebugAccessoryControlW<TcpcControlSpec> {
        DebugAccessoryControlW::new(self, 4)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<TcpcControlSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "TCPC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcpc_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcpc_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcpcControlSpec;
impl crate::RegisterSpec for TcpcControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcpc_control::R`](R) reader structure"]
impl crate::Readable for TcpcControlSpec {}
#[doc = "`write(|w| ..)` method takes [`tcpc_control::W`](W) writer structure"]
impl crate::Writable for TcpcControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCPC_CONTROL to value 0"]
impl crate::Resettable for TcpcControlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SECURE_CTRL` reader"]
pub type R = crate::R<SecureCtrlSpec>;
#[doc = "Register `SECURE_CTRL` writer"]
pub type W = crate::W<SecureCtrlSpec>;
#[doc = "Non-secure register access override\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NonSecureOverride {
    #[doc = "0: Disable Non-secure access to CCI-400 registers"]
    B0 = 0,
    #[doc = "1: Enable Non-secure access to CCI-400 registers"]
    B1 = 1,
}
impl From<NonSecureOverride> for bool {
    #[inline(always)]
    fn from(variant: NonSecureOverride) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NON_SECURE_OVERRIDE` reader - Non-secure register access override"]
pub type NonSecureOverrideR = crate::BitReader<NonSecureOverride>;
impl NonSecureOverrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NonSecureOverride {
        match self.bits {
            false => NonSecureOverride::B0,
            true => NonSecureOverride::B1,
        }
    }
    #[doc = "Disable Non-secure access to CCI-400 registers"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NonSecureOverride::B0
    }
    #[doc = "Enable Non-secure access to CCI-400 registers"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NonSecureOverride::B1
    }
}
#[doc = "Field `NON_SECURE_OVERRIDE` writer - Non-secure register access override"]
pub type NonSecureOverrideW<'a, REG> = crate::BitWriter<'a, REG, NonSecureOverride>;
impl<'a, REG> NonSecureOverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Non-secure access to CCI-400 registers"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(NonSecureOverride::B0)
    }
    #[doc = "Enable Non-secure access to CCI-400 registers"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(NonSecureOverride::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugMonitorSecurityOverride {
    #[doc = "0: Enable Non-secure access to the PMU and Interface Monitor Registers."]
    B0 = 0,
    #[doc = "1: Disable Non-secure access to the PMU and Interface Monitor Registers, unless overridden by bit\\[0\\]"]
    B1 = 1,
}
impl From<DebugMonitorSecurityOverride> for bool {
    #[inline(always)]
    fn from(variant: DebugMonitorSecurityOverride) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUG_MONITOR_SECURITY_OVERRIDE` reader - "]
pub type DebugMonitorSecurityOverrideR = crate::BitReader<DebugMonitorSecurityOverride>;
impl DebugMonitorSecurityOverrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DebugMonitorSecurityOverride {
        match self.bits {
            false => DebugMonitorSecurityOverride::B0,
            true => DebugMonitorSecurityOverride::B1,
        }
    }
    #[doc = "Enable Non-secure access to the PMU and Interface Monitor Registers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DebugMonitorSecurityOverride::B0
    }
    #[doc = "Disable Non-secure access to the PMU and Interface Monitor Registers, unless overridden by bit\\[0\\]"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DebugMonitorSecurityOverride::B1
    }
}
#[doc = "Field `DEBUG_MONITOR_SECURITY_OVERRIDE` writer - "]
pub type DebugMonitorSecurityOverrideW<'a, REG> =
    crate::BitWriter<'a, REG, DebugMonitorSecurityOverride>;
impl<'a, REG> DebugMonitorSecurityOverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Non-secure access to the PMU and Interface Monitor Registers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DebugMonitorSecurityOverride::B0)
    }
    #[doc = "Disable Non-secure access to the PMU and Interface Monitor Registers, unless overridden by bit\\[0\\]"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DebugMonitorSecurityOverride::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Non-secure register access override"]
    #[inline(always)]
    pub fn non_secure_override(&self) -> NonSecureOverrideR {
        NonSecureOverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn debug_monitor_security_override(&self) -> DebugMonitorSecurityOverrideR {
        DebugMonitorSecurityOverrideR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-secure register access override"]
    #[inline(always)]
    #[must_use]
    pub fn non_secure_override(&mut self) -> NonSecureOverrideW<SecureCtrlSpec> {
        NonSecureOverrideW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn debug_monitor_security_override(
        &mut self,
    ) -> DebugMonitorSecurityOverrideW<SecureCtrlSpec> {
        DebugMonitorSecurityOverrideW::new(self, 1)
    }
}
#[doc = "Secure Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secure_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secure_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureCtrlSpec;
impl crate::RegisterSpec for SecureCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_ctrl::R`](R) reader structure"]
impl crate::Readable for SecureCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`secure_ctrl::W`](W) writer structure"]
impl crate::Writable for SecureCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECURE_CTRL to value 0"]
impl crate::Resettable for SecureCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

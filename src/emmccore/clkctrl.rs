#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Internalclockenable {
    #[doc = "1: Oscillate"]
    B1 = 1,
    #[doc = "0: Stop This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
    B0 = 0,
}
impl From<Internalclockenable> for bool {
    #[inline(always)]
    fn from(variant: Internalclockenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTERNALCLOCKENABLE` reader - "]
pub type InternalclockenableR = crate::BitReader<Internalclockenable>;
impl InternalclockenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Internalclockenable {
        match self.bits {
            true => Internalclockenable::B1,
            false => Internalclockenable::B0,
        }
    }
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Internalclockenable::B1
    }
    #[doc = "Stop This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Internalclockenable::B0
    }
}
#[doc = "Field `INTERNALCLOCKENABLE` writer - "]
pub type InternalclockenableW<'a, REG> = crate::BitWriter<'a, REG, Internalclockenable>;
impl<'a, REG> InternalclockenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillate"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Internalclockenable::B1)
    }
    #[doc = "Stop This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Internalclockenable::B0)
    }
}
#[doc = "Internal Clock Stable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Internalclockstable {
    #[doc = "1: Ready"]
    B1 = 1,
    #[doc = "0: Not Ready This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
    B0 = 0,
}
impl From<Internalclockstable> for bool {
    #[inline(always)]
    fn from(variant: Internalclockstable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTERNALCLOCKSTABLE` reader - Internal Clock Stable."]
pub type InternalclockstableR = crate::BitReader<Internalclockstable>;
impl InternalclockstableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Internalclockstable {
        match self.bits {
            true => Internalclockstable::B1,
            false => Internalclockstable::B0,
        }
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Internalclockstable::B1
    }
    #[doc = "Not Ready This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Internalclockstable::B0
    }
}
#[doc = "SD Clock Enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdclkena {
    #[doc = "1: Enable"]
    B1 = 1,
    #[doc = "0: Disable The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
    B0 = 0,
}
impl From<Sdclkena> for bool {
    #[inline(always)]
    fn from(variant: Sdclkena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDCLKENA` reader - SD Clock Enable."]
pub type SdclkenaR = crate::BitReader<Sdclkena>;
impl SdclkenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdclkena {
        match self.bits {
            true => Sdclkena::B1,
            false => Sdclkena::B0,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sdclkena::B1
    }
    #[doc = "Disable The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sdclkena::B0
    }
}
#[doc = "Field `SDCLKENA` writer - SD Clock Enable."]
pub type SdclkenaW<'a, REG> = crate::BitWriter<'a, REG, Sdclkena>;
impl<'a, REG> SdclkenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkena::B1)
    }
    #[doc = "Disable The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdclkena::B0)
    }
}
#[doc = "Clock Generator Select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkgensel {
    #[doc = "1: Programmable Clock Mode"]
    B1 = 1,
    #[doc = "0: Divided Clock Mode This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable= 0, this bit is set by Host Driver. If the Preset Value Enable= 1, this bit is automatically set to a value specified in one of Preset Value registers."]
    B0 = 0,
}
impl From<Clkgensel> for bool {
    #[inline(always)]
    fn from(variant: Clkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKGENSEL` reader - Clock Generator Select."]
pub type ClkgenselR = crate::BitReader<Clkgensel>;
impl ClkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkgensel {
        match self.bits {
            true => Clkgensel::B1,
            false => Clkgensel::B0,
        }
    }
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Clkgensel::B1
    }
    #[doc = "Divided Clock Mode This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable= 0, this bit is set by Host Driver. If the Preset Value Enable= 1, this bit is automatically set to a value specified in one of Preset Value registers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Clkgensel::B0
    }
}
#[doc = "Field `CLKGENSEL` writer - Clock Generator Select."]
pub type ClkgenselW<'a, REG> = crate::BitWriter<'a, REG, Clkgensel>;
impl<'a, REG> ClkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgensel::B1)
    }
    #[doc = "Divided Clock Mode This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable= 0, this bit is set by Host Driver. If the Preset Value Enable= 1, this bit is automatically set to a value specified in one of Preset Value registers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Clkgensel::B0)
    }
}
#[doc = "Field `SDCLKFREQSELUPPER` reader - Field0001 Abstract\n\nUpper Bits of SDCLK Frequency Select.\n\nBit 07-06 is assigned to bit 09-08 of clock divider in SDCLK\n\nFrequency Select"]
pub type SdclkfreqselupperR = crate::FieldReader;
#[doc = "Field `SDCLKFREQSELUPPER` writer - Field0001 Abstract\n\nUpper Bits of SDCLK Frequency Select.\n\nBit 07-06 is assigned to bit 09-08 of clock divider in SDCLK\n\nFrequency Select"]
pub type SdclkfreqselupperW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDCLKFREQSEL` reader - SDCLK Frequency Select.\n\nThis register is used to select the frequency of the SDCLK pin.\n\nThe frequency is not programmed directly; rather this register\n\nholds the divisor of the Base Clock Frequency For SD clock in the\n\ncapabilities register. Only the following settings are allowed.\n\n(1) 8-bit Divided Clock Mode\n\n8'h80: base clock divided by 256\n\n8'h40: base clock divided by 128\n\n8'h20: base clock divided by 64\n\n8'h10: base clock divided by 32\n\n8'h08: base clock divided by 16\n\n8'h04: base clock divided by 8\n\n8'h02: base clock divided by 4\n\n8'h01: base clock divided by 2\n\n8'h00: base clock(10MHz-63MHz)\n\nSetting 8'h00 specifies the highest frequency of the SD Clock.\n\nWhen setting multiple bits, the most significant bit is used as the\n\ndivisor. But multiple bits should not be set. The two default\n\ndivider values can be calculated by the frequency that is defined\n\nby the Base Clock Frequency For SD Clock in the Capabilities\n\nregister.\n\na. 25 MHz divider value\n\nb. 400 KHz divider value\n\nThe frequency of the SDCLK is set by the following formula:\n\nClock Frequency = (Baseclock) / divisor.\n\nThus choose the smallest possible divisor which results in a clock\n\nfrequency that is less than or equal to the target frequency.\n\nMaximum Frequency for SD = 50Mhz (base clock)\n\nMaximum Frequency for eMMC = 52Mhz (base clock)\n\nMinimum Frequency = 195.3125Khz (50Mhz / 256), same\n\ncalculation for eMMC also.\n\n(2) 10-bit Divided Clock Mode\n\nHost Controller supports this mandatory mode instead of the 8-\n\nbit Divided Clock Mode. The length of divider is extended to10\n\nbits and all divider values shall be supported.\n\n10'h3FF:1/2046 Divided Clock\n\nN: 1/2N Divided Clock (Duty 50%)\n\n10'h002: 1/4 Divided Clock\n\n10'h001: 1/2 Divided Clock\n\n10'h000: Base Clock (10MHz-254MHz)"]
pub type SdclkfreqselR = crate::FieldReader;
#[doc = "Field `SDCLKFREQSEL` writer - SDCLK Frequency Select.\n\nThis register is used to select the frequency of the SDCLK pin.\n\nThe frequency is not programmed directly; rather this register\n\nholds the divisor of the Base Clock Frequency For SD clock in the\n\ncapabilities register. Only the following settings are allowed.\n\n(1) 8-bit Divided Clock Mode\n\n8'h80: base clock divided by 256\n\n8'h40: base clock divided by 128\n\n8'h20: base clock divided by 64\n\n8'h10: base clock divided by 32\n\n8'h08: base clock divided by 16\n\n8'h04: base clock divided by 8\n\n8'h02: base clock divided by 4\n\n8'h01: base clock divided by 2\n\n8'h00: base clock(10MHz-63MHz)\n\nSetting 8'h00 specifies the highest frequency of the SD Clock.\n\nWhen setting multiple bits, the most significant bit is used as the\n\ndivisor. But multiple bits should not be set. The two default\n\ndivider values can be calculated by the frequency that is defined\n\nby the Base Clock Frequency For SD Clock in the Capabilities\n\nregister.\n\na. 25 MHz divider value\n\nb. 400 KHz divider value\n\nThe frequency of the SDCLK is set by the following formula:\n\nClock Frequency = (Baseclock) / divisor.\n\nThus choose the smallest possible divisor which results in a clock\n\nfrequency that is less than or equal to the target frequency.\n\nMaximum Frequency for SD = 50Mhz (base clock)\n\nMaximum Frequency for eMMC = 52Mhz (base clock)\n\nMinimum Frequency = 195.3125Khz (50Mhz / 256), same\n\ncalculation for eMMC also.\n\n(2) 10-bit Divided Clock Mode\n\nHost Controller supports this mandatory mode instead of the 8-\n\nbit Divided Clock Mode. The length of divider is extended to10\n\nbits and all divider values shall be supported.\n\n10'h3FF:1/2046 Divided Clock\n\nN: 1/2N Divided Clock (Duty 50%)\n\n10'h002: 1/4 Divided Clock\n\n10'h001: 1/2 Divided Clock\n\n10'h000: Base Clock (10MHz-254MHz)"]
pub type SdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn internalclockenable(&self) -> InternalclockenableR {
        InternalclockenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable."]
    #[inline(always)]
    pub fn internalclockstable(&self) -> InternalclockstableR {
        InternalclockstableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable."]
    #[inline(always)]
    pub fn sdclkena(&self) -> SdclkenaR {
        SdclkenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select."]
    #[inline(always)]
    pub fn clkgensel(&self) -> ClkgenselR {
        ClkgenselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Field0001 Abstract\n\nUpper Bits of SDCLK Frequency Select.\n\nBit 07-06 is assigned to bit 09-08 of clock divider in SDCLK\n\nFrequency Select"]
    #[inline(always)]
    pub fn sdclkfreqselupper(&self) -> SdclkfreqselupperR {
        SdclkfreqselupperR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select.\n\nThis register is used to select the frequency of the SDCLK pin.\n\nThe frequency is not programmed directly; rather this register\n\nholds the divisor of the Base Clock Frequency For SD clock in the\n\ncapabilities register. Only the following settings are allowed.\n\n(1) 8-bit Divided Clock Mode\n\n8'h80: base clock divided by 256\n\n8'h40: base clock divided by 128\n\n8'h20: base clock divided by 64\n\n8'h10: base clock divided by 32\n\n8'h08: base clock divided by 16\n\n8'h04: base clock divided by 8\n\n8'h02: base clock divided by 4\n\n8'h01: base clock divided by 2\n\n8'h00: base clock(10MHz-63MHz)\n\nSetting 8'h00 specifies the highest frequency of the SD Clock.\n\nWhen setting multiple bits, the most significant bit is used as the\n\ndivisor. But multiple bits should not be set. The two default\n\ndivider values can be calculated by the frequency that is defined\n\nby the Base Clock Frequency For SD Clock in the Capabilities\n\nregister.\n\na. 25 MHz divider value\n\nb. 400 KHz divider value\n\nThe frequency of the SDCLK is set by the following formula:\n\nClock Frequency = (Baseclock) / divisor.\n\nThus choose the smallest possible divisor which results in a clock\n\nfrequency that is less than or equal to the target frequency.\n\nMaximum Frequency for SD = 50Mhz (base clock)\n\nMaximum Frequency for eMMC = 52Mhz (base clock)\n\nMinimum Frequency = 195.3125Khz (50Mhz / 256), same\n\ncalculation for eMMC also.\n\n(2) 10-bit Divided Clock Mode\n\nHost Controller supports this mandatory mode instead of the 8-\n\nbit Divided Clock Mode. The length of divider is extended to10\n\nbits and all divider values shall be supported.\n\n10'h3FF:1/2046 Divided Clock\n\nN: 1/2N Divided Clock (Duty 50%)\n\n10'h002: 1/4 Divided Clock\n\n10'h001: 1/2 Divided Clock\n\n10'h000: Base Clock (10MHz-254MHz)"]
    #[inline(always)]
    pub fn sdclkfreqsel(&self) -> SdclkfreqselR {
        SdclkfreqselR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn internalclockenable(&mut self) -> InternalclockenableW<ClkctrlSpec> {
        InternalclockenableW::new(self, 0)
    }
    #[doc = "Bit 2 - SD Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn sdclkena(&mut self) -> SdclkenaW<ClkctrlSpec> {
        SdclkenaW::new(self, 2)
    }
    #[doc = "Bit 5 - Clock Generator Select."]
    #[inline(always)]
    #[must_use]
    pub fn clkgensel(&mut self) -> ClkgenselW<ClkctrlSpec> {
        ClkgenselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Field0001 Abstract\n\nUpper Bits of SDCLK Frequency Select.\n\nBit 07-06 is assigned to bit 09-08 of clock divider in SDCLK\n\nFrequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfreqselupper(&mut self) -> SdclkfreqselupperW<ClkctrlSpec> {
        SdclkfreqselupperW::new(self, 6)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select.\n\nThis register is used to select the frequency of the SDCLK pin.\n\nThe frequency is not programmed directly; rather this register\n\nholds the divisor of the Base Clock Frequency For SD clock in the\n\ncapabilities register. Only the following settings are allowed.\n\n(1) 8-bit Divided Clock Mode\n\n8'h80: base clock divided by 256\n\n8'h40: base clock divided by 128\n\n8'h20: base clock divided by 64\n\n8'h10: base clock divided by 32\n\n8'h08: base clock divided by 16\n\n8'h04: base clock divided by 8\n\n8'h02: base clock divided by 4\n\n8'h01: base clock divided by 2\n\n8'h00: base clock(10MHz-63MHz)\n\nSetting 8'h00 specifies the highest frequency of the SD Clock.\n\nWhen setting multiple bits, the most significant bit is used as the\n\ndivisor. But multiple bits should not be set. The two default\n\ndivider values can be calculated by the frequency that is defined\n\nby the Base Clock Frequency For SD Clock in the Capabilities\n\nregister.\n\na. 25 MHz divider value\n\nb. 400 KHz divider value\n\nThe frequency of the SDCLK is set by the following formula:\n\nClock Frequency = (Baseclock) / divisor.\n\nThus choose the smallest possible divisor which results in a clock\n\nfrequency that is less than or equal to the target frequency.\n\nMaximum Frequency for SD = 50Mhz (base clock)\n\nMaximum Frequency for eMMC = 52Mhz (base clock)\n\nMinimum Frequency = 195.3125Khz (50Mhz / 256), same\n\ncalculation for eMMC also.\n\n(2) 10-bit Divided Clock Mode\n\nHost Controller supports this mandatory mode instead of the 8-\n\nbit Divided Clock Mode. The length of divider is extended to10\n\nbits and all divider values shall be supported.\n\n10'h3FF:1/2046 Divided Clock\n\nN: 1/2N Divided Clock (Duty 50%)\n\n10'h002: 1/4 Divided Clock\n\n10'h001: 1/2 Divided Clock\n\n10'h000: Base Clock (10MHz-254MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfreqsel(&mut self) -> SdclkfreqselW<ClkctrlSpec> {
        SdclkfreqselW::new(self, 8)
    }
}
#[doc = "Clock control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {
    const RESET_VALUE: u16 = 0;
}

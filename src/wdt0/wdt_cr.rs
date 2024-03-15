#[doc = "Register `WDT_CR` reader"]
pub type R = crate::R<WdtCrSpec>;
#[doc = "Register `WDT_CR` writer"]
pub type W = crate::W<WdtCrSpec>;
#[doc = "Writable when the configuration parameter WDT_ALWAYS_EN=0, otherwise, it is readable. This bit is used to enable and disable the watchdog. When disabled, the counter dose not decrement .Thus, no interrupt or system reset is generated. Once this bit has been enabled, it can be cleared only by a system reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtEn {
    #[doc = "0: WDT enabled."]
    B0 = 0,
    #[doc = "1: WDT enabled."]
    B1 = 1,
}
impl From<WdtEn> for bool {
    #[inline(always)]
    fn from(variant: WdtEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_EN` reader - Writable when the configuration parameter WDT_ALWAYS_EN=0, otherwise, it is readable. This bit is used to enable and disable the watchdog. When disabled, the counter dose not decrement .Thus, no interrupt or system reset is generated. Once this bit has been enabled, it can be cleared only by a system reset."]
pub type WdtEnR = crate::BitReader<WdtEn>;
impl WdtEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtEn {
        match self.bits {
            false => WdtEn::B0,
            true => WdtEn::B1,
        }
    }
    #[doc = "WDT enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WdtEn::B0
    }
    #[doc = "WDT enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WdtEn::B1
    }
}
#[doc = "Field `WDT_EN` writer - Writable when the configuration parameter WDT_ALWAYS_EN=0, otherwise, it is readable. This bit is used to enable and disable the watchdog. When disabled, the counter dose not decrement .Thus, no interrupt or system reset is generated. Once this bit has been enabled, it can be cleared only by a system reset."]
pub type WdtEnW<'a, REG> = crate::BitWriter<'a, REG, WdtEn>;
impl<'a, REG> WdtEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDT enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WdtEn::B0)
    }
    #[doc = "WDT enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WdtEn::B1)
    }
}
#[doc = "Response mode. Selects the output response generated to a timeout.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RespMode {
    #[doc = "0: First generate an interrupt and if it is not cleared by the time a second timeout occurs then generate a system reset."]
    B0 = 0,
    #[doc = "1: First generate an interrupt and if it is not cleared by the time a second timeout occurs then generate a system reset."]
    B1 = 1,
}
impl From<RespMode> for bool {
    #[inline(always)]
    fn from(variant: RespMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESP_MODE` reader - Response mode. Selects the output response generated to a timeout."]
pub type RespModeR = crate::BitReader<RespMode>;
impl RespModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RespMode {
        match self.bits {
            false => RespMode::B0,
            true => RespMode::B1,
        }
    }
    #[doc = "First generate an interrupt and if it is not cleared by the time a second timeout occurs then generate a system reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RespMode::B0
    }
    #[doc = "First generate an interrupt and if it is not cleared by the time a second timeout occurs then generate a system reset."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RespMode::B1
    }
}
#[doc = "Field `RESP_MODE` writer - Response mode. Selects the output response generated to a timeout."]
pub type RespModeW<'a, REG> = crate::BitWriter<'a, REG, RespMode>;
impl<'a, REG> RespModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "First generate an interrupt and if it is not cleared by the time a second timeout occurs then generate a system reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RespMode::B0)
    }
    #[doc = "First generate an interrupt and if it is not cleared by the time a second timeout occurs then generate a system reset."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RespMode::B1)
    }
}
#[doc = "Reset pulse length. This is used to select the number of pclk cycles for which the system reset stays asserted.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RstPluseLenth {
    #[doc = "0: 256 pclk cycles"]
    B000 = 0,
    #[doc = "1: 256 pclk cycles"]
    B001 = 1,
    #[doc = "2: 256 pclk cycles"]
    B010 = 2,
    #[doc = "3: 256 pclk cycles"]
    B011 = 3,
    #[doc = "4: 256 pclk cycles"]
    B100 = 4,
    #[doc = "5: 256 pclk cycles"]
    B101 = 5,
    #[doc = "6: 256 pclk cycles"]
    B110 = 6,
    #[doc = "7: 256 pclk cycles"]
    B111 = 7,
}
impl From<RstPluseLenth> for u8 {
    #[inline(always)]
    fn from(variant: RstPluseLenth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RstPluseLenth {
    type Ux = u8;
}
#[doc = "Field `RST_PLUSE_LENTH` reader - Reset pulse length. This is used to select the number of pclk cycles for which the system reset stays asserted."]
pub type RstPluseLenthR = crate::FieldReader<RstPluseLenth>;
impl RstPluseLenthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstPluseLenth {
        match self.bits {
            0 => RstPluseLenth::B000,
            1 => RstPluseLenth::B001,
            2 => RstPluseLenth::B010,
            3 => RstPluseLenth::B011,
            4 => RstPluseLenth::B100,
            5 => RstPluseLenth::B101,
            6 => RstPluseLenth::B110,
            7 => RstPluseLenth::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == RstPluseLenth::B000
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == RstPluseLenth::B001
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == RstPluseLenth::B010
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == RstPluseLenth::B011
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == RstPluseLenth::B100
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == RstPluseLenth::B101
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == RstPluseLenth::B110
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == RstPluseLenth::B111
    }
}
#[doc = "Field `RST_PLUSE_LENTH` writer - Reset pulse length. This is used to select the number of pclk cycles for which the system reset stays asserted."]
pub type RstPluseLenthW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, RstPluseLenth>;
impl<'a, REG> RstPluseLenthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(RstPluseLenth::B000)
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(RstPluseLenth::B001)
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(RstPluseLenth::B010)
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(RstPluseLenth::B011)
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(RstPluseLenth::B100)
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(RstPluseLenth::B101)
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(RstPluseLenth::B110)
    }
    #[doc = "256 pclk cycles"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(RstPluseLenth::B111)
    }
}
impl R {
    #[doc = "Bit 0 - Writable when the configuration parameter WDT_ALWAYS_EN=0, otherwise, it is readable. This bit is used to enable and disable the watchdog. When disabled, the counter dose not decrement .Thus, no interrupt or system reset is generated. Once this bit has been enabled, it can be cleared only by a system reset."]
    #[inline(always)]
    pub fn wdt_en(&self) -> WdtEnR {
        WdtEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Response mode. Selects the output response generated to a timeout."]
    #[inline(always)]
    pub fn resp_mode(&self) -> RespModeR {
        RespModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Reset pulse length. This is used to select the number of pclk cycles for which the system reset stays asserted."]
    #[inline(always)]
    pub fn rst_pluse_lenth(&self) -> RstPluseLenthR {
        RstPluseLenthR::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Writable when the configuration parameter WDT_ALWAYS_EN=0, otherwise, it is readable. This bit is used to enable and disable the watchdog. When disabled, the counter dose not decrement .Thus, no interrupt or system reset is generated. Once this bit has been enabled, it can be cleared only by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_en(&mut self) -> WdtEnW<WdtCrSpec> {
        WdtEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Response mode. Selects the output response generated to a timeout."]
    #[inline(always)]
    #[must_use]
    pub fn resp_mode(&mut self) -> RespModeW<WdtCrSpec> {
        RespModeW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Reset pulse length. This is used to select the number of pclk cycles for which the system reset stays asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rst_pluse_lenth(&mut self) -> RstPluseLenthW<WdtCrSpec> {
        RstPluseLenthW::new(self, 2)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtCrSpec;
impl crate::RegisterSpec for WdtCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_cr::R`](R) reader structure"]
impl crate::Readable for WdtCrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_cr::W`](W) writer structure"]
impl crate::Writable for WdtCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDT_CR to value 0x0a"]
impl crate::Resettable for WdtCrSpec {
    const RESET_VALUE: u32 = 0x0a;
}
